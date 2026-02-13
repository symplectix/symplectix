use crate::{
    Bits,
    BitsMut,
    Word,
};

/// Fixed sized bits.
pub trait Block: Clone + Bits + BitsMut {
    /// The number of bits, which must always be equal to `Bits::bits`.
    const BITS: u64;

    /// Constructs an empty bits block.
    fn empty() -> Self;
}

impl<T: Word, const N: usize> Block for [T; N] {
    const BITS: u64 = T::BITS * N as u64;

    #[inline]
    fn empty() -> Self {
        [T::empty(); N]
    }
}
