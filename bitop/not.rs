use core::cmp::Ordering::*;
use core::iter::{
    Fuse,
    Peekable,
};

use crate::{
    Difference,
    IntoMask,
    compare,
};

/// A and not B.
pub struct Not<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct NotMask<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
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

impl<A, B> IntoIterator for Not<A, B>
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

impl<A: IntoMask, B: IntoMask> IntoMask for Not<A, B>
where
    A::Bits: Difference<B::Bits>,
{
    type Bits = A::Bits;
    type Mask = NotMask<A::Mask, B::Mask>;
    #[inline]
    fn into_mask(self) -> Self::Mask {
        NotMask { a: self.a.into_mask().fuse().peekable(), b: self.b.into_mask().fuse().peekable() }
    }
}

impl<A, B, S1, S2> Iterator for NotMask<A, B>
where
    A: Iterator<Item = (usize, S1)>,
    B: Iterator<Item = (usize, S2)>,
    S1: Difference<S2>,
{
    type Item = (usize, S1);
    fn next(&mut self) -> Option<Self::Item> {
        let a = &mut self.a;
        let b = &mut self.b;
        loop {
            match compare(a.peek(), b.peek(), Less, Less) {
                Less => return a.next(),
                Equal => {
                    let (i, mut s1) = a.next().expect("unreachable");
                    let (j, s2) = b.next().expect("unreachable");
                    debug_assert_eq!(i, j);
                    s1.difference(&s2);
                    return Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            };
        }
    }
}
