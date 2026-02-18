use core::cmp::Ordering::*;
use core::iter::{
    Fuse,
    Peekable,
};

use crate::{
    IntoMask,
    SymmetricDifference,
    compare,
};

/// A xor B.
pub struct Xor<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct XorMask<A: Iterator, B: Iterator> {
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
    Self: IntoMask,
{
    type Item = (usize, <Self as IntoMask>::Bits);
    type IntoIter = <Self as IntoMask>::Mask;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_mask()
    }
}

impl<A: IntoMask, B: IntoMask<Bits = A::Bits>> IntoMask for Xor<A, B>
where
    A::Bits: SymmetricDifference<B::Bits>,
{
    type Bits = A::Bits;
    type Mask = XorMask<A::Mask, B::Mask>;
    #[inline]
    fn into_mask(self) -> Self::Mask {
        XorMask { a: self.a.into_mask().fuse().peekable(), b: self.b.into_mask().fuse().peekable() }
    }
}

impl<A, B, S> Iterator for XorMask<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: SymmetricDifference<S>,
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
                l.symmetric_difference(&r);
                Some((i, l))
            }
            Greater => b.next(),
        }
    }
}
