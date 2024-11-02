use entrypoint::Command;

/// An entrypoint for fuzzing.
#[derive(Clone, Debug, clap::Parser)]
pub struct Run {
    #[command(flatten)]
    command: Command,
}

impl Run {
    pub(crate) async fn run(self) -> anyhow::Result<()> {
        self.command.run().await.map_err(anyhow::Error::from)
    }
}
