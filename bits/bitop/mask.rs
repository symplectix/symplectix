use std::borrow::{
    Cow,
    ToOwned,
};
use std::iter::Enumerate;
use std::slice;

use crate::Block;

/// A helper trait for bit masking.
///
/// A mask defines which bits you want to keep, and which bits
/// you want to clear. Masking is to apply a mask to an another bit
/// container.
pub trait IntoMask: Sized {
    /// Type of a bit container.
    type Bits;

    /// An iterator which yields Bits with its index.
    type Mask: Iterator<Item = (usize, Self::Bits)>;

    /// Returns an iterator which performs bitwise ops lazily.
    fn into_mask(self) -> Self::Mask;
}

/// A helper trait for bit masking.
pub trait FromMask<B>: Sized {
    /// Creates a value from a mask.
    fn from_mask<T: IntoMask<Bits = B>>(iter: T) -> Self;
}

impl<'inner, T: ?Sized> IntoMask for &&'inner T
where
    &'inner T: IntoMask,
{
    type Bits = <&'inner T as IntoMask>::Bits;
    type Mask = <&'inner T as IntoMask>::Mask;
    #[inline]
    fn into_mask(self) -> Self::Mask {
        IntoMask::into_mask(*self)
    }
}

impl<'a, T: Block> IntoMask for &'a [T] {
    type Bits = Cow<'a, T>;
    type Mask = Blocks<'a, T>;
    fn into_mask(self) -> Self::Mask {
        Blocks { blocks: self.iter().enumerate() }
    }
}

impl<'a, B, const N: usize> IntoMask for &'a [B; N]
where
    &'a [B]: IntoMask,
{
    type Bits = <&'a [B] as IntoMask>::Bits;
    type Mask = <&'a [B] as IntoMask>::Mask;
    #[inline]
    fn into_mask(self) -> Self::Mask {
        self.as_ref().into_mask()
    }
}

impl<'a, T: Block> IntoMask for &'a Vec<T> {
    type Bits = <&'a [T] as IntoMask>::Bits;
    type Mask = <&'a [T] as IntoMask>::Mask;
    fn into_mask(self) -> Self::Mask {
        self.as_slice().into_mask()
    }
}

pub struct Blocks<'a, T> {
    blocks: Enumerate<slice::Iter<'a, T>>,
}

impl<'a, T: Block> Iterator for Blocks<'a, T> {
    type Item = (usize, Cow<'a, T>);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.blocks.find_map(|(i, b)| b.any().then(|| (i, Cow::Borrowed(b))))
    }
}

pub trait Assign<That: ?Sized> {
    fn and(a: &mut Self, b: &That);
    fn not(a: &mut Self, b: &That);
    fn or(a: &mut Self, b: &That);
    fn xor(a: &mut Self, b: &That);
}

macro_rules! impl_Assign_for_word {
    ($( $Word:ty )*) => ($(
        impl Assign<$Word> for $Word {
            #[inline]
            fn and(a: &mut Self, b: &$Word) {
                *a &= *b;
            }
            #[inline]
            fn not(a: &mut Self, b: &$Word) {
                *a &= !*b;
            }
            #[inline]
            fn or(a: &mut Self, b: &$Word) {
                *a |= *b;
            }
            #[inline]
            fn xor(a: &mut Self, b: &$Word) {
                *a ^= *b;
            }
        }
    )*)
}
impl_Assign_for_word!(u8 u16 u32 u64 u128);

impl<A, B> Assign<[B]> for [A]
where
    A: Assign<B>,
{
    fn and(this: &mut Self, that: &[B]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            Assign::and(v1, v2);
        }
    }

    fn not(this: &mut Self, that: &[B]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            Assign::not(v1, v2);
        }
    }

    fn or(this: &mut Self, that: &[B]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            Assign::or(v1, v2);
        }
    }

    fn xor(this: &mut Self, that: &[B]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            Assign::xor(v1, v2);
        }
    }
}

impl<A, B: ?Sized, const N: usize> Assign<B> for [A; N]
where
    [A]: Assign<B>,
{
    #[inline]
    fn and(this: &mut Self, that: &B) {
        <[A] as Assign<B>>::and(this.as_mut(), that)
    }
    #[inline]
    fn not(this: &mut Self, that: &B) {
        <[A] as Assign<B>>::not(this.as_mut(), that)
    }
    #[inline]
    fn or(this: &mut Self, that: &B) {
        <[A] as Assign<B>>::or(this.as_mut(), that)
    }
    #[inline]
    fn xor(this: &mut Self, that: &B) {
        <[A] as Assign<B>>::xor(this.as_mut(), that)
    }
}

impl<A, B: ?Sized> Assign<B> for Vec<A>
where
    [A]: Assign<B>,
{
    #[inline]
    fn and(this: &mut Self, that: &B) {
        <[A] as Assign<B>>::and(this.as_mut(), that)
    }
    #[inline]
    fn not(this: &mut Self, that: &B) {
        <[A] as Assign<B>>::not(this.as_mut(), that)
    }
    #[inline]
    fn or(this: &mut Self, that: &B) {
        <[A] as Assign<B>>::or(this.as_mut(), that)
    }
    #[inline]
    fn xor(this: &mut Self, that: &B) {
        <[A] as Assign<B>>::xor(this.as_mut(), that)
    }
}

impl<T, U> Assign<U> for Box<T>
where
    T: ?Sized + Assign<U>,
    U: ?Sized,
{
    #[inline]
    fn and(this: &mut Self, that: &U) {
        <T as Assign<U>>::and(this, that)
    }
    #[inline]
    fn not(this: &mut Self, that: &U) {
        <T as Assign<U>>::not(this, that)
    }
    #[inline]
    fn or(this: &mut Self, that: &U) {
        <T as Assign<U>>::or(this, that)
    }
    #[inline]
    fn xor(this: &mut Self, that: &U) {
        <T as Assign<U>>::xor(this, that)
    }
}

impl<'a, 'b, T, U> Assign<Cow<'b, U>> for Cow<'a, T>
where
    T: ?Sized + ToOwned,
    U: ?Sized + ToOwned,
    T::Owned: Assign<U>,
{
    #[inline]
    fn and(this: &mut Self, that: &Cow<'b, U>) {
        <T::Owned as Assign<U>>::and(this.to_mut(), that.as_ref())
    }
    #[inline]
    fn not(this: &mut Self, that: &Cow<'b, U>) {
        <T::Owned as Assign<U>>::not(this.to_mut(), that.as_ref())
    }
    #[inline]
    fn or(this: &mut Self, that: &Cow<'b, U>) {
        <T::Owned as Assign<U>>::or(this.to_mut(), that.as_ref())
    }
    #[inline]
    fn xor(this: &mut Self, that: &Cow<'b, U>) {
        <T::Owned as Assign<U>>::xor(this.to_mut(), that.as_ref())
    }
}
