use crate::{Block, Count, Rank};

pub trait Select: Rank {
    /// Returns the position of the n-th 1, indexed starting from zero.
    /// `n` must be less than `self.count1()`, orherwise returns `None`.
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        helper::search1(self, n)
    }

    /// Returns the position of the n-th 0, indexed starting from zero.
    /// `n` must be less than `self.count0()`, orherwise returns `None`.
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        helper::search0(self, n)
    }

    // #[inline]
    // fn select1_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select1(self.rank1(..i) + n).map(|pos| pos - i)
    // }

    // #[inline]
    // fn select0_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select0(self.rank0(..i) + n).map(|pos| pos - i)
    // }
}

mod helper {
    use crate::Rank;

    /// Binary search to find and return the smallest index k in `[i, j)` at which f(k) is true,
    /// assuming that on the range `[i, j)`, f(k) == true implies f(k+1) == true.
    ///
    /// Returns the first true index, if there is no such index, returns `j`.
    fn binary_search<P: Fn(usize) -> bool>(mut l: usize, mut r: usize, f: P) -> usize {
        while l < r {
            let m = l + (r - l) / 2;
            if f(m) {
                r = m; // -> f(r) == true
            } else {
                l = m + 1; // -> f(l-1) == false
            }
        }
        l // f(l-1) == false && f(l) (= f(r)) == true
    }

    #[inline]
    pub fn search1<T: ?Sized + Rank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count1()).then(|| binary_search(0, bs.bits(), |k| bs.rank1(..k) > n) - 1)
    }

    #[inline]
    pub fn search0<T: ?Sized + Rank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count0()).then(|| binary_search(0, bs.bits(), |k| bs.rank0(..k) > n) - 1)
    }
}

impl<T: Block> Select for [T] {
    #[inline]
    fn select1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count1();
            if n < count {
                return Some(i * T::BITS + b.select1(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }

    #[inline]
    fn select0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count0();
            if n < count {
                return Some(i * T::BITS + b.select0(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }
}

/// ```
/// # use bits::Select;
/// assert_eq!(Select::select1(&true, 0), Some(0));
/// assert_eq!(Select::select1(&true, 1), None);
///
/// assert_eq!(Select::select1(&false, 0), None);
/// assert_eq!(Select::select1(&false, 1), None);
///
/// assert_eq!(Select::select0(&false, 0), Some(0));
/// assert_eq!(Select::select0(&false, 1), None);
/// ```
impl Select for bool {
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        (n < self.count1()).then(|| 0)
    }

    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        (n < self.count0()).then(|| 0)
    }
}

macro_rules! impl_select {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn select1(&self, n: usize) -> Option<usize> {
            <$X as Select>::select1(self$(.$method())?, n)
        }

        #[inline]
        fn select0(&self, n: usize) -> Option<usize> {
            <$X as Select>::select0(self$(.$method())?, n)
        }
    }
}

impl<'a, T: ?Sized + Select> Select for &'a T {
    impl_select!(T);
}

impl<T, const N: usize> Select for [T; N]
where
    [T]: Select,
{
    impl_select!([T], as_ref);
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::borrow::{Cow, ToOwned};
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<T: ?Sized + Select> Select for Box<T> {
        impl_select!(T);
    }

    impl<T> Select for Vec<T>
    where
        [T]: Select,
    {
        impl_select!([T]);
    }

    impl<'a, T> Select for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Select,
    {
        impl_select!(T, as_ref);
    }
}
