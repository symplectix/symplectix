#![allow(missing_docs)]
use std::ffi::OsString;
use std::os::unix::process::{
    CommandExt,
    ExitStatusExt,
};
use std::path::PathBuf;
use std::process::{
    ExitCode,
    Stdio,
    Termination,
};
use std::sync::Arc;
use std::time::Duration;
use std::{
    env,
    io,
    process,
};

use anyhow::Context;
use arc_swap::ArcSwap;
use futures::future;
use futures::prelude::*;
use tokio::signal::unix::{
    SignalKind,
    signal,
};
use tokio::time;
use tracing::{
    error,
    trace,
};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

#[cfg(test)]
mod tests;

fn main() -> ProcExit {
    ProcExit(try_run(env::args_os()))
}

#[derive(Debug)]
struct ProcExit(anyhow::Result<()>);

impl Termination for ProcExit {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(_) => ExitCode::SUCCESS,
            Err(ref cause) => match cause.downcast_ref::<ExitStatusError>() {
                Some(err) => err.exit_code(),
                None => ExitCode::FAILURE,
            },
        }
    }
}

/// Spawns a new process and waits the status.
#[tokio::main]
async fn try_run<T>(args: impl IntoIterator<Item = T>) -> anyhow::Result<()>
where
    T: Into<OsString> + Clone,
{
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .with_target(false)
                .without_time(),
        )
        .with(EnvFilter::from_env("PROCRUN_LOG"))
        .init();

    #[cfg(target_os = "linux")]
    unsafe {
        assert_eq!(0, libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0));
    }

    let proc = Flags::from_args_os(args)
        .command()
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .await
        .context("Failed to spawn process")?;

    proc.wait()
        .await
        .context("Failed to fetch wait status")?
        .exit_ok()
        .context("Got a failure on running the process")
}

mod c {
    use std::io;

    pub(crate) fn kill(pid: libc::pid_t, sig: libc::c_int) -> io::Result<()> {
        unsafe { if libc::kill(pid, sig) == 0 { Ok(()) } else { Err(io::Error::last_os_error()) } }
    }

    pub(crate) fn killpg(grp: libc::pid_t, sig: libc::c_int) -> io::Result<()> {
        unsafe {
            if libc::killpg(grp, sig) == 0 { Ok(()) } else { Err(io::Error::last_os_error()) }
        }
    }
}

mod fsutil {
    use std::io;
    use std::path::Path;

    use tokio::fs;

    async fn create_dirs_if_missing<P: AsRef<Path>>(path: P) -> io::Result<()> {
        let path = path.as_ref();

        if let Some(dir) = path.parent()
            && !dir.as_os_str().is_empty()
        {
            return fs::create_dir_all(dir).await;
        };

        Ok(())
    }

    pub async fn create_file<P: AsRef<Path>>(path: P, truncate: bool) -> io::Result<std::fs::File> {
        let file = {
            let path = path.as_ref();
            create_dirs_if_missing(path).await?;
            fs::OpenOptions::new().create(true).write(true).truncate(truncate).open(path).await?
        };

        Ok(file.into_std().await)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser)]
struct Flags {
    #[command(flatten)]
    hook: Hook,

    #[command(flatten)]
    timeout: Timeout,

    /// Environment variables visible to the spawned process.
    #[arg(long = "env", value_name = "KEY")]
    envs: Vec<String>,

