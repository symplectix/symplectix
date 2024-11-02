use std::env;
use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command as StdCommand, ExitStatus, Stdio};
use std::ptr;
use std::time::Duration;

use clap::Parser;
use tokio::process;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time;
use tracing::{error, trace};

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
    child_id: u32,
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

unsafe fn kill(pid: libc::pid_t, signal: libc::c_int) -> io::Result<()> {
    if libc::kill(pid, signal) == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

unsafe fn killpg(pid: libc::pid_t, signal: libc::c_int) -> io::Result<()> {
    if libc::killpg(pid, signal) == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
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
        // A process group ID of 0 will use the process ID as the PGID.
        cmd.process_group(0);

        let child = process::Command::from(cmd).spawn()?;
        let child_id = child.id().expect("fetching child pid before polling should not fail");
        let timeout = self.timeout;
        Ok(Process { child, child_id, timeout })
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

    /// Stops and waits the process, including whole descendant processes.
    #[tracing::instrument(skip(self))]
    pub async fn stop(&mut self, gracefully: bool) -> io::Result<ExitStatus> {
        // Notify the spawned process to be terminated.
        unsafe {
            if gracefully {
                _ = kill(self.child_id as libc::c_int, libc::SIGTERM);
                time::sleep(Duration::from_millis(50)).await;
            }
            _ = kill(self.child_id as libc::c_int, libc::SIGKILL);
        }

        let status = loop {
            match self.child.try_wait()? {
                // The exit status is not available at this time. The child may still be running.
                // SIGKILL is sent just before entering the loop, but this happens because
                // kill(2) just sends the signal to the given process(es).
                // Once kill(2) returns, there is no guarantee that the signal has been delivered and handled.
                None => continue,
                Some(status) => break Ok(status),
            }
        };

        // If processes except the spawned one exists, reap them all here.
        unsafe {
            self.reap();
        }

        status
    }

    /// Waits all descendant processes to ensure there are no children left behind.
    unsafe fn reap(&self) {
        _ = killpg(self.child_id as libc::c_int, libc::SIGKILL);

        loop {
            // The WNOHANG option is used to indicate that the call should not block
            // if there are no processes that wish to report status.
            let pid = libc::waitpid(-1, ptr::null_mut(), libc::WNOHANG);
            // If this Error was constructed via last_os_error or from_raw_os_error,
            // then this function will return Some, otherwise it will return None.
            let err = io::Error::last_os_error().raw_os_error().unwrap();

            match (pid, err) {
                // If there are no children not previously awaited, with errno set to ECHILD.
                (-1, libc::ECHILD) => {
                    // No more children, we are done.
                    trace!(pid, err, "no children to be reaped");
                    return;
                }

                (_, libc::EINTR) => {
                    // This likely can't happen since we are calling libc::waitpid with WNOHANG.
                    trace!(pid, err, "got interrupted, try again");
                    continue;
                }

                // The pid is 0 when WNOHANG is specified and there are no stopped or exited children.
                // Otherwise, the process ID of the child represents a stopped or terminated child process.
                (pid, err) => {
                    trace!(pid, err, "continue reaping");
                    continue;
                }
            }
        }
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
