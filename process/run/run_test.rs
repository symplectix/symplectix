use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use testing::TempDirExt;
use tokio::task;

use super::{wait_for, SpawnError};
use crate::Command;

fn command<I, T>(argv: I) -> Arc<Command>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let mut argv = argv.into_iter();

    Arc::new(Command {
        dry_run: false,
        timeout: crate::Timeout { kill_after: None, is_ok: false },
        hook: crate::Hook { wait_for: vec![], on_exit: None },
        program: argv.next().unwrap().into(),
        envs: vec![],
        args: argv.map(|s| s.into()).collect::<Vec<_>>(),
    })
}

impl Command {
    fn timeout(mut self: Arc<Self>, duration: Duration) -> Arc<Self> {
        Arc::make_mut(&mut self).timeout =
            crate::Timeout { kill_after: Some(duration), is_ok: false };
        self
    }
}

#[tokio::test]
async fn executable() {
    // Should be ok because this checks whether we can spawn the process.
    assert!(command(["test", "-e", "/xxx"]).spawn().await.is_ok());
    assert!(command(["test", "-e", "/yyy"]).spawn().await.is_ok());
    assert!(command(["test", "-e", "/tmp"]).spawn().await.is_ok());
}

#[tokio::test]
async fn not_executable() {
    assert!(command(["not_command", "foo"]).spawn().await.is_err());
}

#[tokio::test]
async fn exit() {
    for i in 1..256 {
        let exit =
            command(["sh", "-c", &format!("exit {}", &i.to_string())]).spawn().await.unwrap();
        let status = exit.wait().await.expect("failed to wait");

        let Err(err) = status.exit_ok() else {
            panic!("expected that the command 'exit' exits with failure",);
        };

        assert_eq!(err.0.exit_status.code(), Some(i));
    }
}

#[tokio::test]
async fn sleep() {
    let sleep = command(["sleep", "1"]).spawn().await.unwrap();
    let status = sleep.wait().await.expect("failed to wait");
    assert!(status.exit_ok().is_ok(), "expected that the command 'sleep' exits successfully",);
}

#[tokio::test]
async fn sleep_kill() {
    for sig in [libc::SIGINT, libc::SIGTERM, libc::SIGKILL] {
        let sleep = command(["sleep", "10"]).spawn().await.unwrap();
        // sleep.inner.kill(Some(sig)).await;
        crate::child::kill(sleep.pid().unwrap() as i32, sig).expect("failed to kill");
        let status = sleep.wait().await.expect("failed to wait");
        assert!(
            matches!(status.exit_reason.proc_signaled, Some(got) if sig == got),
            "expected that the command 'sleep' signaled",
        );
    }
}

#[tokio::test]
async fn sleep_timeout() {
    let timeout = Duration::from_millis(100);
    let sleep = command(["sleep", "10"]).timeout(timeout).spawn().await.unwrap();
    let status = sleep.wait().await.expect("failed to wait");
    assert!(
        status.exit_reason.timedout,
        // matches!(status.exit_reason, Some(ExitReason::ProcTimedout)),
        "expected that the command 'sleep' times out",
    );
}

fn create_files<P, T>(temp_dir: &testing::TempDir, paths: T) -> Vec<PathBuf>
where
    P: AsRef<Path>,
    T: AsRef<[P]>,
{
    paths
        .as_ref()
        .iter()
        .map(|path| {
            let (_file, path) = temp_dir
                .create_file(fs::OpenOptions::new().create(true).read(true).write(true), path)
                .expect("create a temporary file");
            path
        })
        .collect()
}

#[tokio::test]
async fn wait_for_files() {
    let temp_dir = testing::tempdir();

    wait_for(&[]).await.expect("wait for nothing");

    let mut oks = create_files(&temp_dir, vec!["東/新宿/ok", "柏/の/葉/ok", "秋/葉/原/ok"]);
    wait_for(&oks).await.expect("waiting for files created just before");

    let err = create_files(&temp_dir, vec!["0.err"]);
    wait_for(&oks).await.expect("affected by an error file not waiting for");

    let more_oks = create_files(&temp_dir, vec!["0"]);
    oks.extend_from_slice(&more_oks);

    match wait_for(&oks)
        .await
        .expect_err("should be an error if '0' and '0.err' exist at the same time")
    {
        SpawnError::FoundErrFile(p) => {
            assert_eq!(p, err[0]);
        }
        others => {
            panic!("unexpected error: {others:?}")
        }
    }

    fs::remove_file(&more_oks[0]).unwrap();
    match wait_for(&oks)
        .await
        .expect_err("should be an error because the error file '0.err' present")
    {
        SpawnError::FoundErrFile(p) => {
            assert_eq!(p, err[0]);
        }
        others => {
            panic!("unexpected error: {others:?}")
        }
    }

    fs::remove_file(&err[0]).unwrap();
    // `wait` does not finish until the file "0" is created.
    let h = task::spawn(async move { wait_for(&oks).await });
    create_files(&temp_dir, vec!["0"]);
    h.await.unwrap().expect("should be ok");

    temp_dir.close().unwrap();
}
