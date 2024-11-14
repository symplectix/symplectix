//! `bits`

pub mod block {
    pub use bits_buf::Buf;
    #[doc(inline)]
    pub use bits_core::block::*;
    pub use smallset::SmallSet;
}

pub mod mask {
    #[doc(inline)]
    pub use bits_core::mask::*;
}

pub mod word {
    #[doc(inline)]
    pub use bits_core::word::Word;
}

pub use bits_aux::Pop;
pub use bits_core::{BitVec, Bits};
