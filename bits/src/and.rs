use crate::block::IntoBlocks;
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

pub trait And: Sized + IntoBlocks {
    fn and<That: IntoBlocks>(self, that: That) -> BitwiseAnd<Self, That>;
}

pub trait AndAssign<That: ?Sized> {
    fn and_assign(a: &mut Self, b: &That);
}

pub struct BitwiseAnd<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Intersection<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<T: IntoBlocks> And for T {
    #[inline]
    fn and<That: IntoBlocks>(self, that: That) -> BitwiseAnd<Self, That> {
        BitwiseAnd { a: self, b: that }
    }
}

macro_rules! impl_and_assign_for_words {
    ($( $Word:ty )*) => ($(
        impl AndAssign<$Word> for $Word {
            #[inline]
            fn and_assign(a: &mut Self, b: &$Word) {
                *a &= *b;
            }
        }
    )*)
}
impl_and_assign_for_words!(u8 u16 u32 u64 u128);

impl<T, U> AndAssign<[U]> for [T]
where
    T: AndAssign<U>,
{
    fn and_assign(this: &mut Self, that: &[U]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            AndAssign::and_assign(v1, v2);
        }
    }
}

impl<T, U: ?Sized, const N: usize> AndAssign<U> for [T; N]
where
    [T]: AndAssign<U>,
{
    #[inline]
    fn and_assign(this: &mut Self, that: &U) {
        <[T] as AndAssign<U>>::and_assign(this.as_mut(), that)
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

impl<A, B> IntoIterator for BitwiseAnd<A, B>
where
    Self: IntoBlocks,
{
    type Item = (usize, <Self as IntoBlocks>::Block);
    type IntoIter = <Self as IntoBlocks>::Blocks;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_blocks()
    }
}

impl<A: IntoBlocks, B: IntoBlocks> IntoBlocks for BitwiseAnd<A, B>
where
    A::Block: AndAssign<B::Block>,
{
    type Block = A::Block;
    type Blocks = Intersection<A::Blocks, B::Blocks>;
    fn into_blocks(self) -> Self::Blocks {
        Intersection {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
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

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::borrow::{Cow, ToOwned};
    use alloc::boxed::Box;
    use alloc::vec::Vec;

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

    impl<T, U: ?Sized> AndAssign<U> for Vec<T>
    where
        [T]: AndAssign<U>,
    {
        #[inline]
        fn and_assign(this: &mut Self, that: &U) {
            <[T] as AndAssign<U>>::and_assign(this.as_mut(), that)
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
