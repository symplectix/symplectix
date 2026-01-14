#![allow(missing_docs)]

use std::ffi::OsStr;
use std::io::{
    BufRead,
    BufReader,
};
use std::path::Path;
use std::process::{
    Command,
    Stdio,
};
use std::thread;
use std::time::Duration;

fn procrun() -> &'static Path {
    static PROCRUN_BIN: &str = env!("CARGO_BIN_EXE_procrun");
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
    pid:  String,
    pgid: String,
    ppid: String,
}

#[test]
fn procrun_orphan_behave_as_expected() {
    let procrun = Command::new(procrun())
        .arg(orphan())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .env("PROCRUN_LOG", "info")
        .spawn()
        .expect("failed to spawn procrun");
    let procrun_id = procrun.id();

    let out = procrun.wait_with_output().expect("failed to wait outout");
    assert!(out.status.success());

    let stderr = BufReader::new(&out.stderr[..]);
    let lines = stderr
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                return None;
            }
            let kvs = line.split_terminator('\t').collect::<Vec<_>>();
            if kvs.len() != 4 {
                // Not the line what we are looking for.
                None
            } else {
                Some(OrphanLog {
                    pid:  kvs[1].to_owned(),
                    pgid: kvs[2].to_owned(),
                    ppid: kvs[3].to_owned(),
                })
            }
        })
        .collect::<Vec<_>>();

    dbg!(&lines);
    if !lines.is_empty() {
        // head: the first process spawned by procrun.
        let head = &lines[0];
        assert_eq!(head.ppid, format!("parent={procrun_id}"));

        // The parent process immediately exits to make the child process an orphan process. While
        // it might be possible to reliably obtain the output of the orphaned child process,
        // I don't believe procrun guarantees this.
        if lines.len() == 2 {
            // Both of parent and child are belong to the same group.
            assert!(all_eq(lines.iter().map(|e| &e.pgid)));

            // the orphan process should be reparented to procrun.
            let last = &lines[1];
            assert_eq!(last.ppid, format!("parent={procrun_id}"));
        }
    }
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

fn from_args<I, T>(args: I) -> Command
where
    I: IntoIterator<Item = T>,
    T: AsRef<OsStr>,
{
    let mut cmd = Command::new(procrun());
    cmd.arg("--")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .env("PROCRUN_LOG", "debug");
    cmd
}

fn timeout<I, T>(duration: T, args: I) -> Command
where
    I: IntoIterator<Item = T>,
    T: AsRef<OsStr>,
{
    let mut cmd = Command::new(procrun());
    cmd.arg("--kill-after")
        .arg(duration)
        .arg("--")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .env("PROCRUN_LOG", "debug");
    cmd
}

#[test]
fn procrun_success_status() {
    let r = from_args(["test", "-e", "/tmp"]).output().unwrap();
    assert!(r.status.success());
}

#[test]
fn procrun_failure_status() {
    let r = from_args(["test", "-e", "/xxx"]).output().unwrap();
    assert!(!r.status.success());
    let r = from_args(["not_command", "foo"]).output().unwrap();
    assert!(!r.status.success());
}

#[test]
fn procrun_exits_with_same_code_with_its_child() {
    let exit = from_args(["sh", "-c", "exit 0"]).output().unwrap();
    assert!(exit.status.success());
    assert_eq!(exit.status.code(), Some(0));

    let exit = from_args(["sh", "-c", "exit 10"]).output().unwrap();
    assert!(!exit.status.success());
    assert_eq!(exit.status.code(), Some(10));

    let exit = from_args(["sh", "-c", "exit 20"]).output().unwrap();
    assert!(!exit.status.success());
    assert_eq!(exit.status.code(), Some(20));
}

#[test]
fn procrun_sleep_kill() {
    let mut sleep = from_args(["sleep", "10"]).spawn().unwrap();
    // Cannot obtain the expected exit status
    // if you kill it too quickly,
    thread::sleep(Duration::from_secs(1));
    unsafe { libc::kill(sleep.id() as i32, libc::SIGTERM) };
    let status = sleep.wait().unwrap();
    assert!(!status.success());
    assert_eq!(status.code(), Some(143));
}

#[test]
fn procrun_exits_with_124_when_timedout() {
    let sleep = timeout("10ms", ["sleep", "1"]).output().unwrap();
    assert!(!sleep.status.success());
    assert_eq!(sleep.status.code(), Some(124));

    let sleep = timeout("10s", ["sleep", "1"]).output().unwrap();
    assert!(sleep.status.success());
    assert_eq!(sleep.status.code(), Some(0));
}
