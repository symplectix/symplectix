use clap::Parser;

mod cmd;

#[derive(Clone, Debug, Parser)]
pub enum Fuzzing {
    Run(cmd::Run),
}

impl Fuzzing {
    async fn run(&self) -> entrypoint::Result {
        match self {
            Fuzzing::Run(f) => f.run().await,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    Fuzzing::parse().run().await.map_err(anyhow::Error::from)
}
