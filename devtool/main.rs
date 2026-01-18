//! A cli to run `devtool`s.

use std::process::ExitCode;

use clap::Parser;

fn main() -> anyhow::Result<ExitCode> {
    devtool::Cli::parse().run()?;
    Ok(ExitCode::SUCCESS)
}
