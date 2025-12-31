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
    let out_dir = env::var("OUT_DIR").expect("can not find OUT_DIR");
    let mut cmd = Command::new("cc");
    let status = cmd
        .arg("src/orphan.c")
        .arg("-o")
        .arg(format!("{out_dir}/orphan"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn")
        .wait()
        .expect("failed to wait");
    if status.success() { Ok(()) } else { Err(io::Error::other("failed to compile")) }
}
