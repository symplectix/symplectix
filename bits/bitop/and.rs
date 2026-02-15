use core::cmp::Ordering::*;
use core::iter::{
    Fuse,
    Peekable,
};

use crate::{
    Assign,
    Block,
    IntoMask,
};

pub struct And<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct Intersection<A: Iterator, B: Iterator> {
    a: Peekable<Fuse<A>>,
    b: Peekable<Fuse<B>>,
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
    Self: IntoMask,
{
    type Item = (usize, <Self as IntoMask>::Bits);
    type IntoIter = <Self as IntoMask>::Mask;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_mask()
    }
}

impl<A: IntoMask, B: IntoMask> IntoMask for And<A, B>
where
    A::Bits: Block + Assign<B::Bits>,
{
    type Bits = A::Bits;
    type Mask = Intersection<A::Mask, B::Mask>;
    fn into_mask(self) -> Self::Mask {
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
    T: Block + Assign<U>,
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
                    Assign::and(&mut s1, &s2);
                    if s1.any() {
                        break Some((i, s1));
                    } else {
                        continue;
                    }
                }
                Greater => {
                    b.next();
                }
            }
        }
    }
}
