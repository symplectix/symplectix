use core::cmp::Ordering::*;
use core::iter::{
    Fuse,
    Peekable,
};

use crate::{
    IntoBlocks,
    Masking,
    compare,
};

/// A or B.
pub struct Or<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

pub struct OrMask<A: Iterator, B: Iterator> {
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
    Self: IntoBlocks,
{
    type Item = (usize, <Self as IntoBlocks>::Block);
    type IntoIter = <Self as IntoBlocks>::Blocks;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_blocks()
    }
}

impl<A: IntoBlocks, B: IntoBlocks<Block = A::Block>> IntoBlocks for Or<A, B>
where
    A::Block: Masking<B::Block>,
{
    type Block = A::Block;
    type Blocks = OrMask<A::Blocks, B::Blocks>;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        OrMask {
            a: self.a.into_blocks().fuse().peekable(),
            b: self.b.into_blocks().fuse().peekable(),
        }
    }
}

impl<A, B, S> Iterator for OrMask<A, B>
where
    A: Iterator<Item = (usize, S)>,
    B: Iterator<Item = (usize, S)>,
    S: Masking<S>,
{
    type Item = (usize, S);
    fn next(&mut self) -> Option<Self::Item> {
        let x = &mut self.a;
        let y = &mut self.b;
        match compare(x.peek(), y.peek(), Greater, Less) {
            Less => x.next(),
            Equal => {
                let (i, mut l) = x.next().expect("unreachable");
                let (j, r) = y.next().expect("unreachable");
                debug_assert_eq!(i, j);
                Masking::or(&mut l, &r);
                Some((i, l))
            }
            Greater => y.next(),
        }
    }
}
