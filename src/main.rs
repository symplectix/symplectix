use std::process;

use clap::Parser;

fn main() {
    let cli = syx::Cli::parse();
    let status = cli.run().expect("failed to run cli");
    process::exit(status.code().unwrap_or(1))
}
