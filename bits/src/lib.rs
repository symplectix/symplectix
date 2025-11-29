//! `bits`

pub mod block {
    #![allow(missing_docs)]
    pub use bits_buf::Buf;
    #[doc(inline)]
    pub use bits_core::block::*;
    pub use smallset::SmallSet;
}

pub mod mask {
    #![allow(missing_docs)]
    #[doc(inline)]
    pub use bits_core::mask::*;
}

pub mod word {
    #![allow(missing_docs)]
    #[doc(inline)]
    pub use bits_core::word::Word;
}

pub use bits_aux::Pop;
pub use bits_core::{BitVec, Bits};
