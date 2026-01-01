//! Collects build information at compile time.

use std::path::PathBuf;
use std::{
    env,
    fs,
    io,
};

fn main() -> io::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("cannot find OUT_DIR"));
    let out_path = out_dir.join("buildinfo.rs");

    println!("{:?}", out_dir);
    println!("{:?}", out_path);

    let body = r#"mod buildinfo {
    pub static FOO: &str = "FOO";
}
"#;

    fs::write(out_path, body)
}
