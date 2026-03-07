use std::path::PathBuf;

static ORPHAN: &str = concat!(env!("OUT_DIR"), "/orphan");

pub fn orphan() -> PathBuf {
    let path = PathBuf::from(ORPHAN);
    assert!(path.exists());
    path
}
