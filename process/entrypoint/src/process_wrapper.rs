use std::env;
use std::io;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Command as StdCommand, ExitStatus, Stdio};
use std::time::Duration;

use clap::Parser;
use futures::future;
use futures::prelude::*;
use tokio::process::{Child, Command};
use tokio::signal::unix::{signal, SignalKind};
use tokio::time;

use crate::fsutil;
use crate::Error::*;
use crate::{Error, Result};

#[derive(Debug, Clone, Parser)]
pub struct ProcessWrapper {
    /// Redirect the child process stdout.
    #[arg(long, value_name = "PATH")]
    stdout: Option<PathBuf>,

    /// Redirect the child process stderr.
    #[arg(long, value_name = "PATH")]
    stderr: Option<PathBuf>,

    /// Environment variables visible to the spawned process.
    #[arg(long = "env", value_name = "KEY")]
    envs: Vec<String>,

    /// Kill the spawned child process after the specified duration.
    #[arg(long, value_name = "DURATION")]
    timeout: Option<humantime::Duration>,

    /// The entrypoint of the child process.
    #[arg(last = true)]
    command: Vec<String>,
}

impl ProcessWrapper {
    #[tracing::instrument(
        skip(self),
        fields(
            command = %self.command[0],
        )
    )]
    pub async fn run(&self) -> Result {
        // #[cfg(target_os = "linux")]
        // unsafe {
        //     libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
        // }

        let mut interrupt = signal(SignalKind::interrupt()).map_err(Error::Io)?;
        let mut terminate = signal(SignalKind::terminate()).map_err(Error::Io)?;

        let mut child = spawn(self).await?;
        let id = child.id().expect("fetching the OS-assigned process id") as libc::c_int;

        tokio::select! {
            biased;
            _ = future::select(
                interrupt.recv().boxed_local(),
                terminate.recv().boxed_local(),
            ) => {
                tracing::trace!(id, branch = "signaled");
            }
            _ = timer(self) => {
                tracing::trace!(id, branch = "timedout");
            }
            wait_result = child.wait() => match wait_result {
                Ok(exit_status) => tracing::trace!(id, branch = "wait_exited", ?exit_status),
                Err(io_err) => tracing::warn!(id, branch = "wait_failed", %io_err),
            }
        }

        killpg(id).await;

        // TODO: Wait all descendant processes, if any.
        // Currently, the direct child is the only process to be waited before exiting.
        match child.try_wait() {
            // It is possible for the child process to complete and exceed the timeout
            // without returning an error.
            Ok(Some(status)) => into_process_result(status),

            // The exit status is not available at this time.
            // The child process(es) may still be running.
            Ok(None) => match child.wait().await {
                Ok(status) => into_process_result(status),
                Err(err) => Err(WaitFailed(err)),
            },

            // Some error happens on collecting the child status.
            Err(err) => Err(WaitFailed(err)),
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

fn timer(opts: &ProcessWrapper) -> future::Either<future::Pending<()>, time::Sleep> {
    match opts.timeout.as_ref() {
        None => future::pending().left_future(),
        Some(&dur) => time::sleep(dur.into()).right_future(),
    }
}

async fn spawn(opts: &ProcessWrapper) -> Result<Child> {
    let mut cmd = StdCommand::new(opts.command[0].as_str());

    cmd.args(&opts.command[1..]);

    cmd.stdout(if let Some(path) = opts.stdout.as_ref() {
        fsutil::stdio_from(path, false).map_err(Error::Io).await?
    } else {
        Stdio::inherit()
    });

    cmd.stderr(if let Some(path) = opts.stderr.as_ref() {
        fsutil::stdio_from(path, false).map_err(Error::Io).await?
    } else {
        Stdio::inherit()
    });

    cmd.env_clear().envs(env::vars().filter(|(key, _)| opts.envs.contains(key)));

    // Put the child into a new process group.
    cmd.process_group(0);

    Command::from(cmd).spawn().map_err(NotSpawned)
}

/// Kill the whole process group, in a graceful manner, to ensure there are no children left behind.
/// This mostly works, but not perfect because:
/// * it is easy to "escape" from the group.
/// * the PID is potentially reused at some point.
#[tracing::instrument]
async fn killpg(id: i32) {
    let killpg = |signal| unsafe {
        if libc::killpg(id, 0) == 0 {
            let killed = libc::killpg(id, signal);
            tracing::trace!(
                signal,
                killed,
                errno = io::Error::last_os_error().raw_os_error().unwrap_or(0)
            );
        }
    };

    killpg(libc::SIGTERM);
    // The time window between the `wait` returning and `SIGKILL` should be small.
    // Don't sleep too much.
    let delay = Duration::from_millis(100);
    time::sleep(delay).await;
    killpg(libc::SIGKILL);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_wrapper<S: Into<String>>(command: Vec<S>) -> ProcessWrapper {
        ProcessWrapper {
            stdout: None,
            stderr: None,
            envs: vec![],
            timeout: None,
            command: command.into_iter().map(|s| s.into()).collect(),
        }
    }

    fn sleep<S: Into<String>>(duration: S) -> ProcessWrapper {
        process_wrapper(vec!["sleep".to_owned(), duration.into()])
    }

    #[tokio::test]
    async fn run_process() {
        assert!(process_wrapper(vec!["date"]).run().await.is_ok());
        assert!(process_wrapper(vec!["unknown_command"]).run().await.is_err());
        assert!(sleep("0.1").run().await.is_ok());
    }
}
