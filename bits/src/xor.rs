use crate::{block::IntoBlocks, index};
use core::{
    cmp::Ordering::*,
    iter::{Fuse, Peekable},
};

pub trait Xor: Sized + IntoBlocks {
    fn xor<That: IntoBlocks>(self, that: That) -> BitwiseXor<Self, That>;
}

pub trait XorAssign<That: ?Sized> {
    fn xor_assign(a: &mut Self, b: &That);
}

pub struct BitwiseXor<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct SymmetricDifference<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
}

impl<T: IntoBlocks> Xor for T {
    #[inline]
    fn xor<That: IntoBlocks>(self, that: That) -> BitwiseXor<Self, That> {
        BitwiseXor { a: self, b: that }
    }
}

macro_rules! impl_xor_assign_for_words {
    ($( $Word:ty )*) => ($(
        impl XorAssign<$Word> for $Word {
            #[inline]
            fn xor_assign(a: &mut Self, b: &$Word) {
                *a ^= *b;
            }
        }
    )*)
}
impl_xor_assign_for_words!(u8 u16 u32 u64 u128);

impl<A: XorAssign<B>, B> XorAssign<[B]> for [A] {
    fn xor_assign(this: &mut Self, that: &[B]) {
        assert_eq!(this.len(), that.len());
        for (v1, v2) in this.iter_mut().zip(that) {
            XorAssign::xor_assign(v1, v2);
        }
    }
}

impl<A, B: ?Sized, const N: usize> XorAssign<B> for [A; N]
where
    [A]: XorAssign<B>,
{
    #[inline]
    fn xor_assign(this: &mut Self, that: &B) {
        <[A] as XorAssign<B>>::xor_assign(this.as_mut(), that)
    }
}

// impl<A: Bits, B: Bits> Bits for Xor<A, B> {
//     /// This could be an incorrect value, different from the consumed result.
//     #[inline]
//     fn len(this: &Self) -> usize {
//         cmp::max(Bits::len(&this.a), Bits::len(&this.b))
//     }

//     #[inline]
//     fn test(this: &Self, i: usize) -> bool {
//         Bits::test(&this.a, i) ^ Bits::test(&this.b, i)
//     }
// }

impl<A, B> IntoIterator for BitwiseXor<A, B>
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

impl<A: IntoBlocks, B: IntoBlocks<Block = A::Block>> IntoBlocks for BitwiseXor<A, B>
where
    A::Block: XorAssign<B::Block>,
{
    type Block = A::Block;
    type Blocks = SymmetricDifference<A::Blocks, B::Blocks>;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        SymmetricDifference {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
        }
    }
}

impl<A, B, S> Iterator for SymmetricDifference<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: XorAssign<S>,
{
    type Item = (usize, S);
    fn next(&mut self) -> Option<Self::Item> {
        let a = &mut self.a;
        let b = &mut self.b;
        match index::compare(a.peek(), b.peek(), Greater, Less) {
            Less => a.next(),
            Equal => {
                let (i, mut l) = a.next().expect("unreachable");
                let (j, r) = b.next().expect("unreachable");
                debug_assert_eq!(i, j);
                XorAssign::xor_assign(&mut l, &r);
                Some((i, l))
            }
            Greater => b.next(),
        }
    }
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<A, B: ?Sized> XorAssign<B> for Vec<A>
    where
        [A]: XorAssign<B>,
    {
        #[inline]
        fn xor_assign(this: &mut Self, that: &B) {
            <[A] as XorAssign<B>>::xor_assign(this.as_mut(), that)
        }
    }

    impl<T, U> XorAssign<U> for Box<T>
    where
        T: ?Sized + XorAssign<U>,
        U: ?Sized,
    {
        #[inline]
        fn xor_assign(this: &mut Self, that: &U) {
            <T as XorAssign<U>>::xor_assign(this, that)
        }
    }

    impl<'a, 'b, T, U> XorAssign<Cow<'b, U>> for Cow<'a, T>
    where
        T: ?Sized + ToOwned,
        U: ?Sized + ToOwned,
        T::Owned: XorAssign<U>,
    {
        #[inline]
        fn xor_assign(this: &mut Self, that: &Cow<'b, U>) {
            <T::Owned as XorAssign<U>>::xor_assign(this.to_mut(), that.as_ref())
        }
    }
}
