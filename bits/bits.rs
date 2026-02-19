//! Provides basic bit operations and utilities.

use std::borrow::Cow;
use std::cmp::Ordering;
use std::iter::successors;
use std::ops::{
    Bound,
    RangeBounds,
};

mod bits_mut;
mod block;
mod buf;
mod mask;
mod word;

pub use bits_mut::BitsMut;
pub use block::{
    Block,
    FromBlocks,
    IntoBlocks,
};
pub use buf::Buf;
pub use mask::Masking;
pub use word::Word;

mod and;
mod not;
mod or;
mod xor;

pub use and::And;
pub use not::Not;
pub use or::Or;
pub use xor::Xor;

/// Calculates the minimum number of blocks to store `n` bits.
#[inline]
pub const fn blocks(n: u64, b: u64) -> usize {
    let len = (n / b) as usize;
    len + (!n.is_multiple_of(b) as usize)
}

/// Returns a pair of numbers.
#[inline]
pub const fn index(i: u64, b: u64) -> (usize, u64) {
    let j = (i / b) as usize;
    let k = i % b;
    (j, k)
}

/// A utility to clamp the given range, which is possibly unbounded,
/// into a bounded `[i, j)`. Panics when debug is enabled and if
/// `!(min <= i && i <= j && j <= max)`.
pub fn range<R>(r: &R, min: u64, max: u64) -> (u64, u64)
where
    R: RangeBounds<u64>,
{
    let i = min_index_inclusive(r.start_bound(), min);
    let j = max_index_exclusive(r.end_bound(), max);
    debug_assert!(min <= i && i <= j && j <= max);
    (i, j)
}

#[inline]
const fn min_index_inclusive(bound: Bound<&u64>, min: u64) -> u64 {
    match bound {
        Bound::Included(&s) => s,
        Bound::Excluded(&s) => s + 1,
        Bound::Unbounded => min,
    }
}

#[inline]
const fn max_index_exclusive(bound: Bound<&u64>, max: u64) -> u64 {
    match bound {
        Bound::Included(&e) => e + 1,
        Bound::Excluded(&e) => e,
        Bound::Unbounded => max,
    }
}

/// Splits a given range [start, end) into chunks.
/// Each chunk is represented as a (index, len) tuple, and its rhs, index+len, is aligned to a
/// multiple of n.
///
/// # Examples
///
/// ```
/// let mut it = bits::chunks(10, 0, 3);
/// assert_eq!(it.next(), None);
///
/// let mut it = bits::chunks(10, 10, 3);
/// assert_eq!(it.next(), None);
///
/// let mut it = bits::chunks(10, 12, 3);
/// assert_eq!(it.next(), Some((10, 2)));
/// assert_eq!(it.next(), None);
///
/// let mut it = bits::chunks(10, 20, 3);
/// assert_eq!(it.next(), Some((10, 2)));
/// assert_eq!(it.next(), Some((12, 3)));
/// assert_eq!(it.next(), Some((15, 3)));
/// assert_eq!(it.next(), Some((18, 2)));
/// assert_eq!(it.next(), None);
///
/// let mut it = bits::chunks(10, 21, 3);
/// assert_eq!(it.next(), Some((10, 2)));
/// assert_eq!(it.next(), Some((12, 3)));
/// assert_eq!(it.next(), Some((15, 3)));
/// assert_eq!(it.next(), Some((18, 3)));
/// assert_eq!(it.next(), None);
/// ```
pub fn chunks(start: u64, end: u64, n: u64) -> impl Iterator<Item = (u64, u64)> {
    let step = move |i| (i < end).then(|| (i, next_multiple_of(i, n).min(end) - i));
    successors(step(start), move |&(index, len)| step(index + len))
}

#[inline]
const fn next_multiple_of(x: u64, n: u64) -> u64 {
    // TODO: Use usize::checked_next_multiple_of
    // https://doc.rust-lang.org/std/primitive.usize.html#method.checked_next_multiple_of
    // https://github.com/rust-lang/rust/issues/88581
    x + (n - x % n)
}

pub(crate) fn compare<X, Y>(
    x: Option<&(usize, X)>,
    y: Option<&(usize, Y)>,
    when_x_is_none: Ordering,
    when_y_is_none: Ordering,
) -> Ordering {
    match (x, y) {
        (None, _) => when_x_is_none,
        (_, None) => when_y_is_none,
        (Some((i, _x)), Some((j, _y))) => i.cmp(j),
    }
}

