#![allow(missing_docs)]

mod imp {
    include!(concat!(env!("OUT_DIR"), "/rrr15.rs"));
}

pub use imp::{
    CLASS_SIZE,
    decode,
    encode,
};
