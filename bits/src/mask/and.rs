use crate::mask::Mask;
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

pub struct And<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub trait AndAssign<That: ?Sized> {
    fn and_assign(a: &mut Self, b: &That);
}

pub struct Intersection<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

macro_rules! ints_impl_and_assign {
    ($( $Word:ty )*) => ($(
        impl AndAssign<$Word> for $Word {
            #[inline]
            fn and_assign(a: &mut Self, b: &$Word) {
                *a &= *b;
            }
        }
    )*)
}
ints_impl_and_assign!(u8 u16 u32 u64 u128);

impl<A, B> AndAssign<[B]> for [A]
where
    A: AndAssign<B>,
{
    fn and_assign(this: &mut Self, that: &[B]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            AndAssign::and_assign(v1, v2);
        }
    }
}

impl<A, B: ?Sized, const N: usize> AndAssign<B> for [A; N]
where
    [A]: AndAssign<B>,
{
    #[inline]
    fn and_assign(this: &mut Self, that: &B) {
        <[A] as AndAssign<B>>::and_assign(this.as_mut(), that)
    }
}

// impl<A: Bits, B: Bits> Bits for And<A, B> {
//     /// This could be an incorrect value, different from the consumed result.
//     #[inline]
//     fn len(this: &Self) -> usize {
//         cmp::min(Bits::len(&this.a), Bits::len(&this.b))
//     }
//     #[inline]
//     fn test(this: &Self, i: usize) -> bool {
//         Bits::test(&this.a, i) && Bits::test(&this.b, i)
//     }
// }

impl<A, B> IntoIterator for And<A, B>
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

impl<A: Mask, B: Mask> Mask for And<A, B>
where
    A::Bits: AndAssign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = Intersection<A::Iter, B::Iter>;
    fn into_mask(self) -> Self::Iter {
        Intersection {
            a: self.a.into_mask().fuse().peekable(),
            b: self.b.into_mask().fuse().peekable(),
        }
    }
}

impl<A, B, T, U> Iterator for Intersection<A, B>
where
    A: Iterator<Item = (usize, T)>,
    B: Iterator<Item = (usize, U)>,
    T: AndAssign<U>,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        // let Intersection { mut a, mut b } = self;
        let a = &mut self.a;
        let b = &mut self.b;
        loop {
            match Ord::cmp(&a.peek()?.0, &b.peek()?.0) {
                Less => {
                    a.next();
                }
                Equal => {
                    let (i, mut s1) = a.next().expect("unreachable");
                    let (j, s2) = b.next().expect("unreachable");
                    debug_assert_eq!(i, j);
                    AndAssign::and_assign(&mut s1, &s2);
                    break Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            }
        }
    }
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<A, B: ?Sized> AndAssign<B> for Vec<A>
    where
        [A]: AndAssign<B>,
    {
        #[inline]
        fn and_assign(this: &mut Self, that: &B) {
            <[A] as AndAssign<B>>::and_assign(this.as_mut(), that)
        }
    }

    impl<T, U> AndAssign<U> for Box<T>
    where
        T: ?Sized + AndAssign<U>,
        U: ?Sized,
    {
        #[inline]
        fn and_assign(this: &mut Self, that: &U) {
            <T as AndAssign<U>>::and_assign(this, that)
        }
    }

    impl<'a, 'b, T, U> AndAssign<Cow<'b, U>> for Cow<'a, T>
    where
        T: ?Sized + ToOwned,
        U: ?Sized + ToOwned,
        T::Owned: AndAssign<U>,
    {
        #[inline]
        fn and_assign(this: &mut Self, that: &Cow<'b, U>) {
            <T::Owned as AndAssign<U>>::and_assign(this.to_mut(), that.as_ref())
        }
    }
}