/// A bit sequence, consisting of 1s and 0s.
///
/// This trait contains methods with cyclic defaults,
/// necessitating the definition of the following:
/// - bit
/// - At least two of bits, count1, and count0
/// - At least one of rank1 and rank0
pub trait Bits {
    /// Reads a bit at `i`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert!(v.bit(0));
    /// assert!(v.bit(64));
    /// assert!(!v.bit(128));
    /// // Returns false if out of bounds.
    /// assert!(!v.bit(200));
    /// ```
    fn bit(&self, i: u64) -> bool;

    /// Reads a word from `i`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u16] = &[0b_1101_0001_1010_0011, 0b_1001_1110_1110_1001];
    /// assert_eq!(v.word::<u8>(0, 4), 0b0011);
    /// assert_eq!(v.word::<u8>(8, 4), 0b0001);
    /// assert_eq!(v.word::<u8>(14, 4), 0b0111);
    /// assert_eq!(v.word::<u8>(30, 4), 0b0010);
    /// ```
    fn word<T: Word>(&self, i: u64, len: u64) -> T {
        debug_assert!(i < self.bits() && len <= T::BITS);

        let mut word = T::empty();
        for b in 0..len {
            if self.bit(i + b) {
                word.set1(b);
            }
        }
        word
    }

    /// The number of bits, which must always be equal to `count1() + count0()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let a: &[u8] = &[];
    /// let b: &[u8] = &[0, 0, 0];
    /// let c: &[u8] = &[0, 1, 3];
    /// assert_eq!(a.bits(), 0);
    /// assert_eq!(b.bits(), 24);
    /// assert_eq!(c.bits(), 24);
    /// ```
    #[inline]
    fn bits(&self) -> u64 {
        self.count1() + self.count0()
    }

    /// The number of 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count1(), 0);
    /// assert_eq!(b.count1(), 0);
    /// assert_eq!(c.count1(), 3);
    /// ```
    #[inline]
    fn count1(&self) -> u64 {
        self.bits() - self.count0()
    }

    /// The number of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 3];
    /// assert_eq!(a.count0(), 0);
    /// assert_eq!(b.count0(), 192);
    /// assert_eq!(c.count0(), 189);
    /// ```
    #[inline]
    fn count0(&self) -> u64 {
        self.bits() - self.count1()
    }

    /// Returns true if all bits are set.
    /// If empty (`self.bits() == 0`), return true.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[!0, !0, !0];
    /// assert!(a.all());
    /// assert!(!b.all());
    /// assert!(c.all());
    /// ```
    #[inline]
    fn all(&self) -> bool {
        self.count0() == 0
    }

    /// Returns true if any bits are set.
    /// If empty (`self.bits() == 0`), return false.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let a: &[u64] = &[];
    /// let b: &[u64] = &[0, 0, 0];
    /// let c: &[u64] = &[0, 1, 0];
    /// assert!(!a.any());
    /// assert!(!b.any());
    /// assert!(c.any());
    /// ```
    #[inline]
    fn any(&self) -> bool {
        self.count1() > 0
    }

