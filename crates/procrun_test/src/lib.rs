//! An internal library for the procrun_orphan executable.

/// Path to the executable.
pub static ORPHAN_BIN: &str = concat!(env!("OUT_DIR"), "/orphan");
