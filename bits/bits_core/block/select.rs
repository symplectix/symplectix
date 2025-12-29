use crate::bits::Bits;
use crate::block::{
    Block,
    Rank,
};

pub trait Select: Rank {
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        helper::search1(self, n)
    }

    // #[inline]
    // fn select1_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select1(self.rank1(..i) + n).map(|pos| pos - i)
    // }

    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        helper::search0(self, n)
    }

    // #[inline]
    // fn select0_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select0(self.rank0(..i) + n).map(|pos| pos - i)
    // }
}

mod helper {
    use crate::block::Rank;

    /// Binary search to find and return the smallest index k in `[i, j)` at which f(k) is true,
    /// assuming that on the range `[i, j)`, f(k) == true implies f(k+1) == true.
    ///
    /// Returns the first true index, if there is no such index, returns `j`.
    fn binary_search(mut l: usize, mut r: usize, p: impl Fn(usize) -> bool) -> usize {
        while l < r {
            let m = l + (r - l) / 2;
            if p(m) {
                r = m; // -> f(r) == true
            } else {
                l = m + 1; // -> f(l-1) == false
            }
        }
        l // f(l-1) == false && f(l) (= f(r)) == true
    }

    #[inline]
    pub(crate) fn search1<T>(bs: &T, n: usize) -> Option<usize>
    where
        T: Rank,
    {
        (n < bs.count1()).then(|| binary_search(0, T::BITS, |k| bs.rank1(..k) > n) - 1)
    }

    #[inline]
    pub(crate) fn search0<T>(bs: &T, n: usize) -> Option<usize>
    where
        T: Rank,
    {
        (n < bs.count0()).then(|| binary_search(0, T::BITS, |k| bs.rank0(..k) > n) - 1)
    }
}

impl<B: Copy + Block + Select, const N: usize> Select for [B; N] {
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        Bits::new(self.as_slice()).select1(n)
    }
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        Bits::new(self.as_slice()).select0(n)
    }
}

impl<B: Block + Select> Select for Box<B> {
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        self.as_ref().select1(n)
    }
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        self.as_ref().select0(n)
    }
}
