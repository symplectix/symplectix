use entrypoint::Entrypoint;

/// Runs a instrumented binary.
#[derive(Clone, Debug, clap::Parser)]
pub struct Test {
    #[command(flatten)]
    entrypoint: Entrypoint,
}

impl Test {
    pub(crate) async fn run(self) -> anyhow::Result<()> {
        use entrypoint::Error::*;

        let process = self.entrypoint.spawn().await?;
        match entrypoint::wait(process).await {
            Ok(_) => Ok(()),
            Err(Timedout(_)) => Ok(()),
            Err(e) => Err(anyhow::Error::from(e)),
        }
    }
}
