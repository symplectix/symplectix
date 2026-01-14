//! This crate `devtool` provides internal tooling.

use std::path::{
    Path,
    PathBuf,
};
use std::process::Stdio;
use std::{
    env,
    process,
};

use anyhow::Context as _;

mod format;
mod info;

trait DevTool {
    fn run(self, ctx: Context) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
struct Context {
    #[allow(dead_code)]
    cargo:      PathBuf,
    revision:   String,
    run_number: String,
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
    Info(info::Info),
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

        let revision = {
            let rev_parse = process::Command::new("git")
                .arg("rev-parse")
                .arg("--short=10")
                .arg("HEAD")
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?
                .wait_with_output()?;
            // String::from_utf8_lossy_owned is unstable.
            // let revision = String::from_utf8_lossy_owned(rev_parse.stdout);
            String::from_utf8_lossy(&rev_parse.stdout).into_owned()
        };
        let revision = revision.lines().next().context("unexpected rev-parse output")?.to_owned();

        // Github Actions environment variables.
        // https://docs.github.com/en/actions/learn-github-actions/contexts#github-context
        //
        // * run_id: A unique number for each workflow run within a repository. This number does not
        //   change if you re-run the workflow run.
        // * run_number: A unique number for each run of a particular workflow in a repository. This
        //   number begins at 1 for the workflow's first run, and increments with each new run. This
        //   number does not change if you re-run the workflow run.
        // * run_attempt: A unique number for each attempt of a particular workflow run in a
        //   repository. This number begins at 1 for the workflow run's first attempt, and
        //   increments with each re-run.
        let run_number = env::var("GITHUB_RUN_NUMBER").unwrap_or("0".to_owned());

        let ctx = Context { cargo: PathBuf::from(env!("CARGO")), revision, run_number };

        match self.cmd {
            Command::Info(c) => c.run(ctx),
            Command::Format(c) => c.run(ctx),
        }
    }
}
