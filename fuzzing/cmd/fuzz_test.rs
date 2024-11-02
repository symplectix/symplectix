use clap::Parser;
use entrypoint::Entrypoint;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fuzzing::run(Test::parse()).await
}

/// Runs a instrumented binary.
#[derive(Clone, Debug, Parser)]
struct Test {
    #[command(flatten)]
    entrypoint: Entrypoint,
}

impl fuzzing::Op for Test {
    async fn run(self) -> anyhow::Result<()> {
        use entrypoint::Error::*;

        let process = self.entrypoint.spawn().await?;
        match entrypoint::wait(process).await {
            Ok(_) => Ok(()),
            Err(Timedout(_)) => Ok(()),
            Err(e) => Err(anyhow::Error::from(e)),
        }
    }
}
