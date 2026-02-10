//! RRR encode/decode impls where b is 31.

mod imp {
    include!(concat!(env!("OUT_DIR"), "/rrr31.rs"));
}

pub use imp::{
    CLASS_SIZE,
    decode,
    encode,
};
