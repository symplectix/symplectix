#![allow(missing_docs)]
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fuzzing::run(Corpus::parse()).await
}

/// Copies and renames a set of corpus files into a given directory.
#[derive(Clone, Debug, clap::Parser)]
pub struct Corpus {
    /// An optional file that lists corpus paths by lines.
    #[arg(long, value_name = "PATH")]
    corpus_list: Option<PathBuf>,

    /// The path of the output directory.
    #[arg(long, value_name = "DIR")]
    output: PathBuf,
}

impl fuzzing::Op for Corpus {
    async fn run(self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output).with_context(|| {
            format!("failed to create a directory at {}", self.output.display())
        })?;

        let corpus_file = BufReader::new({
            let Some(corpus_list) = self.corpus_list.as_ref() else {
                return Ok(());
            };

            fs::File::open(corpus_list)
                .with_context(|| format!("failed to read a file at {}", corpus_list.display()))?
        });

        for line in corpus_file.lines() {
            let from = line.context("error reading lines")?;
            let to = self.output.join(from.replace('/', "_"));
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
