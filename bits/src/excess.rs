use crate::Rank;
use core::ops::RangeBounds;

/// `Excess` extends `Rank` to count exceeded bits.
///
/// # Examples
///
/// ```
/// # use bits::Excess;
/// let v: &[u8] = &[0b_1111_0000, 0b_1111_1100];
/// assert_eq!(v.excess(..10), 2);
/// assert_eq!(v.excess1(..10), None);
/// assert_eq!(v.excess0(..10), Some(2));
///
/// assert_eq!(v.excess(..16), 4);
/// assert_eq!(v.excess1(..16), Some(4));
/// assert_eq!(v.excess0(..16), None);
/// ```
pub trait Excess: Rank {
    fn excess<R: RangeBounds<usize>>(&self, r: R) -> usize;

    fn excess1<R: RangeBounds<usize>>(&self, r: R) -> Option<usize>;

    fn excess0<R: RangeBounds<usize>>(&self, r: R) -> Option<usize>;
}

impl<T: ?Sized + Rank> Excess for T {
    #[inline]
    fn excess<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        ranks(self, index).excess()
    }

    #[inline]
    fn excess1<Index: RangeBounds<usize>>(&self, index: Index) -> Option<usize> {
        ranks(self, index).excess1()
    }

    #[inline]
    fn excess0<Index: RangeBounds<usize>>(&self, index: Index) -> Option<usize> {
        ranks(self, index).excess0()
    }
}

/// Computes `rank0` and `rank1` at a time.
fn ranks<T, Index>(bits: &T, index: Index) -> Ranks
where
    T: ?Sized + Rank,
    Index: RangeBounds<usize>,
{
    let (i, j) = crate::to_range(&index, 0, bits.bits());
    let rank1 = bits.rank1(i..j);
    let rank0 = (j - i) - rank1;
    Ranks { rank0, rank1 }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Ranks {
    rank0: usize,
    rank1: usize,
}

impl Ranks {
    #[inline]
    fn excess(self) -> usize {
        let Ranks { rank0, rank1 } = self;
        rank0.abs_diff(rank1)
    }

    #[inline]
    fn excess1(self) -> Option<usize> {
        let Ranks { rank0, rank1 } = self;
        rank1.checked_sub(rank0)
    }

    #[inline]
    fn excess0(self) -> Option<usize> {
        let Ranks { rank0, rank1 } = self;
        rank0.checked_sub(rank1)
    }
}
