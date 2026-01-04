use std::env;
use std::path::{
    Path,
    PathBuf,
};
use std::process::ExitStatus;

trait Tool {
    fn run(self, ctx: Context) -> anyhow::Result<ExitStatus>;
}

/// Repository management/automation tools.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    tools: Tools,
}

#[derive(Debug, Clone)]
struct Context {
    #[allow(dead_code)]
    cargo: PathBuf,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Tools {
    /// Check code style.
    Format(imp::Format),
}

impl Cli {
    /// Run a tool and wait its result.
    pub fn run(self) -> anyhow::Result<ExitStatus> {
        let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(project_root.join(".git/HEAD").exists());
        // Always run syx from the project root for consistency.
        env::set_current_dir(project_root)?;

        let ctx = Context { cargo: PathBuf::from(env!("CARGO")) };

        match self.tools {
            Tools::Format(t) => t.run(ctx),
        }
    }
}

pub(crate) mod imp {
    use std::process::{
        Command,
        ExitStatus,
        Stdio,
    };

    use anyhow::Context as _;

    use super::{
        Context,
        Tool,
    };

    #[derive(Debug, Clone, clap::Parser)]
    pub(crate) struct Format {}

    impl Tool for Format {
        fn run(self, _ctx: Context) -> anyhow::Result<ExitStatus> {
            Command::new("rustup")
                .args(["run", "nightly", "cargo", "fmt", "--all", "--check"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .context("failed to spawn process")?
                .wait()
                .context("failed to wait output")
        }
    }
}
