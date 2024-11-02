use std::fs;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

use anyhow::Context as _;

/// Prepares for fuzzing.
#[derive(Clone, Debug, clap::Parser)]
pub struct Prep {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, clap::Subcommand)]
enum Command {
    Corpus(Corpus),
}

/// Copies and renames a set of corpus files into a given directory.
#[derive(Clone, Debug, clap::Parser)]
struct Corpus {
    /// An optional file that lists corpus paths by lines.
    #[arg(long, value_name = "PATH")]
    corpus_list: Option<PathBuf>,

    ///// Provide a dictionary of input keywords.
    /////
    ///// For some input languages using a dictionary may significantly improve
    ///// the search speed.
    //#[arg(long = "dict", value_name = "PATH")]
    //dicts: Vec<PathBuf>,
    /// The path of the output directory.
    #[arg(long, value_name = "DIR")]
    output: PathBuf,
}

impl Prep {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Corpus(corpus) => {
                fs::create_dir_all(&corpus.output).with_context(|| {
                    format!("failed to create a directory at {}", corpus.output.display())
                })?;

                let corpus_file = BufReader::new({
                    let Some(corpus_list) = corpus.corpus_list.as_ref() else {
                        return Ok(());
                    };

                    fs::File::open(corpus_list).with_context(|| {
                        format!("failed to read a file at {}", corpus_list.display())
                    })?
                });

                for line in corpus_file.lines() {
                    let from = line.context("error reading lines")?;
                    let to = corpus.output.join(from.replace('/', "_"));
                    fs::copy(&from, &to).with_context(|| {
                        format!(
                            "failed to copy from {from} to {to}",
                            from = from.as_str(),
                            to = to.display()
                        )
                    })?;
                }

                Ok(())
            }
        }
    }
}
