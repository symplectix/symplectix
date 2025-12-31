//! An internal library for the procrun executable.

use std::ffi::OsString;
use std::process::{ExitCode, Termination};
use std::sync::Arc;

use anyhow::Context;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

/// Termination.
pub struct Exit(anyhow::Result<()>);

impl Termination for Exit {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(_) => ExitCode::SUCCESS,
            Err(ref cause) => match cause.downcast_ref::<proc::WaitStatusError>() {
                Some(err) => err.exit_code(),
                None => ExitCode::FAILURE,
            },
        }
    }
}

/// Spawns a new process using `proc` and waits the status.
#[tokio::main]
pub async fn run() -> Exit {
    Exit(try_run(std::env::args_os()).await)
}

/// Spawns a new process using `proc` and waits the status.
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
        .with(EnvFilter::from_env("RUN_LOG"))
        .init();

    Arc::new(proc::Command::from_args_os(args))
        .spawn()
        .await
        .context("Failed to spawn process")?
        .wait()
        .await
        .context("Failed to fetch wait status")?
        .exit_ok()
        .context("Got a failure on running the process")
}
