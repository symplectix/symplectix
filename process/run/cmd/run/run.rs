use std::process::{ExitCode, Termination};
use std::sync::Arc;

use anyhow::Context;
use tracing_subscriber::{prelude::*, EnvFilter};

struct Exit(anyhow::Result<()>);

impl Termination for Exit {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(_) => ExitCode::SUCCESS,
            Err(ref cause) => match cause.downcast_ref::<run::WaitStatusError>() {
                Some(err) => err.exit_code(),
                None => ExitCode::FAILURE,
            },
        }
    }
}

#[tokio::main]
async fn main() -> Exit {
    Exit(run_proc().await)
}

async fn run_proc() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .with_target(false)
                .without_time(),
        )
        .with(EnvFilter::from_env("RUN_LOG"))
        .init();

    let entrypoint = run::Command::from_args_os(std::env::args_os());

    let status = Arc::new(entrypoint)
        .spawn()
        .await
        .context("Failed to spawn process")?
        .wait()
        .await
        .context("Failed to fetch wait status")?;

    status.exit_ok().context("Got a failure on running the process")
}
