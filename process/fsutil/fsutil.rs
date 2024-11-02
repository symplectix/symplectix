use std::io;
use std::path::Path;
use std::process::Stdio;

use futures::TryFutureExt;
use tokio::fs;

pub async fn ensure_path_is_writable<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    let Some(dir) = path.parent() else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("no parent '{}'", path.display()),
        ));
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
