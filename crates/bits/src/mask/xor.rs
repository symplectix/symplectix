use core::cmp::Ordering::*;
use core::iter::{
    Fuse,
    Peekable,
};

use super::{
    Mask,
    compare,
    helper,
};

pub struct Xor<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct SymmetricDifference<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
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

impl<A, B> IntoIterator for Xor<A, B>
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

impl<A: Mask, B: Mask<Bits = A::Bits>> Mask for Xor<A, B>
where
    A::Bits: helper::Assign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = SymmetricDifference<A::Iter, B::Iter>;
    #[inline]
    fn into_mask(self) -> Self::Iter {
        SymmetricDifference {
            a: self.a.into_mask().fuse().peekable(),
            b: self.b.into_mask().fuse().peekable(),
        }
    }
}

impl<A, B, S> Iterator for SymmetricDifference<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: helper::Assign<S>,
{
    type Item = (usize, S);
    fn next(&mut self) -> Option<Self::Item> {
        let a = &mut self.a;
        let b = &mut self.b;
        match compare(a.peek(), b.peek(), Greater, Less) {
            Less => a.next(),
            Equal => {
                let (i, mut l) = a.next().expect("unreachable");
                let (j, r) = b.next().expect("unreachable");
                debug_assert_eq!(i, j);
                helper::Assign::xor(&mut l, &r);
                Some((i, l))
            }
            Greater => b.next(),
        }
    }
}
