//! Builds a C executable.

use std::process::{
    Command,
    Stdio,
};
use std::{
    env,
    io,
};

fn main() -> io::Result<()> {
    println!("cargo::rerun-if-changed=src/orphan.c");
    let out_dir = env::var("OUT_DIR").expect("can not find OUT_DIR");
    let mut cc = Command::new("cc");
    let status = cc
        .arg("src/orphan.c")
        .arg("-o")
        .arg(format!("{out_dir}/orphan"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn")
        .wait()
        .expect("failed to wait");
    if status.success() { Ok(()) } else { Err(io::Error::other("failed to compile")) }
}
