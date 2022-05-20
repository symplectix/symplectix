#![allow(clippy::many_single_char_names)]

use crate as bits;
use crate::{ops::*, BitBlock, Word};
use core::ops::{Range, RangeBounds};

fn for_each_blocks<T, F>(s: usize, e: usize, mut f: F)
where
    T: BitBlock,
    F: FnMut(usize, Range<usize>),
{
    assert!(s <= e);
    if s == e {
        return;
    }

    let (q0, r0) = bits::address::<T>(s);
    let (q1, r1) = bits::address::<T>(e);

    if q0 == q1 {
        f(q0, r0..r1);
    } else {
        f(q0, r0..T::BITS);
        (q0 + 1..q1).for_each(|k| f(k, 0..T::BITS));
        f(q1, 0..r1)
    }
}

impl<T: BitBlock> bits::ops::BitCount for [T] {
    #[inline]
    fn count_1(&self) -> usize {
        self.iter().map(bits::count_1).sum()
    }

    #[inline]
    fn count_0(&self) -> usize {
        self.iter().map(bits::count_0).sum()
    }
}

impl<T: BitBlock> bits::ops::BitAll for [T] {
    #[inline]
    fn all(&self) -> bool {
        self.iter().all(bits::all)
    }
}

impl<T: BitBlock> bits::ops::BitAny for [T] {
    #[inline]
    fn any(&self) -> bool {
        self.iter().any(bits::any)
    }
}

impl<T: BitBlock> bits::ops::BitRank for [T] {
    #[inline]
    fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = bits::to_range(&r, 0, bits::len(self));
        let (i, p) = bits::address::<T>(s);
        let (j, q) = bits::address::<T>(e);
        if i == j {
            self[i].rank_1(p..q)
        } else {
            self[i].rank_1(p..)
                + self[i + 1..j].count_1()
                + self.get(j).map_or(0, |b| bits::rank_1(b, ..q))
        }
    }
}

impl<T: BitBlock> bits::ops::BitSelect for [T] {
    #[inline]
    fn select_1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = bits::count_1(b);
            if n < count {
                return Some(i * T::BITS + bits::select_1(b, n).expect("BUG"));
            }
            n -= count;
        }
        None
    }

    #[inline]
    fn select_0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = bits::count_0(b);
            if n < count {
                return Some(i * T::BITS + bits::select_0(b, n).expect("BUG"));
            }
            n -= count;
        }
        None
    }
}

impl<T: BitBlock> bits::ops::BitGet for [T] {
    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        let (i, o) = bits::address::<T>(i);
        this.get(i)
            .map(|block| bits::get(block, o).expect("index out of bounds"))
    }

    #[inline]
    #[doc(hidden)]
    fn word<N: Word>(&self, i: usize, n: usize) -> N {
        let mut cur = 0;
        let mut out = N::NULL;
        for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() && cur < <N as BitBlock>::BITS {
                out |= bits::word::<_, N>(&self[k], r.start, r.len()) << cur;
                cur += r.len();
            }
        });
        out
    }
}

impl<T: BitBlock> bits::ops::BitPut for [T] {
    #[inline]
    fn put_1(&mut self, i: usize) {
        assert!(i < bits::len(self));
        let (i, o) = bits::address::<T>(i);
        bits::put_1(&mut self[i], o);
    }

    #[inline]
    fn put_0(&mut self, i: usize) {
        assert!(i < bits::len(self));
        let (i, o) = bits::address::<T>(i);
        bits::put_0(&mut self[i], o);
    }

    #[inline]
    #[doc(hidden)]
    fn put_word<N: Word>(&mut self, i: usize, n: usize, word: N) {
        let mut cur = 0;
        for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() {
                let word = bits::word(&word, cur, r.len());
                bits::put_word::<_, N>(&mut self[k], r.start, r.len(), word);
                cur += r.len();
            }
        });
    }
}
