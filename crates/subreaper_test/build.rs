//! Build an executable for test.

use std::path::PathBuf;
use std::process::Command;
use std::{
    env,
    io,
};

fn main() -> io::Result<()> {
    let status = Command::new("cc")
        .arg("orphan.c")
        .arg("-o")
        .arg(PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set")).join("orphan"))
        .status()?;
    assert!(status.success());

    Ok(())
}
