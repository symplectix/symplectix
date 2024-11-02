use crate::index;
use crate::{Bits, Container, Count};
use core::ops::{Range, RangeBounds};

pub trait Rank: Count {
    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn rank1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let r = index::to_range(&index, 0, self.bits());
        r.len() - self.rank0(r)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let r = index::to_range(&index, 0, self.bits());
        r.len() - self.rank1(r)
    }
}

macro_rules! ints_impl_rank {
    ($( $Int:ty )*) => ($(
        impl Rank for $Int {
            #[inline]
            fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
                let Range { start: i, end: j } = index::to_range(&r, 0, self.bits());
                (*self & mask!($Int, i, j)).count1()
            }

            #[inline]
            fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
                (!*self).rank1(r)
            }
        }

    )*)
}
ints_impl_rank!(u8 u16 u32 u64 u128 usize);
ints_impl_rank!(i8 i16 i32 i64 i128 isize);

impl<B: Bits> Rank for [B] {
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let Range { start, end } = index::to_range(&r, 0, self.bits());

        index::between::<B>(start, end)
            .map(|(i, r)| {
                self.get(i).map_or(0, |b| if r.len() == B::BITS { b.count1() } else { b.rank1(r) })
            })
            .sum()

        // if self.is_empty() {
        //     return 0;
        // }
        // let (i, p) = index::address::<B>(s);
        // let (j, q) = index::address::<B>(e);
        // if i == j {
        //     self[i].rank1(p..q)
        // } else {
        //     self[i].rank1(p..) + self[i + 1..j].count1() + self.get(j).map_or(0, |b| b.rank1(..q))
        // }
    }
}

macro_rules! impl_rank {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Rank>::rank1(self$(.$method())?, r)
        }

        #[inline]
        fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
            <$X as Rank>::rank0(self$(.$method())?, r)
        }
    }
}

impl<'a, T: ?Sized + Rank> Rank for &'a T {
    impl_rank!(T);
}

impl<B, const N: usize> Rank for [B; N]
where
    [B]: Rank,
{
    impl_rank!([B], as_ref);
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<B> Rank for Vec<B>
    where
        [B]: Rank,
    {
        impl_rank!([B]);
    }

    impl<T: ?Sized + Rank> Rank for Box<T> {
        impl_rank!(T);
    }

    impl<'a, T> Rank for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Rank,
    {
        impl_rank!(T, as_ref);
    }
}
