use crate::*;

#[cfg(feature = "alloc")]
pub use impl_alloc::Blocks;

pub trait Block: Clone + Bits + Count + Rank + Excess + Select + BitsMut {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn empty() -> Self;
}

impl Block for bool {
    const BITS: usize = 1;

    #[inline]
    fn empty() -> Self {
        false
    }
}

impl<T, const N: usize> Block for [T; N]
where
    T: Copy + Block,
{
    const BITS: usize = T::BITS * N;

    #[inline]
    fn empty() -> Self {
        [T::empty(); N]
    }
}

pub trait IntoBlocks {
    type Block;

    type Blocks: Iterator<Item = (usize, Self::Block)>;

    fn into_blocks(self) -> Self::Blocks;
}

impl<'inner, 'outer, T: ?Sized> IntoBlocks for &'outer &'inner T
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

impl<'a, T, const N: usize> IntoBlocks for &'a [T; N]
where
    &'a [T]: IntoBlocks,
{
    type Block = <&'a [T] as IntoBlocks>::Block;
    type Blocks = <&'a [T] as IntoBlocks>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_ref().into_blocks()
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::borrow::Cow;
    use alloc::boxed::Box;
    use core::{iter::Enumerate, slice};

    impl<T: Block> Block for Box<T> {
        const BITS: usize = T::BITS;
        #[inline]
        fn empty() -> Self {
            Box::new(T::empty())
        }
    }

    impl<'a, T> Block for Cow<'a, T>
    where
        T: ?Sized + Block,
    {
        const BITS: usize = T::BITS;
        #[inline]
        fn empty() -> Self {
            Cow::Owned(T::empty())
        }
    }

    impl<'a, T: Block> IntoBlocks for &'a [T] {
        type Block = Cow<'a, T>;
        type Blocks = Blocks<'a, T>;
        fn into_blocks(self) -> Self::Blocks {
            Blocks {
                blocks: self.iter().enumerate(),
            }
        }
    }

    pub struct Blocks<'a, T> {
        blocks: Enumerate<slice::Iter<'a, T>>,
    }

    impl<'a, T: Block> Iterator for Blocks<'a, T> {
        type Item = (usize, Cow<'a, T>);
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.blocks
                .find_map(|(i, b)| b.any().then(|| (i, Cow::Borrowed(b))))
        }
    }
}
