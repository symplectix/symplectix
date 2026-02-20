//! Provides bitset operations.

use std::borrow::{
    Cow,
    ToOwned,
};
use std::cmp::Ordering;

mod difference;
mod intersection;
mod symmetric_difference;
mod union;

use bits::{
    Buf,
    IntoBlocks,
};
pub use difference::Difference;
pub use intersection::Intersection;
pub use symmetric_difference::SymmetricDifference;
pub use union::Union;

/// Provides bitset operations.
pub trait Mask {
    /// Return the intersection of two sets as an iterator of blocks.
    ///
    /// The intersection of two sets is the set containing
    /// all elements of A that also belong to B or equivalently,
    /// all elements of B that also belong to A.
    fn intersection<'a, That>(&'a self, that: That) -> Intersection<&'a Self, That>
    where
        Intersection<&'a Self, That>: IntoBlocks,
    {
        Intersection { a: self, b: that }
    }

    /// Returns the union of two sets as an iterator of blocks.
    ///
    /// The union of two sets is the set of all elements
    /// in the both of the sets.
    fn union<'a, That>(&'a self, that: That) -> Union<&'a Self, That>
    where
        Union<&'a Self, That>: IntoBlocks,
    {
        Union { a: self, b: that }
    }

    /// Returns the difference of two sets as an iterator of blocks.
    ///
    /// The difference, or subtraction is the set that consists of
    /// elements that are in A but not in B.
    fn difference<'a, That>(&'a self, that: That) -> Difference<&'a Self, That>
    where
        Difference<&'a Self, That>: IntoBlocks,
    {
        Difference { a: self, b: that }
    }

    /// Returns the symmetric difference of two sets as an iterator of blocks.
    ///
    /// The symmetric difference of two sets is the set of elements
    /// which are in either of the sets, but not in their intersection.
    fn symmetric_difference<'a, That>(&'a self, that: That) -> SymmetricDifference<&'a Self, That>
    where
        SymmetricDifference<&'a Self, That>: IntoBlocks,
    {
        SymmetricDifference { a: self, b: that }
    }

    // TODO
    // fn is_disjoint(...) -> ...
    // fn is_subset(...) -> ...
    // fn is_superset(...) -> ...
}

impl<'a, T> Mask for T
where
    T: 'a,
    &'a T: IntoBlocks,
{
}

/// Helper trait for bit masking.
///
/// The mask defines which bits to retain and which to clear.
/// Masking involves applying such a mask to self.
pub trait Masking<Mask: ?Sized = Self> {
    /// Performs inplace and.
    fn and(data: &mut Self, mask: &Mask);

    /// Performs inplace or.
    fn or(data: &mut Self, mask: &Mask);

    /// Performs inplace not.
    fn not(data: &mut Self, mask: &Mask);

    /// Performs inplace xor.
    fn xor(data: &mut Self, mask: &Mask);
}

pub(crate) fn compare<X, Y>(
    x: Option<&(usize, X)>,
    y: Option<&(usize, Y)>,
    when_x_is_none: Ordering,
    when_y_is_none: Ordering,
) -> Ordering {
    match (x, y) {
        (None, _) => when_x_is_none,
        (_, None) => when_y_is_none,
        (Some((i, _x)), Some((j, _y))) => i.cmp(j),
    }
}

macro_rules! buf_impls {
    ($( $Ty:ty ),*) => ($(
        impl<const N: usize> Masking<Self> for Buf<[$Ty; N]> {
            fn and(data: &mut Self, that: &Self) {
                match (data.as_mut(), that.as_ref()) {
                    (Some(this), Some(that)) => {
                        for (a, b) in this.iter_mut().zip(that) {
                            *a &= *b;
                        }
                    }
                    (Some(_), None) => {
                        *data = Buf::new();
                    }
                    _ => {}
                }
            }

            fn or(data: &mut Self, that: &Self) {
                match (data.as_mut(), that.as_ref()) {
                    (Some(this), Some(that)) => {
                        for (a, b) in this.iter_mut().zip(that) {
                            *a |= *b;
                        }
                    }
                    (None, Some(that)) => {
                        data.or_empty().copy_from_slice(that);
                    }
                    _ => {}
                }
            }

            fn not(data: &mut Self, that: &Self) {
                if let (Some(this), Some(that)) = (data.as_mut(), that.as_ref()) {
                    for (a, b) in this.iter_mut().zip(that) {
                        *a &= !*b;
                    }
                }
            }

            fn xor(data: &mut Self, that: &Self) {
                match (data.as_mut(), that.as_ref()) {
                    (Some(this), Some(that)) => {
                        for (a, b) in this.iter_mut().zip(that) {
                            *a ^= *b;
                        }
                    }
                    (None, Some(that)) => {
                        data.or_empty().copy_from_slice(that);
                    }
                    _ => {}
                }
            }
        }
    )*)
}
buf_impls!(u8, u16, u32, u64, u128, usize);

impl<A, B> Masking<B> for Box<A>
where
    A: ?Sized + Masking<B>,
    B: ?Sized,
{
    #[inline]
    fn and(data: &mut Self, mask: &B) {
        Masking::and(data.as_mut(), mask);
    }
    #[inline]
    fn or(data: &mut Self, mask: &B) {
        Masking::or(data.as_mut(), mask);
    }
    #[inline]
    fn not(data: &mut Self, mask: &B) {
        Masking::not(data.as_mut(), mask);
    }
    #[inline]
    fn xor(data: &mut Self, mask: &B) {
        Masking::xor(data.as_mut(), mask);
    }
}

impl<'a, 'b, A, B> Masking<Cow<'b, B>> for Cow<'a, A>
where
    A: ?Sized + ToOwned,
    B: ?Sized + ToOwned,
    A::Owned: Masking<B>,
{
    #[inline]
    fn and(data: &mut Self, mask: &Cow<'b, B>) {
        Masking::and(data.to_mut(), mask);
    }
    #[inline]
    fn or(data: &mut Self, mask: &Cow<'b, B>) {
        Masking::or(data.to_mut(), mask);
    }
    #[inline]
    fn not(data: &mut Self, mask: &Cow<'b, B>) {
        Masking::not(data.to_mut(), mask);
    }
    #[inline]
    fn xor(data: &mut Self, mask: &Cow<'b, B>) {
        Masking::xor(data.to_mut(), mask);
    }
}

#[cfg(test)]
mod mask_test {
    use std::borrow::Cow;

    use crate::Mask;

    // For testing purposes only. Wrapping integers in a Cow is
    // a waste of space.
    macro_rules! impl_masking_for_word {
        ($( $Word:ty )*) => ($(
            impl crate::Masking<$Word> for $Word {
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
        let mut iter = a.intersection(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000100)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b01000000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn union() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.union(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000101)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b11100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b01100000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn difference() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.difference(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000001)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b01100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b00100000)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn symmetric_difference() {
        let a: Vec<u64> = vec![0b00000101, 0b01100011, 0b01100000];
        let b: Vec<u64> = vec![0b00000100, 0b10000000, 0b01000000];
        let mut iter = a.symmetric_difference(&b).into_iter();
        assert_eq!(iter.next().unwrap(), (0, Cow::Owned(0b00000001)));
        assert_eq!(iter.next().unwrap(), (1, Cow::Owned(0b11100011)));
        assert_eq!(iter.next().unwrap(), (2, Cow::Owned(0b00100000)));
        assert_eq!(iter.next(), None);
    }
}
