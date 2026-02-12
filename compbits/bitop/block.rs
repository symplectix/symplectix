use crate::{
    Bits,
    BitsMut,
};

/// Fixed sized bits.
pub trait Block: Clone + Bits + BitsMut {
    /// The number of bits, which must always be equal to `Bits::bits`.
    const BITS: u64;

    /// Constructs an empty bits block.
    fn empty() -> Self;
}
