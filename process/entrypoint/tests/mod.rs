use std::fs;
use std::path::{Path, PathBuf};
use testing::TempDirExt;
use tokio::task;

use crate::Entrypoint;

fn sleep<S: Into<String>>(duration: S) -> Entrypoint {
    Entrypoint {
        wait_files: vec![],
        post_file: None,
        stdout: None,
        stderr: None,
        envs: vec![],
        timeout: None,
        command: "sleep".into(),
        command_args: vec![duration.into()],
    }
}

#[tokio::test]
async fn wait_for_nothing() {
    let opts = sleep("1s");
    crate::wait(&opts).await.expect("wait for nothing");
}

fn create_files<P, T>(temp_dir: &testing::TempDir, paths: T) -> (Vec<fs::File>, Vec<PathBuf>)
where
    P: AsRef<Path>,
    T: AsRef<[P]>,
{
    paths
        .as_ref()
        .iter()
        .map(|path| {
            temp_dir
                .create_file(fs::OpenOptions::new().create(true).read(true).write(true), path)
                .expect("create a temporary file")
        })
        .unzip()
}

async fn wait(paths: Vec<PathBuf>) -> crate::Result<()> {
    let mut opts = sleep("1s");
    opts.wait_files = paths;
    crate::wait(&opts).await
}

#[tokio::test]
async fn wait_for_files() {
    let temp_dir = testing::tempdir();

    let (_, mut oks) = create_files(&temp_dir, vec!["柏/の/葉/ok", "秋/葉/原/ok"]);
    wait(oks.clone()).await.expect("waiting for files created just before");

    let (_, errs) = create_files(&temp_dir, vec!["0.err"]);
    wait(oks.clone()).await.expect("affected by an error file not waiting for");

    let (_, more_oks) = create_files(&temp_dir, vec!["0"]);
    oks.extend_from_slice(&more_oks);

    let err = wait(oks.clone())
        .await
        .expect_err("should be an error if '0' and '0.err' exist at the same time");
    match err {
        crate::Error::ErrFileExists(p) => {
            assert_eq!(p, errs[0]);
        }
        others => {
            panic!("unexpected error: {others:?}")
        }
    }

    fs::remove_file(&more_oks[0]).unwrap();
    let err = wait(oks.clone())
        .await
        .expect_err("should be an error because the error file '0.err' present");

    match err {
        crate::Error::ErrFileExists(p) => {
            assert_eq!(p, errs[0]);
        }
        others => {
            panic!("unexpected error: {others:?}")
        }
    }

    fs::remove_file(&errs[0]).unwrap();
    // `wait` does not finish until the file "0" is created.
    let h = task::spawn(wait(oks.clone()));
    create_files(&temp_dir, vec!["0"]);

    h.await.unwrap().expect("should be ok")
}
