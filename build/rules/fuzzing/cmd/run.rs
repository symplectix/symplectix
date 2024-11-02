use entrypoint::ProcessWrapper;

/// An entrypoint for fuzzing.
#[derive(Clone, Debug, clap::Parser)]
pub struct Run {
    #[command(flatten)]
    process_wrapper: ProcessWrapper,
}

impl Run {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        self.process_wrapper.run().await.map_err(anyhow::Error::from)
    }
}
