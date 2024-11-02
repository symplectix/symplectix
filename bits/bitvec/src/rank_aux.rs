use super::empty;
use crate::blocks;
use crate::L1L2;
use bits::{index, Bits, Container, Count, Rank, Select, Varint};
use fenwicktree::{LowerBound, Nodes, Prefix};
use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::iter::Sum;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, RangeBounds, Sub, SubAssign};

// mod buckets;
// mod pop;
// mod rho;

// pub use accumulate::BitArray;

/// `T` + auxiliary indices to compute [`bits::Rank`](bits::Rank) and [`bits::Select`](bits::Select).
///
/// [`rank`]: crate::bits::Bits
/// [`select`]: crate::bits::Bits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rho<T>(Imp<T, layout::Rho>);

// /// `T` + auxiliary indices to compute [`bits::Rank`](bits::Rank) and [`bits::Select`](bits::Select).
// ///
// /// [`rank`]: crate::bits::Bits
// /// [`select`]: crate::bits::Bits
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Pop<T>(Imp<T, layout::Pop>);

// TODO: implement Debug for Imp, and remove Debug from Buckets
#[derive(Debug, Clone, PartialEq, Eq)]
struct Imp<T, S> {
    buckets: Buckets<S>,
    samples: Option<Vec<Vec<u32>>>,
    bit_vec: T,
}

const UPPER: usize = 1 << 32;
const SUPER: usize = 1 << 11;
const BASIC: usize = 1 << 9;
const MAXL1: usize = UPPER / SUPER; // 2097152
const SAMPLE: usize = 1 << 13;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Buckets<T> {
    pub(crate) hi: Vec<u64>,
    pub(crate) lo: Vec<L1L2>,
    _marker: PhantomData<T>,
}

mod layout {
    /// Defines how to handle `prefix sum` of the population.
    pub(crate) trait Layout {}

    /// Accumulates the number of bits as follows.
    ///
    /// L0: Cumulative absolute counts, per `UPPER` bits.
    /// L1: Cumulative     relative counts
    /// L2: Non-cumulative relative counts
    ///
    /// L1[i] and L2[i] are interleaved into one word.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Pop {}

    /// Builds a [`FenwickTree`] to compute prefix sum instead of accumulating.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Rho {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Uninit {}

    impl Layout for Pop {}
    impl Layout for Rho {}
    impl Layout for Uninit {}
}

// impl<S> fmt::Debug for Buckets<S> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_tuple("Buckets").field(&self.hi).finish()
//     }
// }

pub(crate) fn build<'a, T, I>(
    size: usize,
    super_blocks: I,
) -> (Buckets<layout::Uninit>, Vec<Vec<u32>>)
where
    T: num::Int + bits::Bits + 'a,
    I: IntoIterator<Item = Option<&'a [T]>>,
{
    use fenwicktree::Nodes;

    let mut buckets = Buckets::new(size);
    let mut samples = vec![Vec::new(); buckets.hi.nodes()];
    let mut ones = 0i64;

    fn bbs<W: num::Int + bits::Bits>(sb: Option<&[W]>) -> [u64; L1L2::LEN] {
        let mut bbs = [0; L1L2::LEN];
        if let Some(sb) = sb.as_ref() {
            for (i, bb) in sb.chunks(BASIC / W::BITS).enumerate() {
                bbs[i] = bb.count1() as u64;
            }
        }
        bbs
    }

    for (i, sb) in super_blocks.into_iter().enumerate() {
        let bbs = bbs(sb);
        let sum = bbs.iter().sum::<u64>();

        let (q, r) = num::divrem(i, MAXL1);

        {
            // +1 to skip dummy index
            buckets.hi[q + 1] += sum;
            buckets.lo_mut(q)[r + 1] = L1L2::merge([sum, bbs[0], bbs[1], bbs[2]]);
        }

        {
            // diff between `ones` and `SAMPLE_BITS * k`
            let rem = (-ones).rem_euclid(SAMPLE as i64);

            if (rem as u64) < sum {
                let offset = i * SUPER - q * UPPER;
                let select = sb.unwrap().select1(rem as usize).unwrap();
                samples[q].push(num::cast(offset + select));
            }

            if r == MAXL1 - 1 {
                ones = 0;
            } else {
                ones += sum as i64;
            }
        }
    }

    // fenwick1::init(&mut fws.hi);
    // for q in 0..fws.hi.size() {
    //     fenwick1::init(fws.lo_mut(q));
    // }

    (buckets, samples)
}

