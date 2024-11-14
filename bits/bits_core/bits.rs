use std::borrow::ToOwned;
use std::mem;
use std::ops::{Range, RangeBounds};

use crate::block::{BlockMut, Count, Excess, Pack, Rank, Select};
use crate::{BitVec, Block, Word};

#[derive(Hash, Debug)]
#[repr(transparent)]
pub struct Bits<T> {
    pub(crate) data: [T],
}

impl<T: Clone> ToOwned for Bits<T> {
    type Owned = BitVec<T>;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        BitVec { data: self.data.to_vec() }
    }
}

impl<T> Bits<T> {
    #[inline]
    pub(crate) fn from_slice(slice: &[T]) -> &Bits<T> {
        unsafe { mem::transmute(slice) }
    }

    #[inline]
    pub(crate) fn from_slice_mut(slice: &mut [T]) -> &mut Bits<T> {
        unsafe { mem::transmute(slice) }
    }

    pub fn new<A>(data: &A) -> &Bits<T>
    where
        A: ?Sized + AsRef<[T]>,
    {
        Bits::from_slice(data.as_ref())
    }

    pub fn new_mut<A>(data: &mut A) -> &mut Bits<T>
    where
        A: ?Sized + AsMut<[T]>,
    {
        Bits::from_slice_mut(data.as_mut())
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    #[inline]
    pub fn copy_from_slice(&mut self, that: &Bits<T>)
    where
        T: Copy,
    {
        self.data.copy_from_slice(&that.data)
    }

    pub fn into_vec(self: Box<Bits<T>>) -> BitVec<T> {
        BitVec {
            data: unsafe {
                let len = self.data.len();
                let ptr = Box::into_raw(self) as *mut [T] as *mut T;
                Vec::from_raw_parts(ptr, len, len)
            },
        }
    }
}

impl<T: Block> Bits<T> {
    /// Returns the number of bits.
    ///
    /// # Tests
    ///
    /// ```
    /// # use bits_core::Bits;
    /// let v: &Bits<u8> = Bits::new(&[0, 0, 0]);
    /// let w: &Bits<u8> = Bits::new(&[]);
    /// assert_eq!(v.bits(), 24);
    /// assert_eq!(w.bits(), 0);
    /// ```
    #[inline]
    pub const fn bits(&self) -> usize {
        T::BITS * self.data.len()
    }

    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **None**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits_core::Bits;
    /// let v: &Bits<u64> = Bits::new(&[0b00000101, 0b01100011, 0b01100000]);
    /// assert_eq!(v.test(0),   Some(true));
    /// assert_eq!(v.test(64),  Some(true));
    /// assert_eq!(v.test(128), Some(false));
    /// assert_eq!(v.test(200), None);
    /// ```
    #[inline]
    pub fn test(&self, i: usize) -> Option<bool> {
        let (i, o) = bit::addr(i, T::BITS);
        self.data.get(i).map(|b| b.test(o).expect("index out of bounds"))
    }
}

impl<T: BlockMut> Bits<T> {
    /// Enables the bit at the given index `i`.
    #[inline]
    pub fn set1(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = bit::addr(i, T::BITS);
        self.data[i].set1(o)
    }

    /// Disables the bit at the given index `i`.
    #[inline]
    pub fn set0(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = bit::addr(i, T::BITS);
        self.data[i].set0(o)
    }
}

impl<T: Block + Count> Bits<T> {
    /// Counts the occurrences of `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits_core::Bits;
    /// let a: &Bits<u64> = Bits::new(&[]);
    /// let b: &Bits<u64> = Bits::new(&[0, 0, 0]);
    /// let c: &Bits<u64> = Bits::new(&[0, 1, 3]);
    /// assert_eq!(a.count1(), 0);
    /// assert_eq!(b.count1(), 0);
    /// assert_eq!(c.count1(), 3);
    /// ```
    #[inline]
    pub fn count1(&self) -> usize {
        self.data.iter().map(|b| b.count1()).sum()
    }

    /// Counts the occurrences of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits_core::Bits;
    /// let a: &Bits<u64> = Bits::new(&[]);
    /// let b: &Bits<u64> = Bits::new(&[0, 0, 0]);
    /// let c: &Bits<u64> = Bits::new(&[0, 1, 3]);
    /// assert_eq!(a.count0(), 0);
    /// assert_eq!(b.count0(), 192);
    /// assert_eq!(c.count0(), 189);
    /// ```
    #[inline]
    pub fn count0(&self) -> usize {
        self.data.iter().map(|b| b.count0()).sum()
    }

    /// Returns true if all bits are enabled. An empty bits should return true.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits_core::Bits;
    /// let a: &Bits<u64> = Bits::new(&[0, 0, 0]);
    /// let b: &Bits<u64> = Bits::new(&[]);
    /// let c: &Bits<u64> = Bits::new(&[!0, !0, !0]);
    /// assert!(!a.all());
    /// assert!( b.all());
    /// assert!( c.all());
    /// ```
    #[inline]
    pub fn all(&self) -> bool {
        self.data.iter().all(|b| b.all())
    }

