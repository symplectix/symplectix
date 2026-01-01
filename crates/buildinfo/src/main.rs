//! Just print buildinfo collected at compile time.

include!(concat!(env!("OUT_DIR"), "/print_buildinfo.rs"));