    /// The number of 1 in the given range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.rank1(..), v.count1());
    /// assert_eq!(v.rank1(..), 8);
    /// assert_eq!(v.rank1(1..), 7);
    /// assert_eq!(v.rank1(..2), 1);
    /// assert_eq!(v.rank1(60..70), 3);
    /// ```
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[];
    /// assert_eq!(v.rank1(..), 0);
    /// ```
    ///
    /// ```should_panic
    /// # use bits::Bits;
    /// # let v: &[u64] = &[];
    /// assert_eq!(v.rank1(1..), 0);
    /// assert_eq!(v.rank1(..100), 0);
    /// ```
    #[inline]
    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        let (i, j) = crate::range(&r, 0, self.bits());
        (j - i) - self.rank0(r)
    }

    /// The number of 0 in the given range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.rank0(..), v.count0());
    /// assert_eq!(v.rank0(..5), 3);
    /// assert_eq!(v.rank0(60..70), 7);
    /// ```
    #[inline]
    fn rank0<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        let (i, j) = crate::range(&r, 0, self.bits());
        (j - i) - self.rank1(r)
    }

    /// Excessive bits, absolute diff of rank1 and rank0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.excess(..), v.count1().abs_diff(v.count0()));
    /// assert_eq!(v.excess(10..20), v.rank1(10..20).abs_diff(v.rank0(10..20)));
    /// ```
    #[inline]
    fn excess<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        excess_helper::ranks(self, r).excess()
    }

    /// Excessive bits, computed by `rank1.checked_sub(rank0)`.
    #[inline]
    fn excess1<R: RangeBounds<u64>>(&self, r: R) -> Option<u64> {
        excess_helper::ranks(self, r).excess1()
    }

    /// Excessive bits, computed by `rank0.checked_sub(rank1)`.
    #[inline]
    fn excess0<R: RangeBounds<u64>>(&self, r: R) -> Option<u64> {
        excess_helper::ranks(self, r).excess0()
    }

    /// The position of `n`th occurrence of 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.select1(0).unwrap(), 0);
    /// assert_eq!(v.select1(1).unwrap(), 2);
    /// assert_eq!(v.select1(2).unwrap(), 64);
    /// assert_eq!(v.select1(3).unwrap(), 65);
    /// ```
    #[inline]
    fn select1(&self, n: u64) -> Option<u64> {
        self.search1(n)
    }

    /// The position of `n`th occurrence of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.select0(0).unwrap(), 1);
    /// assert_eq!(v.select0(1).unwrap(), 3);
    /// assert_eq!(v.select0(v.count0() - 1).unwrap(), 191);
    /// ```
    #[inline]
    fn select0(&self, n: u64) -> Option<u64> {
        self.search0(n)
    }

    #[doc(hidden)]
    #[inline]
    fn search1(&self, n: u64) -> Option<u64> {
        (n < self.count1()).then(|| binary_search(0, self.bits(), |k| self.rank1(..k) > n) - 1)
    }

    #[doc(hidden)]
    #[inline]
    fn search0(&self, n: u64) -> Option<u64> {
        (n < self.count0()).then(|| binary_search(0, self.bits(), |k| self.rank0(..k) > n) - 1)
    }

    /// Return the intersection of two sets as an iterator of blocks.
    ///
    /// The intersection of two sets is the set containing
    /// all elements of A that also belong to B or equivalently,
    /// all elements of B that also belong to A.
    fn and<'a, That>(&'a self, that: That) -> And<&'a Self, That>
    where
        And<&'a Self, That>: IntoBlocks,
    {
        And { a: self, b: that }
    }

    /// Returns the union of two sets as an iterator of blocks.
    ///
    /// The union of two sets is the set of all elements
    /// in the both of the sets.
    fn or<'a, That>(&'a self, that: That) -> Or<&'a Self, That>
    where
        Or<&'a Self, That>: IntoBlocks,
    {
        Or { a: self, b: that }
    }

    /// Returns the difference of two sets as an iterator of blocks.
    ///
    /// The difference, or subtraction is the set that consists of
    /// elements that are in A but not in B.
    fn not<'a, That>(&'a self, that: That) -> Not<&'a Self, That>
    where
        Not<&'a Self, That>: IntoBlocks,
    {
        Not { a: self, b: that }
    }

    /// Returns the symmetric difference of two sets as an iterator of blocks.
    ///
    /// The symmetric difference of two sets is the set of elements
    /// which are in either of the sets, but not in their intersection.
    fn xor<'a, That>(&'a self, that: That) -> Xor<&'a Self, That>
    where
        Xor<&'a Self, That>: IntoBlocks,
    {
        Xor { a: self, b: that }
    }
}

/// Finds the smallest index k in `[i, j)` at which f(k) is true,
/// assuming that on the range `[i, j)`, f(k) == true implies
/// f(k+1) == true.
///
/// If there is no such index, returns `j`.
fn binary_search<F>(mut i: u64, mut j: u64, f: F) -> u64
where
    F: Fn(u64) -> bool,
{
    assert!(i < j);
    while i < j {
        let h = i + (j - i) / 2;
        if f(h) {
            j = h; // f(j) == true
        } else {
            i = h + 1; // f(i-1) == false
        }
    }
    i // f(i-1) == false && f(i) (= f(j)) == true
}

mod excess_helper {
    use std::ops::RangeBounds;

    use crate::Bits;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct Ranks {
        rank0: u64,
        rank1: u64,
    }

