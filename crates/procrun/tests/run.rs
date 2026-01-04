#![allow(missing_docs)]

use std::io::{
    BufRead,
    BufReader,
    Cursor,
};
use std::path::Path;
use std::process::{
    Command,
    Stdio,
};

fn procrun() -> &'static Path {
    static PROCRUN_BIN: &'static str = env!("CARGO_BIN_EXE_procrun");
    Path::new(PROCRUN_BIN)
}

fn orphan() -> &'static Path {
    static ORPHAN_BIN: &str = procrun_test::ORPHAN_BIN;
    Path::new(ORPHAN_BIN)
}

#[test]
fn can_find_procrun_bin() {
    assert!(procrun().exists());
    assert!(byc::faccess().x_ok().at(procrun()).is_ok());
}

#[test]
fn can_find_orphan_bin() {
    assert!(orphan().exists());
    assert!(byc::faccess().x_ok().at(orphan()).is_ok());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrphanLog {
    pid:    String,
    group:  String,
    parent: String,
}

// Test that procrun_test/src/orphan.c behaves as expected.
#[test]
fn run_orphan_subreaper() {
    let procrun = Command::new(procrun())
        .arg(orphan())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .env("PROCRUN_LOG", "trace")
        .spawn()
        .expect("failed to spawn procrun");
    let procrun_id = procrun.id();

    let out = procrun.wait_with_output().expect("failed to wait outout");
    assert!(out.status.success());

    let stdout = BufReader::new(Cursor::new(out.stdout));
    let lines = stdout
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let kvs = line.split_terminator('\t').collect::<Vec<_>>();
            OrphanLog {
                pid:    kvs[1].to_owned(),
                group:  kvs[2].to_owned(),
                parent: kvs[3].to_owned(),
            }
        })
        .collect::<Vec<_>>();
    // every processes belong to the same group.
    assert!(all_eq(lines.iter().map(|e| &e.group)));

    // head: the first process spawned by procrun.
    let head = &lines[0];
    assert_eq!(head.parent, format!("parent={procrun_id}"));

    // last: the orphan process, should be reparented to procrun.
    let last = &lines[lines.len() - 1];
    assert_eq!(last.parent, format!("parent={procrun_id}"));
}

fn all_eq<I>(it: I) -> bool
where
    I: IntoIterator,
    I::Item: PartialEq,
{
    let mut iter = it.into_iter();
    let init = iter.next();
    iter.fold(init, |acc, item| acc.and_then(|acc| (acc == item).then_some(acc))).is_some()
}
