//! RRR encode/decode impls where b is 4.
//! This crate is built just for testing purpose.

mod imp {
    include!(concat!(env!("OUT_DIR"), "/rrr4.rs"));
}

pub use imp::{
    CLASS_SIZE,
    decode,
    encode,
};
