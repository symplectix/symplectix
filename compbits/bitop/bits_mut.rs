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
    #[inline]
    fn set1(&mut self, i: u64) {
        let (i, o) = crate::index(i, T::BITS);
        self[i].set1(o)
    }

    #[inline]
    fn set0(&mut self, i: u64) {
        let (i, o) = crate::index(i, T::BITS);
        self[i].set0(o)
    }
}

impl<T: Block, const N: usize> BitsMut for [T; N] {
    #[inline]
    fn set1(&mut self, i: u64) {
        self.as_mut_slice().set1(i)
    }

    #[inline]
    fn set0(&mut self, i: u64) {
        self.as_mut_slice().set0(i)
    }
}

impl<T: Block> BitsMut for Vec<T> {
    #[inline]
    fn set1(&mut self, i: u64) {
        self.as_mut_slice().set1(i)
    }

    #[inline]
    fn set0(&mut self, i: u64) {
        self.as_mut_slice().set0(i)
    }
}

impl<T: BitsMut> BitsMut for Box<T> {
    #[inline]
    fn set1(&mut self, i: u64) {
        self.as_mut().set1(i)
    }

    #[inline]
    fn set0(&mut self, i: u64) {
        self.as_mut().set0(i)
    }
}
