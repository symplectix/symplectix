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

/// Performs inplace intersection.
pub trait Intersection<That: ?Sized> {
    /// self & that.
    fn intersection(&mut self, that: &That);
}

/// Performs inplace union.
pub trait Union<That: ?Sized> {
    /// self | that.
    fn union(&mut self, that: &That);
}

/// Performs inplace difference.
pub trait Difference<That: ?Sized> {
    /// self & not that.
    fn difference(&mut self, that: &That);
}

/// Performs inplace symmetric difference.
pub trait SymmetricDifference<That: ?Sized> {
    /// self ^ that.
    fn symmetric_difference(&mut self, that: &That);
}

macro_rules! impl_Assign_for_word {
    ($( $Word:ty )*) => ($(
        impl Intersection<$Word> for $Word {
            #[inline]
            fn intersection(&mut self, that: &$Word) {
                *self &= *that;
            }
        }
        impl Union<$Word> for $Word {
            #[inline]
            fn union(&mut self, that: &$Word) {
                *self |= *that;
            }
        }
        impl Difference<$Word> for $Word {
            #[inline]
            fn difference(&mut self, that: &$Word) {
                *self &= !*that;
            }
        }
        impl SymmetricDifference<$Word> for $Word {
            #[inline]
            fn symmetric_difference(&mut self, that: &$Word) {
                *self ^= *that;
            }
        }
    )*)
}
impl_Assign_for_word!(u8 u16 u32 u64 u128);

impl<A, B> Intersection<[B]> for [A]
where
    A: Intersection<B>,
{
    fn intersection(&mut self, that: &[B]) {
        assert_eq!(self.len(), that.len());
        for (v1, v2) in self.iter_mut().zip(that) {
            v1.intersection(v2);
        }
    }
}

impl<A, B> Union<[B]> for [A]
where
    A: Union<B>,
{
    fn union(&mut self, that: &[B]) {
        assert_eq!(self.len(), that.len());
        for (v1, v2) in self.iter_mut().zip(that) {
            v1.union(v2);
        }
    }
}

impl<A, B> Difference<[B]> for [A]
where
    A: Difference<B>,
{
    fn difference(&mut self, that: &[B]) {
        assert_eq!(self.len(), that.len());
        for (v1, v2) in self.iter_mut().zip(that) {
            v1.difference(v2);
        }
    }
}

impl<A, B> SymmetricDifference<[B]> for [A]
where
    A: SymmetricDifference<B>,
{
    fn symmetric_difference(&mut self, that: &[B]) {
        assert_eq!(self.len(), that.len());
        for (v1, v2) in self.iter_mut().zip(that) {
            v1.symmetric_difference(v2);
        }
    }
}

impl<A, B: ?Sized, const N: usize> Intersection<B> for [A; N]
where
    [A]: Intersection<B>,
{
    #[inline]
    fn intersection(&mut self, that: &B) {
        self.as_mut_slice().intersection(that);
    }
}
impl<A, B: ?Sized, const N: usize> Union<B> for [A; N]
where
    [A]: Union<B>,
{
    #[inline]
    fn union(&mut self, that: &B) {
        self.as_mut_slice().union(that);
    }
}
impl<A, B: ?Sized, const N: usize> Difference<B> for [A; N]
where
    [A]: Difference<B>,
{
    #[inline]
    fn difference(&mut self, that: &B) {
        self.as_mut_slice().difference(that);
    }
}
impl<A, B: ?Sized, const N: usize> SymmetricDifference<B> for [A; N]
where
    [A]: SymmetricDifference<B>,
{
    #[inline]
    fn symmetric_difference(&mut self, that: &B) {
        self.as_mut_slice().symmetric_difference(that);
    }
}

