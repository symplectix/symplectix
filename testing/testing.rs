#![allow(missing_docs)]
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::LazyLock as Lazy;
use std::{env, fs, io};

use runfiles::Runfiles;
pub use tempfile::TempDir;

// The local repository's workspace name.
pub static WORKSPACE: Lazy<String> =
    Lazy::new(|| env::var("TEST_WORKSPACE").expect("fetching the environment variable 'TEST_WORKSPACE'"));

static RUNFILES: Lazy<Runfiles> = Lazy::new(|| Runfiles::create().expect("runfiles can not be created"));

// Signifies test executable is being driven by bazel test.
// https://bazel.build/reference/test-encyclopedia
static BAZEL_TEST: Lazy<bool> = Lazy::new(|| if let Ok(val) = env::var("BAZEL_TEST") { val == "1" } else { false });

/// Absolute path to the base of the runfiles tree.
pub static SRCDIR: Lazy<String> =
    Lazy::new(|| env::var("TEST_SRCDIR").expect("fetching the environment variable 'TEST_SRCDIR'"));

/// Absolute path to a private writable directory.
pub static TMPDIR: Lazy<String> =
    Lazy::new(|| env::var("TEST_TMPDIR").expect("fetching the environment variable 'TEST_TMPDIR'"));

fn in_bazel_test() -> bool {
    *BAZEL_TEST
}

/// Returns the runtime path of a runfile, assuming the path is from the workspace root.
pub fn rlocation(path: impl AsRef<Path>) -> PathBuf {
    runfiles::rlocation!(RUNFILES, path).unwrap()
}

/// Create a new temporary directory in [`TMPDIR`].
/// The directory is automatically removed when the `TempDir` [drop](std::ops::Drop)s.
pub fn tempdir() -> TempDir {
    assert!(in_bazel_test());
    tempfile::tempdir_in(&*TMPDIR).expect("creating a temporary directory in testing::TMPDIR")
}

/// Creates a new temporary directory in the `path` adjoined to [`TMPDIR`].
/// Panics if the `path` is not relative.
pub fn tempdir_in<P: AsRef<Path>>(path: P) -> TempDir {
    assert!(in_bazel_test());
    assert!(path.as_ref().is_relative());

    let dir = Path::new(&*TMPDIR).join(path);
    fs::create_dir_all(&dir)
        .and_then(|_| tempfile::tempdir_in(&dir))
        .expect("creating a temporary directory in testing::TMPDIR")
}

mod private {
    pub trait Sealed {}
    impl Sealed for tempfile::TempDir {}
}

pub trait TempDirExt: private::Sealed {
    /// Creates a new temporary file in `self.path()`.
    ///
    /// For various reasons, getting a `Path` from a `File` is not trivial.
    /// If you need a temporary file and its path,
    /// [`create_file`](TempDirExt::create_file) is available for such case.
    fn tempfile(&self) -> File;

    /// Creates a new temporary file at the `path` adjoined to `self.path()`.
    /// Panics if the `path` is not relative.
    ///
    /// Note that reopening a file with the same path does not necessarily open the same file.
    fn create_file<P>(&self, options: &fs::OpenOptions, path: P) -> io::Result<(File, PathBuf)>
    where
        P: AsRef<Path>;
}

impl TempDirExt for TempDir {
    fn tempfile(&self) -> File {
        tempfile::tempfile_in(self.path()).expect("creating a temporary file")
    }

    fn create_file<P>(&self, options: &fs::OpenOptions, path: P) -> io::Result<(File, PathBuf)>
    where
        P: AsRef<Path>,
    {
        assert!(path.as_ref().is_relative());

        let filepath = self.path().join(path);
        let Some(dir) = filepath.parent() else {
            return Err(io::Error::other(format!("no parent '{}'", filepath.display())));
        };

        fs::create_dir_all(dir).and_then(|_| options.open(&filepath)).map(|file| (file, filepath))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worspace() {
        assert_eq!(rlocation("syx").join(".rustfmt.toml"), rlocation("syx/.rustfmt.toml"),);
        assert_eq!(rlocation("rules_rust").join(".rustfmt.toml"), rlocation("rules_rust/.rustfmt.toml"),);
        assert_eq!(rlocation("rules_rust").join(".clippy.toml"), rlocation("rules_rust/.clippy.toml"),);
    }
}
