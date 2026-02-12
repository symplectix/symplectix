use crate::{
    Bits,
    Block,
};

/// A mutatable bit sequence.
pub trait BitsMut: Bits {
    /// Set a bit at `i`.
    fn set1(&mut self, i: u64);

    /// Unset a bit at `i`.
    fn set0(&mut self, i: u64);
}

impl<T: Block> BitsMut for [T] {
    fn set1(&mut self, i: u64) {
        let (i, o) = crate::index(i, T::BITS);
        self[i].set1(o)
    }

    fn set0(&mut self, i: u64) {
        let (i, o) = crate::index(i, T::BITS);
        self[i].set0(o)
    }
}
