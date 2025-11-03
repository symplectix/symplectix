use std::io;
use std::path::Path;
use std::process::Stdio;

use tokio::fs;

async fn create_dirs_if_missing<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    if let Some(dir) = path.parent() {
        if !dir.as_os_str().is_empty() {
            return fs::create_dir_all(dir).await;
        }
    };

    Ok(())
}

pub async fn create_file<P: AsRef<Path>>(path: P, truncate: bool) -> io::Result<std::fs::File> {
    let file = {
        let path = path.as_ref();
        create_dirs_if_missing(path).await?;
        fs::OpenOptions::new().create(true).write(true).truncate(truncate).open(path).await?
    };

    Ok(file.into_std().await)
}

pub async fn stdio_from<P: AsRef<Path>>(path: P, truncate: bool) -> io::Result<Stdio> {
    Ok(Stdio::from(create_file(path, truncate).await?))
}
