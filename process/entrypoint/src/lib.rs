use std::io;
use std::path::PathBuf;

mod coordinator;
mod process_wrapper;

pub use coordinator::Coordinator;
pub use process_wrapper::ProcessWrapper;

mod fsutil {
    use std::io;
    use std::path::Path;
    use std::process::Stdio;

    use futures::TryFutureExt;
    use tokio::fs;

    pub async fn ensure_path_is_writable<P: AsRef<Path>>(path: P) -> io::Result<()> {
        let path = path.as_ref();
        let Some(dir) = path.parent() else {
            return Err(io::Error::new(io::ErrorKind::Other, format!("no parent '{}'", path.display())));
        };

        fs::create_dir_all(dir).await
    }

    pub async fn create_file<P: AsRef<Path>>(path: P, truncate: bool) -> io::Result<std::fs::File> {
        let file =
            fs::OpenOptions::new().create(true).write(true).truncate(truncate).open(path).await?;
        Ok(file.into_std().await)
    }

    pub async fn stdio_from<P: AsRef<Path>>(path: P, truncate: bool) -> io::Result<Stdio> {
        let path = path.as_ref();
        let file = ensure_path_is_writable(path).and_then(|_| create_file(path, truncate)).await?;
        Ok(Stdio::from(file))
    }
}

/// ProcessWrapper errors.
pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(io::Error),

    #[error("error file exists at {0}")]
    ErrFileExists(PathBuf),

    #[error("failed to spawn the child process: {0}")]
    NotSpawned(io::Error),

    #[error("failed to wait the child process: {0}")]
    WaitFailed(io::Error),

    #[error("the spawned child process was killed by a signal: {0}")]
    KilledBySignal(i32),

    #[error("the spawned child exited unsuccessfully with non-zero code: {0}")]
    ExitedUnsuccessfully(i32),
}
