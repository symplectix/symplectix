//! Provides helper function(s) for subreaper testing.

use std::path::PathBuf;

static ORPHAN: &str = concat!(env!("OUT_DIR"), "/orphan");

/// Returns the path to the executable `orphan`.
pub fn orphan() -> PathBuf {
    let path = PathBuf::from(ORPHAN);
    assert!(path.exists());
    path
}
