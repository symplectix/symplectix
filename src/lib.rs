use std::env;
use std::path::{
    Path,
    PathBuf,
};

trait Tool<T> {
    async fn run(self, ctx: Context) -> anyhow::Result<T>;
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
    /// Print version.
    Version(imp::Version),
}

impl Cli {
    /// Run a tool and wait its result.
    pub async fn run(self) -> anyhow::Result<()> {
        let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(project_root.join(".git/HEAD").exists());
        // Always run syx from the project root for consistency.
        env::set_current_dir(project_root)?;

        let ctx = Context { cargo: PathBuf::from(env!("CARGO")) };

        match self.tools {
            Tools::Version(t) => t.run(ctx).await,
        }
    }
}

pub(crate) mod imp {
    use std::process::Stdio;

    use anyhow::Context as _;

    use super::{
        Context,
        Tool,
    };

    #[derive(Debug, Clone, clap::Parser)]
    pub(crate) struct Version {}

    impl Tool<()> for Version {
        async fn run(self, ctx: Context) -> anyhow::Result<()> {
            proc::Flags::from_args_os([
                "syx",
                "--",
                ctx.cargo.to_str().unwrap(),
                // "rustup",
                // "run",
                // "nightly",
                // "cargo",
                "fmt",
                "--all",
                "--check",
            ])
            .command()
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .await
            .context("Failed to spawn process")?
            .wait()
            .await
            .context("Failed to wait output")?
            .exit_ok()
            .context("Failed to wait output")
        }
    }
}