    /// Computes `rank0` and `rank1` at a time.
    pub(crate) fn ranks<T, R>(b: &T, r: R) -> Ranks
    where
        T: ?Sized + Bits,
        R: RangeBounds<u64>,
    {
        let (i, j) = crate::range(&r, 0, b.bits());
        let len = j - i;
        let rank1 = b.rank1(r);
        let rank0 = len - rank1;
        Ranks { rank0, rank1 }
    }

    impl Ranks {
        #[inline]
        pub(crate) fn excess(self) -> u64 {
            let Ranks { rank0, rank1 } = self;
            rank0.abs_diff(rank1)
        }

        #[inline]
        pub(crate) fn excess1(self) -> Option<u64> {
            let Ranks { rank0, rank1 } = self;
            rank1.checked_sub(rank0)
        }

        #[inline]
        pub(crate) fn excess0(self) -> Option<u64> {
            let Ranks { rank0, rank1 } = self;
            rank0.checked_sub(rank1)
        }
    }
}

impl<T: Block> Bits for [T] {
    #[inline]
    fn bits(&self) -> u64 {
        T::BITS * self.len() as u64
    }

    #[inline]
    fn count1(&self) -> u64 {
        self.iter().map(|b| b.count1()).sum()
    }

    #[inline]
    fn count0(&self) -> u64 {
        self.iter().map(|b| b.count0()).sum()
    }

    #[inline]
    fn all(&self) -> bool {
        self.iter().all(|b| b.all())
    }

    #[inline]
    fn any(&self) -> bool {
        self.iter().any(|b| b.any())
    }

    #[inline]
    fn bit(&self, i: u64) -> bool {
        let (i, o) = crate::index(i, T::BITS);
        self.get(i).is_some_and(|t| t.bit(o))
    }

    fn word<B: Word>(&self, i: u64, len: u64) -> B {
        debug_assert!(i < self.bits() && len <= B::BITS);
        let (s, p) = crate::index(i, T::BITS);
        let (e, q) = crate::index(i + len, T::BITS);
        if s == e {
            self[s].word(p, q - p)
        } else {
            let mut cur = 0;
            let mut out = B::empty();
            out |= self[s].word::<B>(p, T::BITS - p) << (cur as usize);
            cur += T::BITS - p;
            for block in self.iter().take(e).skip(s + 1) {
                out |= block.word::<B>(0, T::BITS) << (cur as usize);
                cur += T::BITS;
            }
            if e < self.len() {
                out |= self[e].word::<B>(0, q) << (cur as usize);
            }
            out
        }
    }

    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        match crate::range(&r, 0, self.bits()) {
            (0, j) => {
                let (j, q) = crate::index(j, T::BITS);
                self[..j].count1() + self.get(j).map_or(0, |p| p.rank1(..q))
            }
            (i, j) => {
                let (i, p) = crate::index(i, T::BITS);
                let (j, q) = crate::index(j, T::BITS);
                if i == j {
                    self[i].rank1(p..q)
                } else {
                    self[i].rank1(p..)
                        + self[i + 1..j].count1()
                        + self.get(j).map_or(0, |b| b.rank1(..q))
                }
            }
        }
    }

    #[inline]
    fn rank0<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        let (i, j) = crate::range(&r, 0, self.bits());
        (j - i) - self.rank1(r)
    }

    fn select1(&self, mut n: u64) -> Option<u64> {
        for (i, b) in self.iter().enumerate() {
            let i = i as u64;
            let count = b.count1();
            if n < count {
                return Some(i * T::BITS + b.select1(n).expect("bug"));
            }
            n -= count;
        }
        None
    }

    fn select0(&self, mut n: u64) -> Option<u64> {
        for (i, b) in self.iter().enumerate() {
            let i = i as u64;
            let count = b.count0();
            if n < count {
                return Some(i * T::BITS + b.select0(n).expect("bug"));
            }
            n -= count;
        }
        None
    }
}

impl<T: Block, const N: usize> Bits for [T; N] {
    #[inline]
    fn bits(&self) -> u64 {
        T::BITS * N as u64
    }

    #[inline]
    fn count1(&self) -> u64 {
        self.as_slice().count1()
    }

    #[inline]
    fn count0(&self) -> u64 {
        self.as_slice().count0()
    }

    #[inline]
    fn all(&self) -> bool {
        self.as_slice().all()
    }

    #[inline]
    fn any(&self) -> bool {
        self.as_slice().any()
    }

    #[inline]
    fn bit(&self, i: u64) -> bool {
        self.as_slice().bit(i)
    }

