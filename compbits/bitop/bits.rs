use std::borrow::Cow;
use std::ops::RangeBounds;

use crate::Word;

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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.rank1(..), v.count1());
    /// assert_eq!(v.rank1(..), 8);
    /// assert_eq!(v.rank1(1..), 7);
    /// assert_eq!(v.rank1(..2), 1);
    /// assert_eq!(v.rank1(60..70), 3);
    /// ```
    ///
    /// ```
    /// # use bitop::Bits;
    /// let v: &[u64] = &[];
    /// assert_eq!(v.rank1(..), 0);
    /// ```
    ///
    /// ```should_panic
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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
    /// # use bitop::Bits;
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

/// A mutatable bit sequence.
pub trait BitsMut: Bits {
    /// Set a bit at `i`.
    fn set1(&mut self, i: u64);

    /// Unset a bit at `i`.
    fn set0(&mut self, i: u64);
}

/// Fixed sized bits.
pub trait Block: Clone + Bits + BitsMut {
    /// The number of bits, which must always be equal to `Bits::bits`.
    const BITS: u64;

    /// Constructs an empty bits block.
    fn empty() -> Self;
}

/// Helper trait for blockwise iteration.
pub trait IntoBlocks: Sized {
    /// Type of a bit container.
    type Block;

    /// An iterator which yields `Block`s with its index.
    type Blocks: Iterator<Item = (usize, Self::Block)>;

    /// Returns an iterator.
    fn into_blocks(self) -> Self::Blocks;
}

/// Helper trait for blockwise iteration.
pub trait FromBlocks<B>: Sized {
    /// Constructs a value from blocks.
    fn from_blocks<T: IntoBlocks<Block = B>>(iter: T) -> Self;
}

impl<B: Block> Bits for [B] {
    #[inline]
    fn bits(&self) -> u64 {
        B::BITS * self.len() as u64
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
        let (i, o) = crate::index(i, B::BITS);
        self.get(i).is_some_and(|t| t.bit(o))
    }

    fn word<T: Word>(&self, i: u64, len: u64) -> T {
        debug_assert!(i < self.bits() && len <= T::BITS);
        let (s, p) = crate::index(i, B::BITS);
        let (e, q) = crate::index(i + len, B::BITS);
        if s == e {
            self[s].word(p, q - p)
        } else {
            let mut cur = 0;
            let mut out = T::empty();
            out |= self[s].word::<T>(p, B::BITS - p) << (cur as usize);
            cur += B::BITS - p;
            for block in self.iter().take(e).skip(s + 1) {
                out |= block.word::<T>(0, B::BITS) << (cur as usize);
                cur += B::BITS;
            }
            if e < self.len() {
                out |= self[e].word::<T>(0, q) << (cur as usize);
            }
            out
        }
    }

    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        match crate::range(&r, 0, self.bits()) {
            (0, j) => {
                let (j, q) = crate::index(j, B::BITS);
                self[..j].count1() + self.get(j).map_or(0, |p| p.rank1(..q))
            }
            (i, j) => {
                let (i, p) = crate::index(i, B::BITS);
                let (j, q) = crate::index(j, B::BITS);
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
                return Some(i * B::BITS + b.select1(n).expect("bug"));
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
                return Some(i * B::BITS + b.select0(n).expect("bug"));
            }
            n -= count;
        }
        None
    }
}

impl<B: Block> BitsMut for [B] {
    #[inline]
    fn set1(&mut self, i: u64) {
        let (i, o) = crate::index(i, B::BITS);
        self[i].set1(o)
    }
    #[inline]
    fn set0(&mut self, i: u64) {
        let (i, o) = crate::index(i, B::BITS);
        self[i].set0(o)
    }
}

impl<'a, B: Block> IntoBlocks for &'a [B] {
    type Block = Cow<'a, B>;
    type Blocks = slice::Blocks<'a, B>;
    fn into_blocks(self) -> Self::Blocks {
        slice::Blocks { blocks: self.iter().enumerate() }
    }
}

mod slice {
    use std::borrow::Cow;
    use std::iter::Enumerate;

