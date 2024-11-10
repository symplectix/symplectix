use std::borrow::Cow;

use crate::{Bits, BitsMut};

pub trait Block: Clone + Bits + BitsMut {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn empty() -> Self;
}

impl<B, const N: usize> Block for [B; N]
where
    B: Copy + Block,
{
    const BITS: usize = B::BITS * N;

    #[inline]
    fn empty() -> Self {
        [B::empty(); N]
    }
}

impl<B: Block> Block for Box<B> {
    const BITS: usize = B::BITS;
    #[inline]
    fn empty() -> Self {
        Box::new(B::empty())
    }
}

impl<'a, B: Block> Block for Cow<'a, B> {
    const BITS: usize = B::BITS;
    #[inline]
    fn empty() -> Self {
        Cow::Owned(B::empty())
    }
}
