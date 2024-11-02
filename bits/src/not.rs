use crate::{index, mask::Mask};
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

pub trait Not: Sized + Mask {
    fn not<That: Mask>(self, that: That) -> BitwiseNot<Self, That>;
}

pub trait NotAssign<That: ?Sized> {
    fn not_assign(a: &mut Self, b: &That);
}

pub struct BitwiseNot<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Difference<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<T: Mask> Not for T {
    #[inline]
    fn not<That: Mask>(self, that: That) -> BitwiseNot<Self, That> {
        BitwiseNot { a: self, b: that }
    }
}

macro_rules! ints_impl_not_assign {
    ($( $Word:ty )*) => ($(
        impl NotAssign<$Word> for $Word {
            #[inline]
            fn not_assign(a: &mut Self, b: &$Word) {
                *a &= !*b;
            }
        }
    )*)
}
ints_impl_not_assign!(u8 u16 u32 u64 u128);

impl<A: NotAssign<B>, B> NotAssign<[B]> for [A] {
    fn not_assign(this: &mut Self, that: &[B]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            NotAssign::not_assign(v1, v2);
        }
    }
}

impl<A, B: ?Sized, const N: usize> NotAssign<B> for [A; N]
where
    [A]: NotAssign<B>,
{
    #[inline]
    fn not_assign(this: &mut Self, that: &B) {
        <[A] as NotAssign<B>>::not_assign(this.as_mut(), that)
    }
}

// impl<A: Bits, B: Bits> Bits for AndNot<A, B> {
//     #[inline]
//     fn len(this: &Self) -> usize {
//         Bits::len(&this.a)
//     }
//     #[inline]
//     fn test(this: &Self, i: usize) -> bool {
//         Bits::test(&this.a, i) & !Bits::test(&this.b, i)
//     }
// }

impl<A, B> IntoIterator for BitwiseNot<A, B>
where
    Self: Mask,
{
    type Item = (usize, <Self as Mask>::Bits);
    type IntoIter = <Self as Mask>::Iter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_mask()
    }
}

impl<A: Mask, B: Mask> Mask for BitwiseNot<A, B>
where
    A::Bits: NotAssign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = Difference<A::Iter, B::Iter>;
    #[inline]
    fn into_mask(self) -> Self::Iter {
        Difference {
            a: self.a.into_mask().fuse().peekable(),
            b: self.b.into_mask().fuse().peekable(),
        }
    }
}

impl<A, B, S1, S2> Iterator for Difference<A, B>
where
    A: Iterator<Item = (usize, S1)>,
    B: Iterator<Item = (usize, S2)>,
    S1: NotAssign<S2>,
{
    type Item = (usize, S1);
    fn next(&mut self) -> Option<Self::Item> {
        let a = &mut self.a;
        let b = &mut self.b;
        loop {
            match index::compare(a.peek(), b.peek(), Less, Less) {
                Less => return a.next(),
                Equal => {
                    let (i, mut s1) = a.next().expect("unreachable");
                    let (j, s2) = b.next().expect("unreachable");
                    debug_assert_eq!(i, j);
                    NotAssign::not_assign(&mut s1, &s2);
                    return Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            };
        }
    }
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<A, B: ?Sized> NotAssign<B> for Vec<A>
    where
        [A]: NotAssign<B>,
    {
        #[inline]
        fn not_assign(this: &mut Self, that: &B) {
            <[A] as NotAssign<B>>::not_assign(this.as_mut(), that)
        }
    }

    impl<T, U> NotAssign<U> for Box<T>
    where
        T: ?Sized + NotAssign<U>,
        U: ?Sized,
    {
        #[inline]
        fn not_assign(this: &mut Self, that: &U) {
            <T as NotAssign<U>>::not_assign(this, that)
        }
    }

    impl<'a, 'b, T, U> NotAssign<Cow<'b, U>> for Cow<'a, T>
    where
        T: ?Sized + ToOwned,
        U: ?Sized + ToOwned,
        T::Owned: NotAssign<U>,
    {
        #[inline]
        fn not_assign(this: &mut Self, that: &Cow<'b, U>) {
            <T::Owned as NotAssign<U>>::not_assign(this.to_mut(), that.as_ref())
        }
    }
}
