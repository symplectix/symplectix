mod block_mut;
mod count;
mod excess;
mod pack;
mod rank;
mod select;

pub use block_mut::BlockMut;
pub use count::Count;
pub use excess::Excess;
pub use pack::Pack;
pub use rank::Rank;
pub use select::Select;

use crate::Bits;

pub trait Block: Clone {
    /// The number of bits.
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    /// Returns an empty bits block.
    /// The number of bits in block must be zero.
    fn empty() -> Self;

    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **None**.
    fn test(&self, i: usize) -> Option<bool>;
}

impl<B: Copy + Block, const N: usize> Block for [B; N] {
    const BITS: usize = B::BITS * N;

    #[inline]
    fn empty() -> Self {
        [B::empty(); N]
    }
    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        Bits::new(self.as_slice()).test(i)
    }
}

impl<B: Block> Block for Box<B> {
    const BITS: usize = B::BITS;

    #[inline]
    fn empty() -> Self {
        Box::new(B::empty())
    }
    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        self.as_ref().test(i)
    }
}
