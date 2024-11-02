use crate::{block::IntoBlocks, index};
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

pub trait Or: Sized + IntoBlocks {
    fn or<That: IntoBlocks>(self, that: That) -> BitwiseOr<Self, That>;
}

pub trait OrAssign<That: ?Sized> {
    fn or_assign(a: &mut Self, b: &That);
}

pub struct BitwiseOr<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Union<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<T: IntoBlocks> Or for T {
    #[inline]
    fn or<That: IntoBlocks>(self, that: That) -> BitwiseOr<Self, That> {
        BitwiseOr { a: self, b: that }
    }
}

macro_rules! impl_or_assign_for_words {
    ($( $Word:ty )*) => ($(
        impl OrAssign<$Word> for $Word {
            #[inline]
            fn or_assign(a: &mut Self, b: &$Word) {
                *a |= *b;
            }
        }
    )*)
}
impl_or_assign_for_words!(u8 u16 u32 u64 u128);

impl<A: OrAssign<B>, B> OrAssign<[B]> for [A] {
    fn or_assign(this: &mut Self, that: &[B]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            OrAssign::or_assign(v1, v2);
        }
    }
}

impl<A, B: ?Sized, const N: usize> OrAssign<B> for [A; N]
where
    [A]: OrAssign<B>,
{
    #[inline]
    fn or_assign(this: &mut Self, that: &B) {
        <[A] as OrAssign<B>>::or_assign(this.as_mut(), that)
    }
}

// impl<A: Bits, B: Bits> Bits for Or<A, B> {
//     /// This could be an incorrect value, different from the consumed result.
//     #[inline]
//     fn len(this: &Self) -> usize {
//         cmp::max(Bits::len(&this.a), Bits::len(&this.b))
//     }
//     #[inline]
//     fn test(this: &Self, i: usize) -> bool {
//         Bits::test(&this.a, i) || Bits::test(&this.b, i)
//     }
// }

impl<A, B> IntoIterator for BitwiseOr<A, B>
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

impl<A: IntoBlocks, B: IntoBlocks<Block = A::Block>> IntoBlocks for BitwiseOr<A, B>
where
    A::Block: OrAssign<B::Block>,
{
    type Block = A::Block;
    type Blocks = Union<A::Blocks, B::Blocks>;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        Union {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
        }
    }
}

impl<A, B, S> Iterator for Union<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: OrAssign<S>,
{
    type Item = (usize, S);
    fn next(&mut self) -> Option<Self::Item> {
        let x = &mut self.a;
        let y = &mut self.b;
        match index::compare(x.peek(), y.peek(), Greater, Less) {
            Less => x.next(),
            Equal => {
                let (i, mut l) = x.next().expect("unreachable");
                let (j, r) = y.next().expect("unreachable");
                debug_assert_eq!(i, j);
                OrAssign::or_assign(&mut l, &r);
                Some((i, l))
            }
            Greater => y.next(),
        }
    }
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<A, B: ?Sized> OrAssign<B> for Vec<A>
    where
        [A]: OrAssign<B>,
    {
        #[inline]
        fn or_assign(this: &mut Self, that: &B) {
            <[A] as OrAssign<B>>::or_assign(this.as_mut(), that)
        }
    }

    impl<T, U> OrAssign<U> for Box<T>
    where
        T: ?Sized + OrAssign<U>,
        U: ?Sized,
    {
        #[inline]
        fn or_assign(this: &mut Self, that: &U) {
            <T as OrAssign<U>>::or_assign(this, that)
        }
    }

    impl<'a, 'b, T, U> OrAssign<Cow<'b, U>> for Cow<'a, T>
    where
        T: ?Sized + ToOwned,
        U: ?Sized + ToOwned,
        T::Owned: OrAssign<U>,
    {
        #[inline]
        fn or_assign(this: &mut Self, that: &Cow<'b, U>) {
            <T::Owned as OrAssign<U>>::or_assign(this.to_mut(), that.as_ref())
        }
    }
}
