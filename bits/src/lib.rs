//! `bits`

pub use bits_core::{Bits, BitsMut, Word};
pub use bits_mask as mask;
pub use bits_mask::Mask;

pub mod block {
    pub use bits_core::Block;
    pub use roaring_block::BoxContainer;
}

pub use bits_aux::Pop;
