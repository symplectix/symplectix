// TODO: Wait all descendant processes to ensure there are no children left behind.
// Currently, the spawned child is the only process to be waited before exiting.
//
// It is viable to wait all descendant processes by calling
// waitpid(-1, NULL, WNOHANG) every time SIGCHLD arrives.
// The concern is that tokio::process::Child relies on SIGCHLD to get woken up.

use std::env;
use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command as StdCommand, ExitStatus, Stdio};
use std::time::Duration;

use clap::Parser;
use tokio::process;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time;

#[derive(Debug, Clone, Parser)]
pub struct Entrypoint {
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
    #[arg(trailing_var_arg = true)]
    argv: Vec<OsString>,
}

#[derive(Debug)]
pub struct Process {
    child: process::Child,
    timeout: Option<Duration>,
}

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(io::Error),

    #[error("the spawned child exited unsuccessfully: {0}")]
    ExitedUnsuccessfully(ExitStatus),

    #[error("the spawned child timedout: {0}")]
    Timedout(ExitStatus),
}

#[tracing::instrument(skip(entrypoint))]
pub async fn run(entrypoint: &Entrypoint) -> Result {
    let process = entrypoint.spawn().await.map_err(Error::Io)?;
    wait(process).await
}

#[tracing::instrument(skip(process))]
pub async fn wait(mut process: Process) -> Result {
    let mut interrupt = signal(SignalKind::interrupt()).map_err(Error::Io)?;
    let mut terminate = signal(SignalKind::terminate()).map_err(Error::Io)?;

    let timedout = tokio::select! {
        biased;
        _ = interrupt.recv() => { false },
        _ = terminate.recv() => { false },
        r = process.wait() => matches!(r, Ok(None)),
    };

    let exit_status = process.stop(true).await.map_err(Error::Io)?;

    if timedout {
        Err(Error::Timedout(exit_status))
    } else if !exit_status.success() {
        Err(Error::ExitedUnsuccessfully(exit_status))
    } else {
        Ok(())
    }
}

impl Entrypoint {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Entrypoint {
        Entrypoint {
            stdout: None,
            stderr: None,
            envs: vec![],
            timeout: None,
            argv: vec![program.as_ref().to_owned()],
        }
    }

    pub fn arg<S>(&mut self, arg: S) -> &mut Entrypoint
    where
        S: AsRef<OsStr>,
    {
        self.argv.push(arg.as_ref().to_owned());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Entrypoint
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    pub fn timeout(&mut self, duration: Duration) -> &mut Entrypoint {
        self.timeout = Some(duration);
        self
    }

    pub async fn spawn(&self) -> io::Result<Process> {
        // #[cfg(target_os = "linux")]
        // unsafe {
        //     libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
        // }

        let mut cmd = StdCommand::new(&self.argv[0]);

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
        let timeout = self.timeout;
        Ok(Process { child, timeout })
    }
}

impl Process {
    /// Waits until the process exits or times out.
    /// For the case of timeout, Ok(None) will be returned.
    pub async fn wait(&mut self) -> io::Result<Option<ExitStatus>> {
        match self.timeout {
            // Always some because no timeout given.
            None => self.child.wait().await.map(Some),
            Some(dur) => match time::timeout(dur, self.child.wait()).await {
                Err(_elapsed) => Ok(None),
                Ok(status) => status.map(Some),
            },
        }
    }

    /// Stops the whole process group, and waits the child exits.
    #[tracing::instrument(skip(self))]
    pub async fn stop(&mut self, gracefully: bool) -> io::Result<ExitStatus> {
        if gracefully {
            self.killpg(libc::SIGTERM);
            time::sleep(Duration::from_millis(50)).await;
        }

        self.killpg(libc::SIGKILL);

        loop {
            match self.child.try_wait()? {
                // The exit status is not available at this time. The child may still be running.
                // SIGKILL is sent just before entering the loop, but this happens because
                // kill(2) just sends the signal to the given process(es).
                // Once kill(2) returns, there is no guarantee that the signal has been delivered and handled.
                None => continue,
                Some(status) => break Ok(status),
            }
        }
    }

    /// Sends signal to a process group.
    fn killpg(&self, signal: libc::c_int) {
        // The child already has been polled to completion.
        let Some(id) = self.child.id() else { return };
        let id = id as libc::c_int;

        // The killpg() function returns 0 if successful;
        // otherwise -1 is returned and the global variable errno is set to indicate the error.
        let killed = unsafe { libc::killpg(id, signal) };
        let last_os_error = (killed == -1).then_some(format!("{}", io::Error::last_os_error()));
        tracing::trace!(pgid = id, signal, killed, last_os_error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn spawn_commands() {
        assert!(Entrypoint::new("date").spawn().await.is_ok());
        assert!(Entrypoint::new("unknown_command").spawn().await.is_err());
    }

    #[tokio::test]
    async fn run_process() {
        use Error::*;

        let sleep = wait(Entrypoint::new("sleep").arg("0.1").spawn().await.unwrap());
        let Ok(_) = sleep.await else {
            panic!("expected that the command 'sleep' exit successfully");
        };

        let sleep = wait(
            Entrypoint::new("sleep")
                .arg("10")
                .timeout(Duration::from_millis(10))
                .spawn()
                .await
                .unwrap(),
        );
        let Err(Timedout(_exit_status)) = sleep.await else {
            panic!("expected that the command 'sleep' times out");
        };
    }
}