    /// The entrypoint of the child process.
    #[arg()]
    program: OsString,

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

struct Command {
    cmd:   process::Command,
    flags: Arc<ArcSwap<Flags>>,
}

struct Process {
    reaper:    reaper::Channel,
    child:     tokio::process::Child,
    child_pid: u32,
    flags:     Arc<ArcSwap<Flags>>,
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{exit_status}")]
struct ExitStatus {
    exit_status:  process::ExitStatus,
    exit_reasons: ExitReasons,
    flags:        Arc<ArcSwap<Flags>>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct ExitReasons {
    timedout:       bool,
    child_signaled: Option<libc::c_int>,
    iam_signaled:   Option<libc::c_int>,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct ExitStatusError(ExitStatus);

#[derive(Debug)]
enum SpawnError {
    Io(io::Error),
    FoundErrFile(PathBuf),
}

impl Flags {
    fn from_args_os<I, T>(args_os: I) -> Flags
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        <Self as clap::Parser>::parse_from(args_os)
    }

    fn command(self) -> Command {
        let cmd = process::Command::new(&self.program);
        let flags = Arc::new(ArcSwap::from_pointee(self));
        Command { cmd, flags }
    }
}

impl Command {
    fn stdout<T: Into<Stdio>>(mut self, stdio: T) -> Self {
        self.cmd.stdout(stdio);
        self
    }

    fn stderr<T: Into<Stdio>>(mut self, stdio: T) -> Self {
        self.cmd.stderr(stdio);
        self
    }

