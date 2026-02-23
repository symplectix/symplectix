use std::borrow::Cow;

use crate::{
    Bits,
    BitsMut,
    Word,
};

/// Fixed sized bits.
// TODO: remove BitsMut from constraints.
pub trait Block: Clone + Bits + BitsMut {
    /// The number of bits, which must always be equal to `Bits::bits`.
    const BITS: u64;

    /// Constructs an empty bits block.
    fn empty() -> Self;
}

/// Helper trait for blockwise iteration.
pub trait IntoBlocks: Sized {
    /// Type of a bit container.
    type Block;

    /// An iterator which yields `Block`s with its index.
    type Blocks: Iterator<Item = (usize, Self::Block)>;

    /// Returns an iterator.
    fn into_blocks(self) -> Self::Blocks;
}

/// Helper trait for blockwise iteration.
pub trait FromBlocks<B>: Sized {
    /// Constructs a value from blocks.
    fn from_blocks<T: IntoBlocks<Block = B>>(iter: T) -> Self;
}

/// Helper trait for bit masking.
///
/// The mask defines which bits to retain and which to clear.
/// Masking involves applying such a mask to self.
pub trait Mask<T: ?Sized = Self> {
    /// Performs inplace and.
    fn and(data: &mut Self, mask: &T);

    /// Performs inplace or.
    fn or(data: &mut Self, mask: &T);

    /// Performs inplace not.
    fn not(data: &mut Self, mask: &T);

    /// Performs inplace xor.
    fn xor(data: &mut Self, mask: &T);
}

impl<'a, B: Block> IntoBlocks for &'a [B] {
    type Block = Cow<'a, B>;
    type Blocks = slice::Blocks<'a, B>;
    fn into_blocks(self) -> Self::Blocks {
        slice::Blocks { blocks: self.iter().enumerate() }
    }
}

mod slice {
    use std::borrow::Cow;
    use std::iter::Enumerate;

    use crate::Block;

    pub struct Blocks<'a, B> {
        pub(crate) blocks: Enumerate<std::slice::Iter<'a, B>>,
    }
    impl<'a, B: Block> Iterator for Blocks<'a, B> {
        type Item = (usize, Cow<'a, B>);
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.blocks.find_map(|(i, b)| b.any().then(|| (i, Cow::Borrowed(b))))
        }
    }
}

impl<B: Word, const N: usize> Block for [B; N] {
    const BITS: u64 = B::BITS * N as u64;
    #[inline]
    fn empty() -> Self {
        [B::empty(); N]
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

impl<'a, B: Block> IntoBlocks for &'a Vec<B> {
    type Block = <&'a [B] as IntoBlocks>::Block;
    type Blocks = <&'a [B] as IntoBlocks>::Blocks;
    fn into_blocks(self) -> Self::Blocks {
        self.as_slice().into_blocks()
    }
}

impl<B: Block> Block for Box<B> {
    const BITS: u64 = B::BITS;
    #[inline]
    fn empty() -> Self {
        Box::new(B::empty())
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

impl<A, B> Mask<B> for Box<A>
where
    A: ?Sized + Mask<B>,
    B: ?Sized,
{
    #[inline]
    fn and(data: &mut Self, mask: &B) {
        Mask::and(data.as_mut(), mask);
    }
    #[inline]
    fn or(data: &mut Self, mask: &B) {
        Mask::or(data.as_mut(), mask);
    }
    #[inline]
    fn not(data: &mut Self, mask: &B) {
        Mask::not(data.as_mut(), mask);
    }
    #[inline]
    fn xor(data: &mut Self, mask: &B) {
        Mask::xor(data.as_mut(), mask);
    }
}

impl<'a, 'b, A, B> Mask<Cow<'b, B>> for Cow<'a, A>
where
    A: ?Sized + ToOwned,
    B: ?Sized + ToOwned,
    A::Owned: Mask<B>,
{
    #[inline]
    fn and(data: &mut Self, mask: &Cow<'b, B>) {
        Mask::and(data.to_mut(), mask);
    }
    #[inline]
    fn or(data: &mut Self, mask: &Cow<'b, B>) {
        Mask::or(data.to_mut(), mask);
    }
    #[inline]
    fn not(data: &mut Self, mask: &Cow<'b, B>) {
        Mask::not(data.to_mut(), mask);
    }
    #[inline]
    fn xor(data: &mut Self, mask: &Cow<'b, B>) {
        Mask::xor(data.to_mut(), mask);
    }
}

#[cfg(test)]
mod mask_test {
    use std::borrow::Cow;

    use crate::Bits;

    // For testing purposes only. Wrapping integers in a Cow is
    // a waste of space.
    macro_rules! impl_masking_for_word {
        ($( $Word:ty )*) => ($(
            impl crate::Mask<$Word> for $Word {
                #[inline]
                fn and(data: &mut Self, mask: &$Word) {
                    *data &= *mask;
                }
                #[inline]
                fn or(data: &mut Self, mask: &$Word) {
                    *data |= *mask;
                }
                #[inline]
                fn not(data: &mut Self, mask: &$Word) {
                    *data &= !*mask;
                }
                #[inline]
                fn xor(data: &mut Self, mask: &$Word) {
                    *data ^= *mask;
                }
            }
        )*)
    }
    impl_masking_for_word!(u8 u16 u32 u64 u128);

    #[test]
    fn intersection() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.and(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000100)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b01000000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn union() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.or(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000101)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b11100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b01100000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn difference() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.not(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000001)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b01100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b00100000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn symmetric_difference() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.xor(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000001)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b11100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b00100000)));
        assert_eq!(iter.next(), None);
    }
}