pub(crate) fn sbs_from_words<T: num::Int + bits::Bits>(
    slice: &[T],
) -> impl Iterator<Item = Option<&[T]>> {
    slice.chunks(SUPER / T::BITS).map(Some)
}

impl From<Buckets<layout::Uninit>> for Buckets<layout::Rho> {
    fn from(mut uninit: Buckets<layout::Uninit>) -> Buckets<layout::Rho> {
        fenwicktree::build(&mut uninit.hi);
        for q in 0..uninit.lo_parts() {
            fenwicktree::build(uninit.lo_mut(q));
        }
        Buckets { hi: uninit.hi, lo: uninit.lo, _marker: PhantomData }
    }
}

impl From<Buckets<layout::Uninit>> for Buckets<layout::Pop> {
    fn from(mut flat: Buckets<layout::Uninit>) -> Buckets<layout::Pop> {
        use fenwicktree::Nodes;

        let mut sum = 0;
        for acc in flat.hi[1..].iter_mut() {
            sum += *acc;
            *acc = sum;
        }

        for q in 0..flat.hi.nodes() {
            let lo = flat.lo_mut(q);

            let mut sum = 0;
            for L1L2(acc) in lo[1..].iter_mut() {
                let cur = *acc & L1L2::L1;
                *acc = (*acc & !L1L2::L1) | sum;
                sum += cur;
            }
        }

        Buckets { hi: flat.hi, lo: flat.lo, _marker: PhantomData }
    }
}

// impl From<Buckets<Rho>> for Buckets<Pop> {
//     fn from(mut f: Buckets<Rho>) -> Buckets<Pop> {
//         use fenwicktree::Nodes;

//         let hi = fenwicktree::accumulate(&f.hi);

//         for q in 0..f.hi.nodes() {
//             let lo = f.lo_mut(q);
//             for (i, l1) in fenwicktree::accumulate(lo).iter().enumerate() {
//                 if i + 1 < lo.len() {
//                     let L1L2(ll) = lo[i + 1];
//                     lo[i + 1] = L1L2((ll & !L1L2::L1) | l1);
//                 }
//             }
//         }

//         Buckets { hi, lo: f.lo, _marker: PhantomData }
//     }
// }

#[inline]
fn hilen(n: usize) -> usize {
    blocks(n, UPPER) + 1
}

#[inline]
fn lolen(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        // A minimum and a *logical* length of a vector to store `LL`.
        let supers = blocks(n, SUPER);
        // Computes how many fenwicks do we need actually.
        // Remenber that fenwicks for L1 and L2 is logically `Vec<Vec<LL>>` but flattened.
        let (q, r) = num::divrem(supers, MAXL1);
        // Need additional space for each fenwicks because of its 1-based indexing.
        supers + q + (r > 0) as usize
    }
}

impl<S> Buckets<S> {
    pub(crate) fn new(n: usize) -> Buckets<S> {
        let hi = vec![0; hilen(n)];
        let lo = vec![L1L2(0); lolen(n)];
        Buckets { hi, lo, _marker: std::marker::PhantomData }
    }

    #[inline]
    pub(crate) fn lo(&self, i: usize) -> &[L1L2] {
        let s = (MAXL1 + 1) * i;
        let e = cmp::min(s + (MAXL1 + 1), self.lo.len());
        &self.lo[s..e]
    }

    #[inline]
    pub(crate) fn lo_mut(&mut self, i: usize) -> &mut [L1L2] {
        let s = (MAXL1 + 1) * i;
        let e = cmp::min(s + (MAXL1 + 1), self.lo.len());
        &mut self.lo[s..e]
    }