    #[tracing::instrument(skip(self))]
    async fn spawn(mut self) -> io::Result<Process> {
        let flags = self.flags.load();
        wait_for(&flags.hook.wait_for).await.map_err(|err| match err {
            SpawnError::Io(io_err) => io_err,
            SpawnError::FoundErrFile(path) => io::Error::new(
                io::ErrorKind::InvalidData,
                format!("found an error file at {}", path.display()),
            ),
        })?;

        self.cmd
            .args(&flags.args[..])
            // Put the child into a new process group.
            // A process group ID of 0 will use the process ID as the PGID.
            .process_group(0);

        // TODO: nightly
        // #[cfg(target_os = "linux")]
        // {
        //     use std::os::linux::process::CommandExt;
        //     self.cmd.create_pidfd(true);
        // }

        let reaper = reaper::subscribe();
        let child = tokio::process::Command::from(self.cmd).kill_on_drop(false).spawn()?;
        let child_pid = child.id().expect("fetching the process id before polling should not fail");
        Ok(Process { reaper, child, child_pid, flags: self.flags })
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
    fn pid(&self) -> u32 {
        self.child_pid
    }

    #[tracing::instrument(
        skip(self),
        fields(
            pid = self.pid(),
        )
    )]
    async fn wait(self) -> io::Result<ExitStatus> {
        let Process { mut reaper, mut child, child_pid, flags } = self;

        // SIGTERM: stop monitored process
        // SIGINT:  e.g., Ctrl-C at terminal
        // SIGQUIT: e.g., Ctrl-\ at terminal
        // SIGHUP:  e.g., terminal closed
        let mut sigterm = signal(SignalKind::terminate())?;
        let mut sigint = signal(SignalKind::interrupt())?;

        let mut reasons = ExitReasons::default();
        let mut _interrupted = 0;

        let result = loop {
            tokio::select! {
                biased;
                reaped = reaper.recv() => match reaped {
                    Err(err) => {
                        trace!("closed({}), lagged({})", err.closed(), err.lagged().unwrap_or(0));
                    }
                    Ok((pid, exit_status)) => if pid == child_pid as libc::pid_t {
                        break Ok(to_exit_status(exit_status, reasons, &flags));
                    }
                },
                _ = sigterm.recv() => {
                    _interrupted += 1;
                    reasons.iam_signaled = reasons.iam_signaled.or(Some(libc::SIGTERM));
                    kill(child_pid, Some(libc::SIGTERM)).await;
                },
                _ = sigint.recv() => {
                    _interrupted += 1;
                    reasons.iam_signaled = reasons.iam_signaled.or(Some(libc::SIGINT));
                    kill(child_pid, Some(libc::SIGINT)).await;
                },
                child_stat = wait_child(&mut child, flags.load()) => match child_stat {
                    Err(err) => {
                        // IO errors occur sometimes.
                        // There may be some conflict between the reaper implementation and wait_child.
                        //
                        // - Reaper uses SIGCHLD
                        // - Tokio uses pidfd on Linux: https://github.com/tokio-rs/tokio/pull/6152
                        //
                        // Interestingly, ignoring this error seems to get the flaky tests to work properly.
                        error!("got an error while waiting the child: {}", err.to_string());
                        kill(child_pid, None).await;
                    }
                    Ok(None) => {
                        _interrupted += 1;
                        reasons.timedout = true;
                        kill(child_pid, None).await;
                    }
                    Ok(Some(exit_status)) => {
                        break Ok(to_exit_status(exit_status, reasons, &flags));
                    }
                },
            }
        };

        // Reap all descendant processes here,
        // to ensure there are no children left behind.
        killpg(child_pid);
        on_exit(flags.load().hook.on_exit.as_ref(), result).await
    }
}

/// Waits until the child exits or times out, and returns ExitStatus.
/// For the case of timeout, Ok(None) will be returned.
async fn wait_child(
    child: &mut tokio::process::Child,
    flags: arc_swap::Guard<Arc<Flags>>,
) -> io::Result<Option<process::ExitStatus>> {
    match flags.timeout.kill_after {
        // Always some because no timeout given.
        None => child.wait().await.map(Some),
        Some(dur) => match time::timeout(dur, child.wait()).await {
            Err(_elapsed) => Ok(None),
            Ok(status) => status.map(Some),
        },
    }
}

fn to_exit_status(
    exit_status: process::ExitStatus,
    mut cause: ExitReasons,
    flags: &Arc<ArcSwap<Flags>>,
) -> ExitStatus {
    cause.child_signaled = exit_status.signal().or(cause.child_signaled);
    ExitStatus { exit_status, exit_reasons: cause, flags: Arc::clone(flags) }
}

#[tracing::instrument]
async fn on_exit(path: Option<&PathBuf>, result: io::Result<ExitStatus>) -> io::Result<ExitStatus> {
    if let Some(path) = path {
        if matches!(result, Ok(ref status) if status.exit_ok().is_ok()) {
            fsutil::create_file(path, true).await?;
        } else {
            fsutil::create_file(path.with_extension("err"), true).await?;
        }
    }

    result
}

async fn kill(pid: u32, signal: Option<libc::c_int>) {
    let gracefully = true;
    let pid = pid as libc::pid_t;

    // Notify the spawned process to be terminated.
    if gracefully {
        let signal = signal.unwrap_or(libc::SIGTERM);
        if let Err(err) = c::kill(pid, signal) {
            trace!("kill({}): {}", signal, err);
        }
        time::sleep(Duration::from_millis(1000)).await;
    }

    if let Err(err) = c::kill(pid, libc::SIGKILL) {
        trace!("kill({}): {}", libc::SIGKILL, err);
    }
}

fn killpg(pid: u32) {
    // Reap all descendant processes here,
    // to ensure there are no children left behind.
    if let Err(err) = c::killpg(pid as libc::c_int, libc::SIGKILL) {
        trace!("killpg({}): {}", libc::SIGKILL, err);
    }
}

impl ExitStatus {
    fn exit_ok(&self) -> Result<(), ExitStatusError> {
        let exit_success = self.exit_status.success();
        let timedout_but_ok = self.exit_reasons.timedout && self.flags.load().timeout.is_ok;
        if exit_success || timedout_but_ok { Ok(()) } else { Err(ExitStatusError(self.clone())) }
    }
}

impl ExitStatusError {
    fn exit_code(&self) -> ExitCode {
        let ws = &self.0;

        if ws.exit_reasons.timedout {
            return ExitCode::from(124);
        }

        if let Some(s) = ws.exit_reasons.iam_signaled {
            return ExitCode::from(128 + s as u8);
        }
        if let Some(s) = ws.exit_reasons.child_signaled {
            return ExitCode::from(128 + s as u8);
        }

        ws.exit_status.code().map(|c| ExitCode::from(c as u8)).unwrap_or(ExitCode::FAILURE)
    }
}
