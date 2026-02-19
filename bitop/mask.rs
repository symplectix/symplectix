use std::borrow::{
    Cow,
    ToOwned,
};

use crate::Block;

/// Helper trait for bit masking.
///
/// The mask defines which bits to retain and which to clear.
/// Masking involves applying such a mask to self.
pub trait Masking<Mask: ?Sized = Self> {
    /// The intersection of two sets A and B is the set containing
    /// all elements of A that also belong to B or equivalently,
    /// all elements of B that also belong to A.
    fn intersection(&mut self, mask: &Mask);

    /// The union of two sets is the set of all elements
    /// in the both of the sets.
    fn union(&mut self, mask: &Mask);

    /// The difference, or subtraction is the set that consists of
    /// elements that are in A but not in B.
    fn difference(&mut self, mask: &Mask);

    /// The symmetric difference of two sets is the set of elements
    /// which are in either of the sets, but not in their intersection.
    fn symmetric_difference(&mut self, mask: &Mask);
}

macro_rules! impl_Assign_for_word {
    ($( $Word:ty )*) => ($(
        impl Masking<$Word> for $Word {
            #[inline]
            fn intersection(&mut self, that: &$Word) {
                *self &= *that;
            }
            #[inline]
            fn union(&mut self, that: &$Word) {
                *self |= *that;
            }
            #[inline]
            fn difference(&mut self, that: &$Word) {
                *self &= !*that;
            }
            #[inline]
            fn symmetric_difference(&mut self, that: &$Word) {
                *self ^= *that;
            }
        }
    )*)
}
impl_Assign_for_word!(u8 u16 u32 u64 u128);

impl<A, B> Masking<B> for Box<A>
where
    A: ?Sized + Masking<B>,
    B: ?Sized,
{
    #[inline]
    fn intersection(&mut self, that: &B) {
        self.as_mut().intersection(that);
    }
    #[inline]
    fn union(&mut self, that: &B) {
        self.as_mut().union(that);
    }
    #[inline]
    fn difference(&mut self, that: &B) {
        self.as_mut().difference(that);
    }
    #[inline]
    fn symmetric_difference(&mut self, that: &B) {
        self.as_mut().symmetric_difference(that);
    }
}

impl<'a, 'b, A, B> Masking<Cow<'b, B>> for Cow<'a, A>
where
    A: ?Sized + ToOwned,
    B: ?Sized + ToOwned,
    A::Owned: Masking<B>,
{
    #[inline]
    fn intersection(&mut self, that: &Cow<'b, B>) {
        self.to_mut().intersection(that.as_ref());
    }
    #[inline]
    fn union(&mut self, that: &Cow<'b, B>) {
        self.to_mut().union(that.as_ref());
    }
    #[inline]
    fn difference(&mut self, that: &Cow<'b, B>) {
        self.to_mut().difference(that.as_ref());
    }
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