    // The logical number of fenwicks hiding at `lo`.
    // #[cfg(test)]
    #[inline]
    pub(crate) fn lo_parts(&self) -> usize {
        // if self.low.len() == 1 {
        //     0
        // } else {
        //     blocks(self.low.len(), MAXL1 + 1)
        // }
        blocks(self.lo.len(), MAXL1 + 1)
    }
}

impl Buckets<layout::Rho> {
    pub(crate) fn add(&mut self, p0: usize, delta: u64) {
        use fenwicktree::Incr;

        let (q0, r0) = num::divrem(p0, UPPER);
        let (q1, r1) = num::divrem(r0, SUPER);

        let hi = &mut self.hi;
        hi.incr(q0 + 1, delta);

        let lo = self.lo_mut(q0);
        lo.incr(q1 + 1, delta);

        // Update L2 array which is interleaved into L1
        let sb = q1 + 1; // +1 because fenwick doesn't use index 0
        let bb = r1 / BASIC + 1; // +1 to skip index 0 which is for L1
        if bb < L1L2::LEN {
            lo[sb] = {
                let mut arr = L1L2::split(lo[sb]);
                arr[bb] += delta;
                L1L2::merge(arr)
            };
        }
    }

    pub(crate) fn sub(&mut self, p0: usize, delta: u64) {
        use fenwicktree::Decr;

        let (q0, r0) = num::divrem(p0, UPPER);
        let (q1, r1) = num::divrem(r0, SUPER);

        let hi = &mut self.hi;
        hi.decr(q0 + 1, delta);

        let lo = self.lo_mut(q0);
        lo.decr(q1 + 1, delta);

        let sb = q1 + 1;
        let bb = r1 / BASIC + 1;
        if bb < L1L2::LEN {
            lo[sb] = {
                let mut arr = L1L2::split(lo[sb]);
                arr[bb] -= delta;
                L1L2::merge(arr)
            };
        }
    }

    // fn resize(&mut self, cur_len: usize, new_len: usize) {
    //     let (cur_top_len, cur_low_len) = (self.top.len(), self.low.len());
    //     let (new_top_len, new_low_len) = (toplen(new_len), lowlen(new_len));
    //
    //     let top = &mut self.top;
    //     let low = &mut self.low;
    //
    //     match dbg!(cur_len.cmp(&new_len)) {
    //         // cur_len == new_len
    //         EQ =>
    //         {
    //             #[cfg(test)]
    //             (|| {
    //                 assert_eq!(cur_top_len, new_top_len);
    //                 assert_eq!(cur_low_len, new_low_len);
    //             })()
    //         }
    //
    //         // cur_len < new_len
    //         LT => {
    //             unimplemented!()
    //         }
    //
    //         // cur_len > new_len
    //         GT => {
    //             top.truncate(new_top_len);
    //             low.truncate(new_low_len);
    //         }
    //     }
    // }
}

impl<T: Bits> Rho<Vec<T>> {
    #[inline]
    pub fn new(n: usize) -> Rho<Vec<T>> {
        let dat = empty(n);
        Rho(Imp { buckets: Buckets::new(dat.bits()), samples: None, bit_vec: dat })
    }
}

// impl<'a, T: Clone> From<Rho<&'a [T]>> for Rho<Vec<T>> {
//     fn from(Rho(imp): Rho<&'a [T]>) -> Self {
//         Rho(Imp { buckets: imp.buckets, samples: None, bit_vec: imp.bit_vec.to_vec() })
//     }
// }

impl<'a, T: num::Int + bits::Bits> From<&'a [T]> for Rho<&'a [T]> {
    fn from(dat: &'a [T]) -> Self {
        let (buckets, _) = build(dat.bits(), sbs_from_words(dat));
        Rho(Imp { buckets: buckets.into(), samples: None, bit_vec: dat })
    }
}

// impl<'a, T: WordArray> From<&'a [Block<T>]> for Rho<&'a [Block<T>]> {
//     fn from(dat: &'a [Block<T>]) -> Self {
//         let (buckets, _) = build(Seq::len(&dat), sbs_from_heaps(&dat));
//         Rho(Imp { buckets: buckets.into(), samples: None, bit_vec: dat })
//     }
// }

