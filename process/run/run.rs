use std::ffi::OsString;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{ExitCode, ExitStatus, Stdio};
use std::sync::Arc;
use std::time::Duration;
use std::{env, io, process};

use futures::future;
use futures::prelude::*;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::signal::unix::{SignalKind, signal};
use tokio::time;
use tracing::{error, info, trace};

mod child;

#[cfg(test)]
mod run_test;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser)]
pub struct Command {
    #[arg(long)]
    dry_run: bool,

    #[command(flatten)]
    hook: Hook,

    #[command(flatten)]
    timeout: Timeout,

    /// The entrypoint of the child process.
    #[arg()]
    program: OsString, // ENTRYPOINT

    /// Environment variables visible to the spawned process.
    #[arg(long = "env", value_name = "KEY")]
    envs: Vec<String>,

    /// The arguments passed to the command.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<OsString>, // CMD
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::Parser)]
struct Timeout {
    /// Kill the spawned process if it still running after the specified duration.
    #[arg(
        long,
        value_name = "DURATION",
        value_parser = humantime::parse_duration,
    )]
    kill_after: Option<Duration>,

    /// Exit with a zero status on timeout.
    // For example, timeout is not a failure for '//fuzzing:fuzz_test'.
    #[arg(long = "timeout-is-ok")]
    is_ok: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser)]
struct Hook {
    /// Check existence of given files before spawning the child process.
    ///
    /// Note that the timeout duration does not elapse until the child is spawned.
    /// So the operations before spawning, i.e., waiting for files, never times out.
    #[arg(long = "wait", value_name = "PATH")]
    wait_for: Vec<PathBuf>,

    /// Create an empty file after the child process exits.
    #[arg(long, value_name = "PATH")]
    on_exit: Option<PathBuf>,
}

pub struct Process {
    inner: ProcessInner,
}

enum ProcessInner {
    DryRun { cmd: Arc<Command> },
    Spawned { reaper: reaper::Channel, child: child::Child },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{exit_status}")]
pub struct WaitStatus {
    exit_status: ExitStatus,
    exit_reason: ExitReasons,
    cmd: Arc<Command>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct ExitReasons {
    io_error: Option<io::ErrorKind>,
    timedout: bool,
    proc_signaled: Option<libc::c_int>,
    self_signaled: Option<libc::c_int>,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct WaitStatusError(WaitStatus);

#[derive(Debug)]
enum SpawnError {
    Io(io::Error),
    FoundErrFile(PathBuf),
}

impl Command {
    pub fn from_args_os<I, T>(args_os: I) -> Command
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        <Self as clap::Parser>::parse_from(args_os)
    }

    #[tracing::instrument(skip(self))]
    fn dry_run(self: &Arc<Self>) -> io::Result<WaitStatus> {
        info!("[DRYRUN] {:?}", self);

        Ok(WaitStatus { exit_status: ExitStatus::from_raw(0), exit_reason: ExitReasons::default(), cmd: self.clone() })
    }

    #[tracing::instrument(skip(self))]
    pub async fn spawn(self: &Arc<Self>) -> io::Result<Process> {
        if self.dry_run {
            Ok(Process { inner: ProcessInner::DryRun { cmd: self.clone() } })
        } else {
            let mut cmd = process::Command::new(&self.program);

            cmd.args(&self.args[..]);

            cmd.stderr(Stdio::piped());

            cmd.env_clear().envs(env::vars().filter(|(key, _)| self.envs.contains(key)));

            // Put the child into a new process group.
            // A process group ID of 0 will use the process ID as the PGID.
            cmd.process_group(0);

            // TODO: nightly
            // #[cfg(target_os = "linux")]
            // {
            //     use std::os::linux::process::CommandExt;
            //     cmd.create_pidfd(true);
            // }

            wait_for(&self.hook.wait_for).await.map_err(|err| match err {
                SpawnError::Io(io_err) => io_err,
                SpawnError::FoundErrFile(path) => {
                    io::Error::new(io::ErrorKind::InvalidData, format!("found an error file at {}", path.display()))
                }
            })?;

            #[cfg(target_os = "linux")]
            unsafe {
                libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
            }

            let reaper = reaper::subscribe();
            let child = child::spawn(cmd, Arc::clone(self))?;
            Ok(Process { inner: ProcessInner::Spawned { reaper, child } })
        }
    }
}

#[tracing::instrument]
async fn wait_for(paths: &[PathBuf]) -> Result<(), SpawnError> {
    let wait_files = paths.iter().map(|ok_file| async move {
        let err_file = ok_file.with_extension("err");

        loop {
            tracing::trace!(wait_for = %ok_file.display());

            if err_file.try_exists().map_err(SpawnError::Io)? {
                return Err(SpawnError::FoundErrFile(err_file));
            }

            if ok_file.try_exists().map_err(SpawnError::Io)? {
                return Ok(());
            }

            time::sleep(Duration::from_millis(1000)).await;
        }
    });

    future::try_join_all(wait_files).map_ok(|_| ()).await
}

impl Process {
    #[tracing::instrument(
        skip(self),
        fields(
            pid = self.pid(),
        )
    )]
    pub async fn wait(self) -> io::Result<WaitStatus> {
        self.inner.wait().await
    }

