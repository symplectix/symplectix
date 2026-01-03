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
    cargo: PathBuf,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Tools {
    /// Format code.
    Format(imp::Format),
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
            Tools::Format(t) => t.run(ctx).await,
        }
    }
}

pub(crate) mod imp {
    use super::{
        Context,
        Tool,
    };

    #[derive(Debug, Clone, clap::Parser)]
    pub(crate) struct Format {}

    impl Tool<()> for Format {
        async fn run(self, ctx: Context) -> anyhow::Result<()> {
            println!("{ctx:?}");
            Ok(())
        }
    }
}