// impl<T: Word, U: From<Vec<T>>> From<Vec<T>> for Rho<U> {
//     fn from(dat: Vec<T>) -> Self {
//         let (buckets, _) = build(Seq::len(&dat), sbs_from_words(&dat));
//         Rho(Imp { buckets: buckets.into(), samples: None, bit_vec: dat.into() })
//     }
// }

// impl<T: WordArray, U: From<Vec<Block<T>>>> From<Vec<Block<T>>> for Rho<U> {
//     fn from(dat: Vec<Block<T>>) -> Self {
//         let (buckets, _) = build(Seq::len(&dat), sbs_from_heaps(&dat));
//         Rho(Imp { buckets: buckets.into(), samples: None, bit_vec: dat.into() })
//     }
// }

impl<T: Container> Container for Rho<T> {
    #[inline]
    fn bits(&self) -> usize {
        self.0.bit_vec.bits()
    }

    #[inline]
    fn bit(&self, i: usize) -> Option<bool> {
        self.0.bit_vec.bit(i)
    }
}

impl<T: Count> Count for Rho<T> {
    #[inline]
    fn count1(&self) -> usize {
        let bit = &self.0.buckets.hi;
        num::cast::<u64, usize>(bit.sum(bit.nodes()))
        // fenwicktree::sum(&self.0.buckets.hi).cast()
        // cast(self.buckets.hi.sum(self.buckets.hi.size()))
        // let top0 = self.samples.top[0];
        // #[cfg(test)]
        // assert_eq!(top0 as usize, self.buf.count1());
        // cast(top0)
    }
}

impl<T: Rank> Rank for Rho<T> {
    fn rank1<Idx: RangeBounds<usize>>(&self, index: Idx) -> usize {
        fn rank1_impl<U: Rank>(me: &Rho<U>, p0: usize) -> usize {
            if p0 == 0 {
                0
            } else if p0 == me.bits() {
                me.count1()
            } else {
                let Rho(me) = me;
                let (q0, r0) = num::divrem(p0, UPPER);
                let (q1, r1) = num::divrem(r0, SUPER);
                let (q2, r2) = num::divrem(r1, BASIC);

                let hi = &me.buckets.hi;
                let lo = &me.buckets.lo(q0);
                let c0: u64 = hi.sum(q0);
                let c1: u64 = lo.sum(q1);
                let c2 = lo[q1 + 1].l2(q2);
                num::cast::<_, usize>(c0 + c1 + c2) + me.bit_vec.rank1(p0 - r2..p0)
            }
        }
        use std::ops::Range;
        let Range { start: i, end: j } = index::to_range(&index, 0, self.bits());
        rank1_impl(self, j) - rank1_impl(self, i)
    }
}

// impl<T: Bits> Bits for Rho<T> {
//     #[inline]
//     fn word<N: Word>(&self, i: usize, n: usize) -> N {
//         self.0.bit_vec.word(i, n)
//     }
// }

impl<T: Varint + Select> Select for Rho<T> {
    fn select1(&self, n: usize) -> Option<usize> {
        let Rho(imp) = self;
        let mut r = num::cast(n);

        let (s, e) = {
            let p0 = find_l0(&imp.buckets.hi[..], &mut r)?;
            let lo = imp.buckets.lo(p0);
            let p1 = find_l1(lo, &mut r);
            let ll = lo[p1 + 1];
            let l2 = [ll.l2_0(), ll.l2_1(), ll.l2_2()];
            let p2 = find_l2(&l2, &mut r);

            let s = p0 * UPPER + p1 * SUPER + p2 * BASIC;
            (s, cmp::min(s + BASIC, self.bits()))
        };

        let mut r = r as usize;
        {
            debug_assert!(n - r == self.rank1(..s));
            debug_assert!(r < self.rank1(s..e));
        }

        // i + imp.bit_vec[x..y].select1(r).unwrap()

        const BITS: usize = <u128 as Bits>::BITS;
        for i in (s..e).step_by(BITS) {
            let b = imp.bit_vec.varint::<u128>(i, BITS);
            let c = b.count1();
            if r < c {
                // #[cfg(test)]
                // {
                //     dbg!(l0, l1, l2);
                //     dbg!(lo[l1 + 1]);
                //     dbg!(s, e);
                // }
                return Some(i + b.select1(r).unwrap());
            }
            r -= c;
        }
        unreachable!()
    }

