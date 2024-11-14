use core::cmp::Ordering::*;
use core::iter::{Fuse, Peekable};

use super::{compare, helper, Mask};

pub struct Not<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Difference<A: Iterator, B: Iterator> {
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
    Self: Mask,
{
    type Item = (usize, <Self as Mask>::Bits);
    type IntoIter = <Self as Mask>::Iter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_mask()
    }
}

impl<A: Mask, B: Mask> Mask for Not<A, B>
where
    A::Bits: helper::Assign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = Difference<A::Iter, B::Iter>;
    #[inline]
    fn into_mask(self) -> Self::Iter {
        Difference { a: self.a.into_mask().fuse().peekable(), b: self.b.into_mask().fuse().peekable() }
    }
}

impl<A, B, S1, S2> Iterator for Difference<A, B>
where
    A: Iterator<Item = (usize, S1)>,
    B: Iterator<Item = (usize, S2)>,
    S1: helper::Assign<S2>,
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
                    helper::Assign::not(&mut s1, &s2);
                    return Some((i, s1));
                }
                Greater => {
                    b.next();
                }
            };
        }
    }
}
