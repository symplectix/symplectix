use std::env;
use std::io;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Command as StdCommand, ExitStatus, Stdio};
use std::time::Duration;

use clap::Parser;
use futures::prelude::*;
use tokio::process;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time;

pub type Result<T = ()> = std::result::Result<T, Error>;

use Error::{ExitedUnsuccessfully, KilledBySignal};

#[derive(Debug, Clone, Parser)]
pub struct Command {
    /// Redirect the child process stdout.
    #[arg(long, value_name = "PATH")]
    stdout: Option<PathBuf>,

    /// Redirect the child process stderr.
    #[arg(long, value_name = "PATH")]
    stderr: Option<PathBuf>,

    /// Kill the spawned child process after the specified duration.
    #[arg(long, value_name = "DURATION", value_parser = humantime::parse_duration)]
    timeout: Option<Duration>,

    /// Environment variables visible to the spawned process.
    #[arg(long = "env", value_name = "KEY")]
    envs: Vec<String>,

    /// The entrypoint of the child process.
    #[arg(last = true)]
    argv: Vec<String>,
}

#[derive(Debug)]
pub struct Process {
    child: process::Child,
    id: u32,
    timeout: Option<Duration>,
    result: Option<Result>,
}

/// Process errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(io::Error),

    #[error("the spawned child process was killed by a signal: {0}")]
    KilledBySignal(i32),

    #[error("the spawned child exited unsuccessfully with non-zero code: {0}")]
    ExitedUnsuccessfully(i32),
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::KilledBySignal(signal) => KilledBySignal(*signal),
            Error::ExitedUnsuccessfully(code) => ExitedUnsuccessfully(*code),
            Error::Io(io_err) => Error::Io(io::Error::new(io_err.kind(), io_err.to_string())),
        }
    }
}

impl Command {
    #[tracing::instrument(
        skip(self),
        fields(
            argv = ?self.argv,
        )
    )]
    pub async fn run(self) -> Result {
        let mut interrupt = signal(SignalKind::interrupt()).map_err(Error::Io)?;
        let mut terminate = signal(SignalKind::terminate()).map_err(Error::Io)?;
        let mut process = self.spawn().map_err(Error::Io).await?;

        tokio::select! {
            biased;
            _ = interrupt.recv() => {},
            _ = terminate.recv() => {},
            _ = process.wait() => {},
        };
        process.stop(true).await
    }

    pub async fn spawn(&self) -> io::Result<Process> {
        // #[cfg(target_os = "linux")]
        // unsafe {
        //     libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
        // }

        let mut cmd = StdCommand::new(self.argv[0].as_str());

        cmd.args(&self.argv[1..]);

        cmd.stdout(if let Some(path) = self.stdout.as_ref() {
            fsutil::stdio_from(path, false).await?
        } else {
            Stdio::inherit()
        });

        cmd.stderr(if let Some(path) = self.stderr.as_ref() {
            fsutil::stdio_from(path, false).await?
        } else {
            Stdio::inherit()
        });

        cmd.env_clear().envs(env::vars().filter(|(key, _)| self.envs.contains(key)));

        // Put the child into a new process group.
        cmd.process_group(0);

        let child = process::Command::from(cmd).spawn()?;
        let id = child.id().expect("fetching the OS-assigned process id");
        Ok(Process { child, id, timeout: self.timeout, result: None })
    }
}

mod process_impl {
    use super::*;

    impl Drop for Process {
        #[tracing::instrument(skip(self))]
        fn drop(&mut self) {
            self.kill();
        }
    }

    impl Process {
        /// If the process exits before the timeout has elapsed, then the completed
        /// result is returned. Otherwise, a `None` is returned.
        pub async fn wait(&mut self) -> Option<Result> {
            match self.timeout {
                None => Some(self.wait_child().await),
                // It is possible for the child process to complete and exceed the timeout
                // without returning an error.
                Some(dur) => time::timeout(dur, self.wait_child()).await.ok(),
            }
        }

        async fn wait_child(&mut self) -> Result {
            into_process_result(self.child.wait().map_err(Error::Io).await?)
        }

        /// Same as [crate::Process::kill], but in a slightly more graceful way if gracefully is true.
        #[tracing::instrument(skip(self))]
        pub async fn stop(&mut self, gracefully: bool) -> Result {
            if gracefully {
                self.killpg(libc::SIGTERM);
                time::sleep(Duration::from_millis(50)).await;
            }
            self.kill();
            self.result.clone().unwrap()
        }

        /// Kill the whole process group, and wait the child exits.
        /// This should mostly work, but not perfect because:
        /// - it is easy to "escape" from the group
        /// - the PID is potentially reused at some point
        ///
        /// TODO: Wait all descendant processes to ensure there are no children left behind.
        /// Currently, the direct child is the only process to be waited before exiting.
        pub fn kill(&mut self) {
            if self.result.is_some() {
                return;
            }

            self.killpg(libc::SIGKILL);

            // Note that this loop is necessary even if the child process exits successfully
            // in order to 1) wait all descendant processes, 2) ensure there are no children left behind.
            self.result = loop {
                match self.child.try_wait() {
                    // The exit status is not available at this time. The child may still be running.
                    // SIGKILL is sent just before entering the loop, but this happens because
                    // kill(2) just sends the signal to the given process(es).
                    // Once kill(2) returns, there is no guarantee that the signal has been delivered and handled.
                    Ok(None) => continue,
                    Ok(Some(status)) => break Some(into_process_result(status)),
                    Err(err) => break Some(Err(Error::Io(err))),
                }
            };
        }

        fn killpg(&self, signal: libc::c_int) {
            let id = self.id as libc::c_int;
            unsafe {
                let killed = libc::killpg(id, signal);
                tracing::trace!(
                    signal,
                    killed,
                    errno = io::Error::last_os_error().raw_os_error().unwrap_or(0)
                );
            }
        }
    }

    fn into_process_result(status: ExitStatus) -> Result {
        if status.success() {
            Ok(())
        } else if let Some(code) = status.code() {
            Err(ExitedUnsuccessfully(code))
        } else {
            // because `status.code()` returns `None`
            Err(KilledBySignal(status.signal().expect("WIFSIGNALED is true")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command<S: Into<String>>(program: Vec<S>) -> Command {
        Command {
            stdout: None,
            stderr: None,
            envs: vec![],
            timeout: None,
            argv: program.into_iter().map(|s| s.into()).collect(),
        }
    }

    fn sleep<S: Into<String>>(duration: S) -> Command {
        command(vec!["sleep".to_owned(), duration.into()])
    }

    #[tokio::test]
    async fn run_process() {
        assert!(command(vec!["date"]).run().await.is_ok());
        assert!(command(vec!["unknown_command"]).run().await.is_err());
        assert!(sleep("0.1").run().await.is_ok());
    }
}
