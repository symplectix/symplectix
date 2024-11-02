use std::time::Duration;

use entrypoint::Entrypoint;
use runfiles::Runfiles;

static GECKODRIVER_BIN: &str = "geckodriver/geckodriver";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    let runfiles = Runfiles::create().expect("runfiles can not be created");
    let mut entrypoint = Entrypoint::new(runfiles.rlocation(GECKODRIVER_BIN));
    let driver = entrypoint.timeout(Duration::from_secs(1)).spawn().await?;
    entrypoint::wait(driver).await?;

    Ok(())
}