    fn pid(&self) -> Option<u32> {
        match &self.inner {
            ProcessInner::DryRun { .. } => None,
            ProcessInner::Spawned { child, .. } => Some(child.pid),
        }
    }
}

impl ProcessInner {
    async fn wait(self) -> io::Result<WaitStatus> {
        match self {
            ProcessInner::DryRun { cmd } => cmd.dry_run(),
            ProcessInner::Spawned { mut reaper, mut child } => {
                // SIGTERM: stop monitored process
                // SIGINT:  e.g., Ctrl-C at terminal
                // SIGQUIT: e.g., Ctrl-\ at terminal
                // SIGHUP:  e.g., terminal closed
                let mut sigterm = signal(SignalKind::terminate())?;
                let mut sigint = signal(SignalKind::interrupt())?;

                let stderr = child.stderr().take().unwrap();
                let mut stderr = BufReader::new(stderr).lines();

                let mut cause = ExitReasons::default();
                let mut _interrupted = 0;

                let to_wait_status = |exit_status: ExitStatus, mut cause: ExitReasons, cmd| -> WaitStatus {
                    cause.proc_signaled = exit_status.signal().or(cause.proc_signaled);
                    WaitStatus { exit_status, exit_reason: cause, cmd: Arc::clone(cmd) }
                };

                let result = loop {
                    tokio::select! {
                        biased;
                        reaped = reaper.recv() => match reaped {
                            Err(err) => {
                                trace!("closed({}), lagged({})", err.closed(), err.lagged().unwrap_or(0));
                            }
                            Ok((pid, exit_status)) => if pid == child.pid as libc::pid_t {
                                break Ok(to_wait_status(exit_status, cause, &child.cmd));
                            }
                        },
                        _ = sigterm.recv() => {
                            _interrupted += 1;
                            cause.self_signaled = cause.self_signaled.or(Some(libc::SIGTERM));
                            child.kill(Some(libc::SIGTERM)).await;
                        },
                        _ = sigint.recv() => {
                            _interrupted += 1;
                            cause.self_signaled = cause.self_signaled.or(Some(libc::SIGINT));
                            child.kill(Some(libc::SIGINT)).await;
                        },
                        child_stat = child.wait() => match child_stat {
                            Err(err) => {
                                error!("got an error while waiting the child: {}", err.to_string());
                                cause.io_error = cause.io_error.or(Some(err.kind()));
                                child.kill(None).await;
                            }
                            Ok(None) => {
                                _interrupted += 1;
                                cause.timedout = true;
                                child.kill(None).await;
                            }
                            Ok(Some(exit_status)) => {
                                break Ok(to_wait_status(exit_status, cause, &child.cmd));
                            }
                        },
                        line = stderr.next_line() => match line {
                            Err(err) => {
                                error!("got an error while reading lines: {}", err.to_string());
                                cause.io_error = cause.io_error.or(Some(err.kind()));
                                child.kill(None).await;
                            }
                            Ok(None) => {
                                trace!("got an empty result from next_line");
                            }
                            Ok(Some(line)) => {
                                tracing::info!("{}", line);
                            }
                        },
                    }
                };

                // Reap all descendant processes here,
                // to ensure there are no children left behind.
                child.killpg();
                on_exit(child.cmd.hook.on_exit.as_ref(), result).await
            }
        }
    }
}

impl WaitStatus {
    pub fn exit_ok(&self) -> Result<(), WaitStatusError> {
        let exit_success = self.exit_status.success();
        let timedout_but_ok = self.exit_reason.timedout && self.cmd.timeout.is_ok;

        if exit_success || timedout_but_ok { Ok(()) } else { Err(WaitStatusError(self.clone())) }
    }
}

impl WaitStatusError {
    pub fn exit_code(&self) -> ExitCode {
        let ws = &self.0;

        if ws.exit_reason.timedout {
            return ExitCode::from(124);
        }

        if let Some(s) = ws.exit_reason.self_signaled {
            return ExitCode::from(128 + s as u8);
        }
        if let Some(s) = ws.exit_reason.proc_signaled {
            return ExitCode::from(128 + s as u8);
        }

        if ws.exit_reason.io_error.is_some() {
            return ExitCode::from(125);
        }

        ws.exit_status.code().map(|c| ExitCode::from(c as u8)).unwrap_or(ExitCode::FAILURE)
    }
}

#[tracing::instrument]
async fn on_exit(path: Option<&PathBuf>, result: io::Result<WaitStatus>) -> io::Result<WaitStatus> {
    if let Some(path) = path {
        if matches!(result, Ok(ref status) if status.exit_ok().is_ok()) {
            fsutil::create_file(path, true).await?;
        } else {
            fsutil::create_file(path.with_extension("err"), true).await?;
        }
    }

    result
}
