//! Collects build information at compile time.

use std::path::PathBuf;
use std::process::{
    Command,
    Stdio,
};
use std::{
    env,
    fs,
    io,
};

fn git() -> Command {
    Command::new("git")
}

fn main() -> io::Result<()> {
    let out_path = {
        let out_dir = env::var("OUT_DIR").expect("cannot find OUT_DIR");
        PathBuf::from(out_dir).join("print_buildinfo.rs")
    };

    // for (k, v) in env::vars() {
    //     println!("{} {}", k, v);
    // }

    // let cargo = env!("CARGO");
    // let pkgver = env!("CARGO_PKG_VERSION");
    // let pkgver_major = env!("CARGO_PKG_VERSION_MAJOR");
    // let pkgver_minor = env!("CARGO_PKG_VERSION_MINOR");
    // let pkgver_patch = env!("CARGO_PKG_VERSION_PATCH");

    let rev_count = git()
        .arg("rev-list")
        .arg("--count")
        .arg("HEAD")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?
        .wait_with_output()?;

    let rev_parse = git()
        .arg("rev-parse")
        .arg("--short=10")
        .arg("HEAD")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?
        .wait_with_output()?;

    let run_id = env::var("GITHUB_RUN_ID").unwrap_or("0".to_owned());
    let run_number = env::var("GITHUB_RUN_NUMBER").unwrap_or("0".to_owned());
    let run_attempt = env::var("GITHUB_RUN_ATTEMPT").unwrap_or("0".to_owned());

    let body = format!(
        r#"
fn buildinfo () {{
    println!("pkgver: r{rev_count}.{revision}");
    println!("run: {run_id}-{run_number}-{run_attempt}");
}}

fn main() {{
    buildinfo();
}}
"#,
        revision = &String::from_utf8_lossy(&rev_parse.stdout)[..rev_parse.stdout.len() - 1],
        rev_count = &String::from_utf8_lossy(&rev_count.stdout)[..rev_count.stdout.len() - 1]
    );

    fs::write(out_path, body)
}
