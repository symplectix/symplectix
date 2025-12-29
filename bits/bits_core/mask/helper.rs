use std::borrow::{
    Cow,
    ToOwned,
};
use std::boxed::Box;
use std::vec::Vec;

use crate::Bits;

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

impl<A, B> Assign<Bits<B>> for Bits<A>
where
    A: Assign<B>,
{
    fn and(this: &mut Self, that: &Bits<B>) {
        let this = this.as_mut_slice();
        let that = that.as_slice();

        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            Assign::and(v1, v2);
        }
    }

    fn not(this: &mut Self, that: &Bits<B>) {
        let this = this.as_mut_slice();
        let that = that.as_slice();

        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            Assign::not(v1, v2);
        }
    }

    fn or(this: &mut Self, that: &Bits<B>) {
        let this = this.as_mut_slice();
        let that = that.as_slice();

        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            Assign::or(v1, v2);
        }
    }

    fn xor(this: &mut Self, that: &Bits<B>) {
        let this = this.as_mut_slice();
        let that = that.as_slice();

        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            Assign::xor(v1, v2);
        }
    }
}

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
