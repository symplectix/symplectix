use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = syx::Cli::parse();
    cli.run().await
}
