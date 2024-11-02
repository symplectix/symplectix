use std::path::PathBuf;

use clap::Parser;
use entrypoint::Entrypoint;

#[derive(Clone, Debug, Parser)]
pub struct Run {
    /// Custom corpus directories or artifact files.
    #[arg(long = "corpus", value_name = "DIR")]
    corpus: Option<PathBuf>,

    /// Provide a dictionary of input keywords.
    ///
    /// For some input languages using a dictionary may significantly improve
    /// the search speed.
    #[arg(long = "dict", value_name = "PATH")]
    dicts: Vec<PathBuf>,

    /// A fuzzing entrypoint.
    #[clap(flatten)]
    entrypoint: Entrypoint,
}

impl Run {
    pub(crate) async fn run(&self) -> entrypoint::Result {
        self.entrypoint.run().await
    }
}