    /// Returns true if any bits are enabled. An empty bits should return false.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits_core::Bits;
    /// let b1: &Bits<u64> = Bits::new(&[]);
    /// let b2: &Bits<u64> = Bits::new(&[0, 0, 0]);
    /// let b3: &Bits<u64> = Bits::new(&[!0, !0, !0]);
    /// let b4: &Bits<u64> = Bits::new(&[0, 0, 1]);
    /// assert!(!b1.any());
    /// assert!(!b2.any());
    /// assert!( b3.any());
    /// assert!( b4.any());
    /// ```
    #[inline]
    pub fn any(&self) -> bool {
        self.data.iter().any(|b| b.any())
    }
}

impl<T: Block + Rank> Bits<T> {
    /// Counts occurrences of `1` in the given range.
    pub fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let Range { start, end } = bit::bounded(&r, 0, self.bits());

        // TODO: benchmark
        // bit::chunks(start, end, B::BITS)
        //     .map(|(index, len)| {
        //         let (i, p) = bit::addr(index, B::BITS);
        //         self.get(i)
        //             .map_or(0, |b| if len == B::BITS { b.count1() } else { b.rank1(p..p + len) })
        //     })
        //     .sum()

        if self.data.is_empty() {
            return 0;
        }
        let (i, p) = bit::addr(start, T::BITS);
        let (j, q) = bit::addr(end, T::BITS);
        if i == j {
            self.data[i].rank1(p..q)
        } else {
            self.data[i].rank1(p..)
                + Bits::new(&self.data[i + 1..j]).count1()
                + self.data.get(j).map_or(0, |b| b.rank1(..q))
        }
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    pub fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let r = bit::bounded(&r, 0, self.bits());
        r.len() - self.rank1(r)
    }
}

impl<T: Block + Excess> Bits<T> {
    fn ranks<R: RangeBounds<usize>>(&self, r: R) -> (usize, usize) {
        let r = bit::bounded(&r, 0, self.bits());
        let len = r.len();
        let rank1 = self.rank1(r);
        let rank0 = len - rank1;
        (rank0, rank1)
    }

    #[inline]
    pub fn excess1<R: RangeBounds<usize>>(&self, r: R) -> Option<usize> {
        let (rank0, rank1) = self.ranks(r);
        rank1.checked_sub(rank0)
    }

    #[inline]
    pub fn excess0<R: RangeBounds<usize>>(&self, r: R) -> Option<usize> {
        let (rank0, rank1) = self.ranks(r);
        rank0.checked_sub(rank1)
    }
}

impl<T: Block + Select> Bits<T> {
    /// Returns the position of the n-th 1, indexed starting from zero.
    /// `n` must be less than `self.count1()`, otherwise returns `None`.
    pub fn select1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.data.iter().enumerate() {
            let count = b.count1();
            if n < count {
                return Some(i * T::BITS + b.select1(n).expect("select1(n) must be ok"));
            }
            n -= count;
        }
        None
    }

    /// Returns the position of the n-th 0, indexed starting from zero.
    /// `n` must be less than `self.count0()`, otherwise returns `None`.
    pub fn select0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.data.iter().enumerate() {
            let count = b.count0();
            if n < count {
                return Some(i * T::BITS + b.select0(n).expect("select0(n) must be ok"));
            }
            n -= count;
        }
        None
    }
}

fn range_over<T: Block>(s: usize, e: usize, mut f: impl FnMut(usize, usize, usize) -> bool) {
    assert!(s <= e);
    if s == e {
        return;
    }

    let (q0, r0) = bit::addr(s, T::BITS);
    let (q1, r1) = bit::addr(e, T::BITS);

    if q0 == q1 {
        f(q0, r0, r1);
        return;
    }

    use std::iter::{once, repeat};
    for (index, cur) in (q0..q1).zip(once(r0).chain(repeat(0))) {
        if !f(index, cur, T::BITS) {
            return;
        }
    }

    f(q1, 0, r1);
}

impl<B: Block + Pack> Bits<B> {
    /// Writes `n` bits of the given to `[i, i+n)`.
    pub fn pack<T: Word>(&mut self, i: usize, n: usize, bits: T) {
        let mut cur = 0;
        range_over::<B>(i, i + n, |idx, s, e| {
            (idx < self.data.len())
                .then(|| {
                    let len = e - s;
                    self.data[idx].pack::<T>(s, len, bits.unpack(cur, len));
                    cur += len;
                })
                .is_some()
        });
    }

    /// Reads `n` bits from `i`, and returns it as the lowest `n` bits of `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits_core::Bits;
    /// let bits: &Bits<u16> = Bits::new(&[0b_1101_0001_1010_0011, 0b_1001_1110_1110_1001]);
    /// let len = 4;
    /// assert_eq!(bits.unpack::<u8>(0,  len), 0b0011);
    /// assert_eq!(bits.unpack::<u8>(8,  len), 0b0001);
    /// assert_eq!(bits.unpack::<u8>(14, len), 0b0111);
    /// assert_eq!(bits.unpack::<u8>(30, len), 0b0010);
    /// ```
    pub fn unpack<T: Word>(&self, i: usize, n: usize) -> T {
        let mut cur = 0;
        let mut out = T::empty();
        range_over::<B>(i, i + n, |idx, s, e| {
            (idx < self.data.len() && cur < T::BITS)
                .then(|| {
                    let len = e - s;
                    out |= self.data[idx].unpack::<T>(s, len) << cur;
                    cur += len;
                })
                .is_some()
        });
        out
    }
}
