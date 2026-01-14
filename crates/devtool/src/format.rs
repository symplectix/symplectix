use std::process::{
    Command,
    Stdio,
};

use anyhow::Context as _;

use super::{
    Context,
    DevTool,
};

#[derive(Debug, Clone, clap::Parser)]
pub(crate) struct Format {
    /// Check style without formatting.
    #[clap(long)]
    check: bool,
}

impl DevTool for Format {
    fn run(self, _ctx: Context) -> anyhow::Result<()> {
        Command::new("rustup")
            .args({
                let args = ["run", "nightly", "cargo", "fmt", "--all"];
                let check = self.check.then_some("--check");
                args.into_iter().chain(check)
            })
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .context("failed to spawn process")?
            .wait()
            .context("failed to wait output")?;
        Ok(())
    }
}
