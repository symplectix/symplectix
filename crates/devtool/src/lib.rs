//! This crate `devtool` provides internal tooling.

use std::env;
use std::path::{
    Path,
    PathBuf,
};

mod buildinfo;
mod format;

trait DevTool {
    fn run(self, ctx: Context) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
struct Context {
    #[allow(dead_code)]
    cargo: PathBuf,
}

/// Arguments for the devtool cli.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Command {
    /// Print buildinfo collected at compile time.
    BuildInfo(buildinfo::BuildInfo),
    /// Format code.
    Format(format::Format),
}

impl Cli {
    /// Run a tool and wait its result.
    pub fn run(self) -> anyhow::Result<()> {
        let project_root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        assert!(project_root.join(".git/HEAD").exists());
        // Always run tools from the project root for consistency.
        env::set_current_dir(project_root)?;

        let ctx = Context { cargo: PathBuf::from(env!("CARGO")) };

        match self.cmd {
            Command::BuildInfo(c) => c.run(ctx),
            Command::Format(c) => c.run(ctx),
        }
    }
}