    #[inline]
    fn word<B: Word>(&self, i: u64, len: u64) -> B {
        self.as_slice().word(i, len)
    }

    #[inline]
    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.as_slice().rank1(r)
    }

    #[inline]
    fn rank0<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.as_slice().rank0(r)
    }

    #[inline]
    fn select1(&self, n: u64) -> Option<u64> {
        self.as_slice().select1(n)
    }

    #[inline]
    fn select0(&self, n: u64) -> Option<u64> {
        self.as_slice().select0(n)
    }
}

impl<T: Block> Bits for Vec<T> {
    #[inline]
    fn bits(&self) -> u64 {
        T::BITS * self.as_slice().len() as u64
    }

    #[inline]
    fn count1(&self) -> u64 {
        self.as_slice().count1()
    }

    #[inline]
    fn count0(&self) -> u64 {
        self.as_slice().count0()
    }

    #[inline]
    fn all(&self) -> bool {
        self.as_slice().all()
    }

    #[inline]
    fn any(&self) -> bool {
        self.as_slice().any()
    }

    #[inline]
    fn bit(&self, i: u64) -> bool {
        self.as_slice().bit(i)
    }

    #[inline]
    fn word<B: Word>(&self, i: u64, len: u64) -> B {
        self.as_slice().word(i, len)
    }

    #[inline]
    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.as_slice().rank1(r)
    }

    #[inline]
    fn rank0<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.as_slice().rank0(r)
    }

    #[inline]
    fn select1(&self, n: u64) -> Option<u64> {
        self.as_slice().select1(n)
    }

    #[inline]
    fn select0(&self, n: u64) -> Option<u64> {
        self.as_slice().select0(n)
    }
}

impl<T: Bits> Bits for Box<T> {
    #[inline]
    fn bits(&self) -> u64 {
        self.as_ref().bits()
    }

    #[inline]
    fn count1(&self) -> u64 {
        self.as_ref().count1()
    }

    #[inline]
    fn count0(&self) -> u64 {
        self.as_ref().count0()
    }

    #[inline]
    fn all(&self) -> bool {
        self.as_ref().all()
    }

    #[inline]
    fn any(&self) -> bool {
        self.as_ref().any()
    }

    #[inline]
    fn bit(&self, i: u64) -> bool {
        self.as_ref().bit(i)
    }

    #[inline]
    fn word<B: Word>(&self, i: u64, len: u64) -> B {
        self.as_ref().word(i, len)
    }

    #[inline]
    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.as_ref().rank1(r)
    }

    #[inline]
    fn rank0<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.as_ref().rank0(r)
    }

    #[inline]
    fn select1(&self, n: u64) -> Option<u64> {
        self.as_ref().select1(n)
    }

    #[inline]
    fn select0(&self, n: u64) -> Option<u64> {
        self.as_ref().select0(n)
    }
}

impl<T> Bits for Cow<'_, T>
where
    T: ?Sized + ToOwned + Bits,
{
    #[inline]
    fn bits(&self) -> u64 {
        self.as_ref().bits()
    }

    #[inline]
    fn count1(&self) -> u64 {
        self.as_ref().count1()
    }

    #[inline]
    fn count0(&self) -> u64 {
        self.as_ref().count0()
    }

    #[inline]
    fn all(&self) -> bool {
        self.as_ref().all()
    }

    #[inline]
    fn any(&self) -> bool {
        self.as_ref().any()
    }

    #[inline]
    fn bit(&self, i: u64) -> bool {
        self.as_ref().bit(i)
    }

    #[inline]
    fn word<B: Word>(&self, i: u64, len: u64) -> B {
        self.as_ref().word(i, len)
    }

    #[inline]
    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.as_ref().rank1(r)
    }

    #[inline]
    fn rank0<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.as_ref().rank0(r)
    }

    #[inline]
    fn select1(&self, n: u64) -> Option<u64> {
        self.as_ref().select1(n)
    }

    #[inline]
    fn select0(&self, n: u64) -> Option<u64> {
        self.as_ref().select0(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn next_multiple_of() {
        use super::next_multiple_of;
        assert_eq!(next_multiple_of(0, 8), 8);
        assert_eq!(next_multiple_of(12, 3), 15);
        assert_eq!(next_multiple_of(16, 8), 24);
        assert_eq!(next_multiple_of(23, 8), 24);
        assert_eq!(next_multiple_of(9, 3), 12);
    }
}