    fn select0(&self, n: usize) -> Option<usize> {
        let Rho(imp) = self;
        let mut r = num::cast(n);

        let (s, e) = {
            const UB: u64 = UPPER as u64;
            const SB: u64 = SUPER as u64;
            const BB: u64 = BASIC as u64;
            let hi_complemented = fenwicktree::complement(&imp.buckets.hi[..], UB);
            let p0 = find_l0(&hi_complemented, &mut r)?;
            let lo = imp.buckets.lo(p0);
            let lo_complemented = fenwicktree::complement(lo, SB);
            let p1 = find_l1(&lo_complemented, &mut r);
            let ll = lo[p1 + 1];
            let l2 = [BB - ll.l2_0(), BB - ll.l2_1(), BB - ll.l2_2()];
            let p2 = find_l2(&l2, &mut r);

            let s = p0 * UPPER + p1 * SUPER + p2 * BASIC;
            (s, cmp::min(s + BASIC, self.bits()))
        };

        let mut r = r as usize;
        {
            debug_assert!(n - r == self.rank0(..s));
            debug_assert!(r < self.rank0(s..e));
        }

        const BITS: usize = <u128 as Bits>::BITS;
        for i in (s..e).step_by(BITS) {
            let b = imp.bit_vec.varint::<u128>(i, BITS);
            let c = b.count0();
            if r < c {
                return Some(i + b.select0(r).unwrap());
            }
            r -= c;
        }
        unreachable!()
    }
}

fn find_l0<L0>(l0: &L0, r: &mut u64) -> Option<usize>
where
    L0: ?Sized + Nodes + Prefix<u64> + LowerBound<u64>,
    u64: Sum<L0::Node>,
{
    // r: +1 because `select1(n)` returns the position of the n-th one, indexed starting from zero.
    // i: -1 is safe because lower_bound(x) returns 0 iif x is 0
    let p0 = l0.lower_bound(*r + 1) - 1;
    if p0 >= l0.nodes() {
        None
    } else {
        *r -= l0.sum(p0);
        Some(p0)
    }
}

fn find_l1<L1>(l1: &L1, r: &mut u64) -> usize
where
    L1: ?Sized + Nodes + Prefix<u64> + LowerBound<u64>,
    u64: Sum<L1::Node>,
{
    let p1 = l1.lower_bound(*r + 1) - 1;
    *r -= l1.sum(p1);
    p1
}

fn find_l2<'a, L2>(l2: L2, r: &mut u64) -> usize
where
    L2: IntoIterator<Item = &'a u64> + 'a,
{
    let mut p2 = 0;
    for &c in l2.into_iter() {
        if *r < c {
            break;
        }
        *r -= c;
        p2 += 1;
    }
    p2
}

// impl<T: Bits + BitPut> Rho<T> {
//     /// Swaps a bit at `i` by `bit` and returns the previous value.
//     fn swap(&mut self, i: usize, bit: bool) -> bool {
//         let before = self.0.bit_vec.bit(i);
//         if bit {
//             self.0.bit_vec.put1(i);
//         } else {
//             self.0.bit_vec.put0(i);
//         }
//         before
//     }

//     // /// Resizes the `Pop` in-place so that `Pop` has at least `min` bits.
//     // #[inline]
//     // pub fn resize(&mut self, new_len: usize) {
//     //     let cur_len = len(&self.buf);
//     //     self.buf.resize_with(blocks(new_len, T::BITS), T::empty);
//     //     self.samples.resize(cur_len, len(&self.buf));
//     // }
// }

// impl<T: Bits + BitPut> BitPut for Rho<T> {
//     #[inline]
//     fn put1(&mut self, p0: usize) {
//         if !self.swap(p0, true) {
//             self.0.buckets.add(p0, 1);
//         }
//     }
//     #[inline]
//     fn put0(&mut self, p0: usize) {
//         if self.swap(p0, false) {
//             self.0.buckets.sub(p0, 1);
//         }
//     }
// }
