use std::ops::RangeBounds;

use crate::block::Rank;

pub trait Excess: Rank {
    fn excess1<R: RangeBounds<usize>>(&self, r: R) -> Option<usize>;
    fn excess0<R: RangeBounds<usize>>(&self, r: R) -> Option<usize>;
}

impl<T: Rank> Excess for T {
    #[inline]
    fn excess1<R: RangeBounds<usize>>(&self, r: R) -> Option<usize> {
        helper::ranks(self, r).excess1()
    }
    #[inline]
    fn excess0<R: RangeBounds<usize>>(&self, r: R) -> Option<usize> {
        helper::ranks(self, r).excess0()
    }
}

pub(crate) mod helper {
    use std::ops::RangeBounds;

    use crate::block::Rank;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Ranks {
        rank0: usize,
        rank1: usize,
    }

    /// Computes `rank0` and `rank1` at a time.
    pub(crate) fn ranks<T, R>(b: &T, r: R) -> Ranks
    where
        T: Rank,
        R: RangeBounds<usize>,
    {
        let r = bit::bounded(&r, 0, T::BITS);
        let len = r.len();
        let rank1 = b.rank1(r);
        let rank0 = len - rank1;
        Ranks { rank0, rank1 }
    }

    impl Ranks {
        #[inline]
        pub(crate) fn excess1(self) -> Option<usize> {
            let Ranks { rank0, rank1 } = self;
            rank1.checked_sub(rank0)
        }

        #[inline]
        pub(crate) fn excess0(self) -> Option<usize> {
            let Ranks { rank0, rank1 } = self;
            rank0.checked_sub(rank1)
        }
    }
}