    use crate::Block;

    pub struct Blocks<'a, B> {
        pub(crate) blocks: Enumerate<std::slice::Iter<'a, B>>,
    }
    impl<'a, B: Block> Iterator for Blocks<'a, B> {
        type Item = (usize, Cow<'a, B>);
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.blocks.find_map(|(i, b)| b.any().then(|| (i, Cow::Borrowed(b))))
        }
    }
}

impl<B: Block, const N: usize> Bits for [B; N] {
    #[inline]
    fn bits(&self) -> u64 {
        B::BITS * N as u64
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
    fn word<T: Word>(&self, i: u64, len: u64) -> T {
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
impl<B: Block, const N: usize> BitsMut for [B; N] {
    #[inline]
    fn set1(&mut self, i: u64) {
        self.as_mut_slice().set1(i)
    }
    #[inline]
    fn set0(&mut self, i: u64) {
        self.as_mut_slice().set0(i)
    }
}
impl<B: Word, const N: usize> Block for [B; N] {
    const BITS: u64 = B::BITS * N as u64;
    #[inline]
    fn empty() -> Self {
        [B::empty(); N]
    }
}
impl<'a, B, const N: usize> IntoBlocks for &'a [B; N]
where
    &'a [B]: IntoBlocks,
{
    type Block = <&'a [B] as IntoBlocks>::Block;
    type Blocks = <&'a [B] as IntoBlocks>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        self.as_ref().into_blocks()
    }
}

impl<B: Block> Bits for Vec<B> {
    #[inline]
    fn bits(&self) -> u64 {
        B::BITS * self.as_slice().len() as u64
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
    fn word<T: Word>(&self, i: u64, len: u64) -> T {
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
impl<B: Block> BitsMut for Vec<B> {
    #[inline]
    fn set1(&mut self, i: u64) {
        self.as_mut_slice().set1(i)
    }
    #[inline]
    fn set0(&mut self, i: u64) {
        self.as_mut_slice().set0(i)
    }
}
impl<'a, B: Block> IntoBlocks for &'a Vec<B> {
    type Block = <&'a [B] as IntoBlocks>::Block;
    type Blocks = <&'a [B] as IntoBlocks>::Blocks;
    fn into_blocks(self) -> Self::Blocks {
        self.as_slice().into_blocks()
    }
}

impl<B: Bits> Bits for Box<B> {
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
    fn word<T: Word>(&self, i: u64, len: u64) -> T {
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
impl<B: BitsMut> BitsMut for Box<B> {
    #[inline]
    fn set1(&mut self, i: u64) {
        self.as_mut().set1(i)
    }
    #[inline]
    fn set0(&mut self, i: u64) {
        self.as_mut().set0(i)
    }
}
impl<B: Block> Block for Box<B> {
    const BITS: u64 = B::BITS;
    #[inline]
    fn empty() -> Self {
        Box::new(B::empty())
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
    fn word<W: Word>(&self, i: u64, len: u64) -> W {
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
impl<T, B> BitsMut for Cow<'_, T>
where
    T: ?Sized + ToOwned<Owned = B> + Bits,
    B: BitsMut,
{
    #[inline]
    fn set1(&mut self, i: u64) {
        self.to_mut().set1(i)
    }
    #[inline]
    fn set0(&mut self, i: u64) {
        self.to_mut().set0(i)
    }
}
impl<T, B> Block for Cow<'_, T>
where
    T: ?Sized + ToOwned<Owned = B> + Bits,
    B: Block,
{
    const BITS: u64 = B::BITS;
    #[inline]
    fn empty() -> Self {
        Cow::Owned(B::empty())
    }
}

impl<'inner, T: ?Sized> IntoBlocks for &&'inner T
where
    &'inner T: IntoBlocks,
{
    type Block = <&'inner T as IntoBlocks>::Block;
    type Blocks = <&'inner T as IntoBlocks>::Blocks;
    #[inline]
    fn into_blocks(self) -> Self::Blocks {
        IntoBlocks::into_blocks(*self)
    }
}
