use std::ops::RangeBounds;

use crate::bits::Bits;
use crate::block::{Block, Count};

pub trait Rank: Count {
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let r = bit::bounded(&r, 0, Self::BITS);
        r.len() - self.rank0(r)
    }

    #[inline]
    fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let r = bit::bounded(&r, 0, Self::BITS);
        r.len() - self.rank1(r)
    }
}

impl<B: Copy + Block + Rank, const N: usize> Rank for [B; N] {
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        Bits::new(self.as_slice()).rank1(r)
    }
    #[inline]
    fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
        Bits::new(self.as_slice()).rank1(r)
    }
}

impl<B: Block + Rank> Rank for Box<B> {
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        self.as_ref().rank1(r)
    }
    #[inline]
    fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
        self.as_ref().rank0(r)
    }
}
