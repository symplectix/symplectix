use std::borrow::Cow;
use std::iter::Enumerate;
use std::slice;

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

impl<T: Block> Block for Box<T> {
    const BITS: u64 = T::BITS;

    #[inline]
    fn empty() -> Self {
        Box::new(T::empty())
    }
}

impl<T: Block> Block for Option<T> {
    const BITS: u64 = T::BITS;

    #[inline]
    fn empty() -> Self {
        None
    }
}

impl<T, B> Block for Cow<'_, T>
where
    T: ?Sized + ToOwned<Owned = B> + Bits,
    B: Block,
{
    const BITS: u64 = B::BITS;

    #[inline]
    fn empty() -> Self {
        Cow::Owned(B::empty())
    }
}

/// A helper trait for bit masking.
///
/// A mask defines which bits you want to keep, and which bits
/// you want to clear. Masking is to apply a mask to an another bit
/// container.
pub trait IntoBlocks: Sized {
    /// Type of a bit container.
    type Block;

    /// An iterator which yields `Block`s with its index.
    type Blocks: Iterator<Item = (usize, Self::Block)>;

    /// Returns an iterator which performs bitwise ops lazily.
    fn into_blocks(self) -> Self::Blocks;
}

/// A helper trait for bit masking.
pub trait FromBlocks<B>: Sized {
    /// Creates a value from a mask.
    fn from_blocks<T: IntoBlocks<Block = B>>(iter: T) -> Self;
}

impl<'a, T: Block> IntoBlocks for &'a [T] {
    type Block = Cow<'a, T>;
    type Blocks = SliceBlocks<'a, T>;
    fn into_blocks(self) -> Self::Blocks {
        SliceBlocks { blocks: self.iter().enumerate() }
    }
}

pub struct SliceBlocks<'a, T> {
    blocks: Enumerate<slice::Iter<'a, T>>,
}

impl<'a, T: Block> Iterator for SliceBlocks<'a, T> {
    type Item = (usize, Cow<'a, T>);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.blocks.find_map(|(i, b)| b.any().then(|| (i, Cow::Borrowed(b))))
    }
}

impl<'a, B, const N: usize> IntoBlocks for &'a [B; N]
where
    &'a [B]: IntoBlocks,
{
    type Block = <&'a [B] as IntoBlocks>::Block;
    type Blocks = <&'a [B] as IntoBlocks>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_ref().into_blocks()
    }
}

impl<'a, T: Block> IntoBlocks for &'a Vec<T> {
    type Block = <&'a [T] as IntoBlocks>::Block;
    type Blocks = <&'a [T] as IntoBlocks>::Blocks;
    fn into_blocks(self) -> Self::Blocks {
        self.as_slice().into_blocks()
    }
}

impl<'inner, T: ?Sized> IntoBlocks for &&'inner T
where
    &'inner T: IntoBlocks,
{
    type Block = <&'inner T as IntoBlocks>::Block;
    type Blocks = <&'inner T as IntoBlocks>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        IntoBlocks::into_blocks(*self)
    }
}
