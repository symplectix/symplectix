use core::cmp::Ordering::*;
use core::iter::{Fuse, Peekable};

use crate::{helper, Mask};

pub struct Or<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Union<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
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

impl<A, B> IntoIterator for Or<A, B>
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

impl<A: Mask, B: Mask<Bits = A::Bits>> Mask for Or<A, B>
where
    A::Bits: helper::Assign<B::Bits>,
{
    type Bits = A::Bits;
    type Iter = Union<A::Iter, B::Iter>;
    #[inline]
    fn into_mask(self) -> Self::Iter {
        Union { a: self.a.into_mask().fuse().peekable(), b: self.b.into_mask().fuse().peekable() }
    }
}

impl<A, B, S> Iterator for Union<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: helper::Assign<S>,
{
    type Item = (usize, S);
    fn next(&mut self) -> Option<Self::Item> {
        let x = &mut self.a;
        let y = &mut self.b;
        match crate::compare(x.peek(), y.peek(), Greater, Less) {
            Less => x.next(),
            Equal => {
                let (i, mut l) = x.next().expect("unreachable");
                let (j, r) = y.next().expect("unreachable");
                debug_assert_eq!(i, j);
                helper::Assign::or(&mut l, &r);
                Some((i, l))
            }
            Greater => y.next(),
        }
    }
}
