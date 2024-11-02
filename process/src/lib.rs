use std::env;
use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command as StdCommand, Stdio};
use std::time::Duration;

use clap::Parser;
use tokio::process;
use tokio::time;

pub use std::process::ExitStatus;

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
    argv: Vec<OsString>,
}

#[derive(Debug)]
pub struct Process {
    child: process::Child,
    id: u32,
    timeout: Option<Duration>,
}

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        Command {
            stdout: None,
            stderr: None,
            envs: vec![],
            timeout: None,
            argv: vec![program.as_ref().to_owned()],
        }
    }

    pub fn arg<S>(&mut self, arg: S) -> &mut Command
    where
        S: AsRef<OsStr>,
    {
        self.argv.push(arg.as_ref().to_owned());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    pub fn timeout(&mut self, duration: Duration) -> &mut Command {
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
        let id = child.id().expect("fetching the OS-assigned process id");
        Ok(Process { child, id, timeout: self.timeout })
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

    /// Stop the whole process group, and wait the child exits.
    #[tracing::instrument(skip(self))]
    pub async fn stop(&mut self, gracefully: bool) -> io::Result<ExitStatus> {
        if gracefully {
            self.killpg(libc::SIGTERM);
            time::sleep(Duration::from_millis(50)).await;
        }

        // TODO: Wait all descendant processes to ensure there are no children left behind.
        // Currently, the direct child is the only process to be waited before exiting.

        self.killpg(libc::SIGKILL);

        // Note that this loop is necessary even if the child process exits successfully
        // in order to 1) wait all descendant processes, 2) ensure there are no children left behind.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn spawn_commands() {
        assert!(Command::new("date").spawn().await.is_ok());
        assert!(Command::new("unknown_command").spawn().await.is_err());
    }
}
