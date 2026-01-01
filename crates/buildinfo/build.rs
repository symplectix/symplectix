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

    let revision = {
        let rev_parse = git()
            .arg("rev-parse")
            .arg("--short=10")
            .arg("HEAD")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?
            .wait_with_output()?;
        // String::from_utf8_lossy_owned is unstable.
        // let revision = String::from_utf8_lossy_owned(rev_parse.stdout);
        String::from_utf8_lossy(&rev_parse.stdout).into_owned()
    };
    let revision = revision.lines().next().expect("unexpected rev-parse output");

    // Github Actions environment variables.
    // https://docs.github.com/en/actions/learn-github-actions/contexts#github-context
    //
    // * run_id: A unique number for each workflow run within a repository. This number does not
    //   change if you re-run the workflow run.
    // * run_number: A unique number for each run of a particular workflow in a repository. This
    //   number begins at 1 for the workflow's first run, and increments with each new run. This
    //   number does not change if you re-run the workflow run.
    // * run_attempt: A unique number for each attempt of a particular workflow run in a repository.
    //   This number begins at 1 for the workflow run's first attempt, and increments with each
    //   re-run.
    let run_number = env::var("GITHUB_RUN_NUMBER").unwrap_or("0".to_owned());

    let body = format!(
        r#"
fn buildinfo () {{
    println!("r{run_number}.{revision}");
}}

fn main() {{
    buildinfo();
}}
"#,
    );

    fs::write(out_path, body)
}