impl<A, B: ?Sized> Intersection<B> for Vec<A>
where
    [A]: Intersection<B>,
{
    #[inline]
    fn intersection(&mut self, that: &B) {
        self.as_mut_slice().intersection(that);
    }
}
impl<A, B: ?Sized> Union<B> for Vec<A>
where
    [A]: Union<B>,
{
    #[inline]
    fn union(&mut self, that: &B) {
        self.as_mut_slice().union(that);
    }
}
impl<A, B: ?Sized> Difference<B> for Vec<A>
where
    [A]: Difference<B>,
{
    #[inline]
    fn difference(&mut self, that: &B) {
        self.as_mut_slice().difference(that);
    }
}
impl<A, B: ?Sized> SymmetricDifference<B> for Vec<A>
where
    [A]: SymmetricDifference<B>,
{
    #[inline]
    fn symmetric_difference(&mut self, that: &B) {
        self.as_mut_slice().symmetric_difference(that);
    }
}

impl<A, B> Intersection<B> for Box<A>
where
    A: ?Sized + Intersection<B>,
    B: ?Sized,
{
    #[inline]
    fn intersection(&mut self, that: &B) {
        self.as_mut().intersection(that);
    }
}
impl<A, B> Union<B> for Box<A>
where
    A: ?Sized + Union<B>,
    B: ?Sized,
{
    #[inline]
    fn union(&mut self, that: &B) {
        self.as_mut().union(that);
    }
}
impl<A, B> Difference<B> for Box<A>
where
    A: ?Sized + Difference<B>,
    B: ?Sized,
{
    #[inline]
    fn difference(&mut self, that: &B) {
        self.as_mut().difference(that);
    }
}
impl<A, B> SymmetricDifference<B> for Box<A>
where
    A: ?Sized + SymmetricDifference<B>,
    B: ?Sized,
{
    #[inline]
    fn symmetric_difference(&mut self, that: &B) {
        self.as_mut().symmetric_difference(that);
    }
}

impl<'a, 'b, A, B> Intersection<Cow<'b, B>> for Cow<'a, A>
where
    A: ?Sized + ToOwned,
    B: ?Sized + ToOwned,
    A::Owned: Intersection<B>,
{
    #[inline]
    fn intersection(&mut self, that: &Cow<'b, B>) {
        self.to_mut().intersection(that.as_ref());
    }
}

impl<'a, 'b, A, B> Union<Cow<'b, B>> for Cow<'a, A>
where
    A: ?Sized + ToOwned,
    B: ?Sized + ToOwned,
    A::Owned: Union<B>,
{
    #[inline]
    fn union(&mut self, that: &Cow<'b, B>) {
        self.to_mut().union(that.as_ref());
    }
}

impl<'a, 'b, A, B> Difference<Cow<'b, B>> for Cow<'a, A>
where
    A: ?Sized + ToOwned,
    B: ?Sized + ToOwned,
    A::Owned: Difference<B>,
{
    #[inline]
    fn difference(&mut self, that: &Cow<'b, B>) {
        self.to_mut().difference(that.as_ref());
    }
}

impl<'a, 'b, A, B> SymmetricDifference<Cow<'b, B>> for Cow<'a, A>
where
    A: ?Sized + ToOwned,
    B: ?Sized + ToOwned,
    A::Owned: SymmetricDifference<B>,
{
    #[inline]
    fn symmetric_difference(&mut self, that: &Cow<'b, B>) {
        self.to_mut().symmetric_difference(that.as_ref());
    }
}

// impl<'a, 'b, T, U> Assign<Cow<'b, U>> for Cow<'a, T>
// where
//     T: ?Sized + ToOwned,
//     U: ?Sized + ToOwned,
//     T::Owned: Assign<U>,
// {
//     #[inline]
//     fn and(this: &mut Self, that: &Cow<'b, U>) {
//         <T::Owned as Assign<U>>::and(this.to_mut(), that.as_ref())
//     }
//     #[inline]
//     fn not(this: &mut Self, that: &Cow<'b, U>) {
//         <T::Owned as Assign<U>>::not(this.to_mut(), that.as_ref())
//     }
//     #[inline]
//     fn or(this: &mut Self, that: &Cow<'b, U>) {
//         <T::Owned as Assign<U>>::or(this.to_mut(), that.as_ref())
//     }
//     #[inline]
//     fn xor(this: &mut Self, that: &Cow<'b, U>) {
//         <T::Owned as Assign<U>>::xor(this.to_mut(), that.as_ref())
//     }
// }
