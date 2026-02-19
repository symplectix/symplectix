use core::cmp::Ordering::*;
use core::iter::{
    Fuse,
    Peekable,
};

use crate::{
    Block,
    IntoBlocks,
    Masking,
};

/// A and B.
pub struct And<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct AndMask<A: Iterator, B: Iterator> {
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
    Self: IntoBlocks,
{
    type Item = (usize, <Self as IntoBlocks>::Block);
    type IntoIter = <Self as IntoBlocks>::Blocks;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_blocks()
    }
}

impl<A: IntoBlocks, B: IntoBlocks> IntoBlocks for And<A, B>
where
    A::Block: Block + Masking<B::Block>,
{
    type Block = A::Block;
    type Blocks = AndMask<A::Blocks, B::Blocks>;
    fn into_blocks(self) -> Self::Blocks {
        AndMask {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
        }
    }
}

impl<A, B, T, U> Iterator for AndMask<A, B>
where
    A: Iterator<Item = (usize, T)>,
    B: Iterator<Item = (usize, U)>,
    T: Block + Masking<U>,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
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
                    Masking::and(&mut s1, &s2);
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
