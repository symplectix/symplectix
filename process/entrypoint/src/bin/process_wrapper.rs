use clap::Parser;
use entrypoint::ProcessWrapper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    ProcessWrapper::parse().run().await.map_err(anyhow::Error::from)
}
