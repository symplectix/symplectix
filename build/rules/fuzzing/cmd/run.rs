use entrypoint::Entrypoint;

/// An entrypoint for fuzzing.
#[derive(Clone, Debug, clap::Parser)]
pub struct Run {
    #[command(flatten)]
    entrypoint: Entrypoint,
}

impl Run {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        self.entrypoint.run().await.map_err(anyhow::Error::from)
    }
}
