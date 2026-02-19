//! Provides bits::Block impls.

use std::convert::{
    AsMut,
    AsRef,
};
use std::ops::RangeBounds;

use crate::{
    Bits,
    BitsMut,
    Block,
    Difference,
    Intersection,
    SymmetricDifference,
    Union,
    Word,
};

mod private {
    use crate::Word;

    pub trait Repr: Sized {}

    impl<B: Word, const N: usize> Repr for [B; N] {}
}

/// An array of `Word`s which can be used as a fixed size of bits block.
#[derive(Debug, Clone, Default)]
pub struct Array<A: private::Repr>(
    // Note that `None` does not imply empty bits. `None` is
    // a bit sequence all set to 0.
    //
    // Null Pointer Optimization:
    // https://doc.rust-lang.org/std/option/#representation
    Option<Box<A>>,
);

impl<B: Word, const N: usize> Array<[B; N]> {
    pub fn new() -> Self {
        Array(None)
    }

    #[inline]
    pub fn as_slice(&self) -> Option<&[B]> {
        self.0.as_deref().map(|a| a.as_slice())
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> Option<&mut [B]> {
        self.0.as_deref_mut().map(|a| a.as_mut_slice())
    }

    #[inline]
    fn or_empty(&mut self) -> &mut [B; N] {
        self.0.get_or_insert_with(|| Box::new([B::empty(); N]))
    }
}

impl<B: Word, const N: usize> From<[B; N]> for Array<[B; N]> {
    fn from(array: [B; N]) -> Self {
        Array(Some(Box::new(array)))
    }
}

// impl<B: Word, const N: usize> Block for Buf<[B; N]> {
//     const BITS: usize = <[B; N]>::BITS;

//     /// # Tests
//     ///
//     /// ```
//     /// # use bits::block::*;
//     /// # use bits_buf::Buf;
//     /// let mut b = Buf::<[u32; 16]>::empty();
//     /// assert_eq!(Buf::<[u32; 16]>::BITS, 512);
//     ///
//     /// b.set1(100);
//     /// assert_eq!(b.count1(), 1);
//     /// assert_eq!(b.count0(), 511);
//     /// ```
//     #[inline]
//     fn empty() -> Self {
//         Buf(None)
//     }

//     #[inline]
//     fn test(&self, i: usize) -> Option<bool> {
//         self.inner().and_then(|b| b.test(i))
//     }
// }
// impl<B: Word, const N: usize> BlockMut for Buf<[B; N]> {
//     #[inline]
//     fn set1(&mut self, i: usize) {
//         assert!(i < Self::BITS);
//         self.or_empty().set1(i);
//     }

//     #[inline]
//     fn set0(&mut self, i: usize) {
//         assert!(i < Self::BITS);
//         self.or_empty().set0(i);
//     }
// }

// impl<B: Word, const N: usize> Count for Buf<[B; N]> {
//     #[inline]
//     fn count1(&self) -> usize {
//         self.inner().map_or(0, |b| b.count1())
//     }
// }

// impl<B: Word, const N: usize> Rank for Buf<[B; N]> {
//     #[inline]
//     fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
//         self.inner().map_or(0, |b| b.rank1(r))
//     }
// }

// impl<B: Word, const N: usize> Select for Buf<[B; N]> {
//     #[inline]
//     fn select1(&self, n: usize) -> Option<usize> {
//         self.inner().and_then(|b| b.select1(n))
//     }

//     /// # Tests
//     ///
//     /// ```
//     /// # use bits::block::*;
//     /// # use bits_buf::Buf;
//     /// let mut b = Buf::<[u64; 8]>::empty();
//     /// assert_eq!(b.select1(0), None);
//     /// assert_eq!(b.select0(0), Some(0));
//     /// assert_eq!(b.select0(Buf::<[u64; 8]>::BITS - 1), Some(511));
//     ///
//     /// b.set1(1);
//     /// b.set1(511);
//     /// assert_eq!(b.select1(0), Some(1));
//     /// assert_eq!(b.select1(1), Some(511));
//     /// assert_eq!(b.select0(0), Some(0));
//     /// assert_eq!(b.select0(1), Some(2));
//     /// ```
//     #[inline]
//     fn select0(&self, n: usize) -> Option<usize> {
//         match self.inner() {
//             Some(b) => b.select0(n),
//             // self.count0() == Self::BITS
//             None => (n < Self::BITS).then_some(n),
//         }
//     }
// }

impl<B: Word, const N: usize> Bits for Array<[B; N]> {
    #[inline]
    fn bits(&self) -> u64 {
        <[B; N]>::BITS
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert_eq!(b.count1(), 0);
    /// let b: Array<[u8; 3]> = Array::from([0, 1, 0]);
    /// assert_eq!(b.count1(), 1);
    /// ```
    #[inline]
    fn count1(&self) -> u64 {
        self.0.as_ref().map_or(0, Bits::count1)
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert_eq!(b.count0(), 24);
    /// let b: Array<[u8; 3]> = Array::from([0, 0, 0]);
    /// assert_eq!(b.count0(), 24);
    /// ```
    #[inline]
    fn count0(&self) -> u64 {
        self.0.as_ref().map_or(<[B; N]>::BITS, Bits::count0)
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert!(!b.all());
    /// let b: Array<[u8; 3]> = Array::from([!0, !0, !0]);
    /// assert!(b.all());
    /// ```
    #[inline]
    fn all(&self) -> bool {
        self.0.as_ref().is_some_and(Bits::all)
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert!(!b.any());
    /// let b: Array<[u8; 3]> = Array::from([0, 0, 0]);
    /// assert!(!b.any());
    /// let b: Array<[u8; 3]> = Array::from([0, 1, 0]);
    /// assert!(b.any());
    /// ```
    #[inline]
    fn any(&self) -> bool {
        self.0.as_ref().is_some_and(Bits::any)
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert!(!b.bit(8));
    /// let b: Array<[u8; 3]> = Array::from([0, 1, 0]);
    /// assert!(b.bit(8));
    /// ```
    #[inline]
    fn bit(&self, i: u64) -> bool {
        self.0.as_ref().is_some_and(|b| b.bit(i))
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert_eq!(b.word::<u8>(0, 3), 0b_000);
    /// let b: Array<[u8; 3]> = Array::from([1, 1, 1]);
    /// assert_eq!(b.word::<u8>(0, 3), 0b_001);
    /// ```
    #[inline]
    fn word<T: Word>(&self, i: u64, len: u64) -> T {
        self.0.as_ref().map_or(T::empty(), |b| b.word(i, len))
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert_eq!(b.rank1(..10), 0);
    /// let b: Array<[u8; 3]> = Array::from([0, 1, 0]);
    /// assert_eq!(b.rank1(..10), 1);
    /// ```
    #[inline]
    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.0.as_ref().map_or(0, |b| b.rank1(r))
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert_eq!(b.rank0(..10), 10);
    /// let b: Array<[u8; 3]> = Array::from([0, 1, 0]);
    /// assert_eq!(b.rank0(..10), 9);
    /// ```
    #[inline]
    fn rank0<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        let (i, j) = crate::range(&r, 0, self.bits());
        self.0.as_ref().map_or(j - i, |b| b.rank0(r))
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert_eq!(b.select1(0), None);
    /// let b: Array<[u8; 3]> = Array::from([0, 1, 0]);
    /// assert_eq!(b.select1(0), Some(8));
    /// ```
    #[inline]
    fn select1(&self, n: u64) -> Option<u64> {
        self.0.as_ref().and_then(|b| b.select1(n))
    }

    /// ```
    /// # use bitop::{Array, Bits};
    /// let b: Array<[u8; 3]> = Array::new();
    /// assert_eq!(b.select0(0), Some(0));
    /// assert_eq!(b.select0(100), None);
    ///
    /// let b: Array<[u8; 3]> = Array::from([0, 1, 0]);
    /// assert_eq!(b.select0(10), Some(11));
    /// ```
    #[inline]
    fn select0(&self, n: u64) -> Option<u64> {
        (n < self.count0()).then(|| self.0.as_ref().map_or(Some(n), |b| b.select0(n))).flatten()
    }
}

impl<B: Word, const N: usize> BitsMut for Array<[B; N]> {
    #[inline]
    fn set1(&mut self, i: u64) {
        self.or_empty().set1(i)
    }

    #[inline]
    fn set0(&mut self, i: u64) {
        self.or_empty().set0(i)
    }
}

impl<B: Word, const N: usize> Block for Array<[B; N]> {
    const BITS: u64 = <[B; N]>::BITS;

    #[inline]
    fn empty() -> Self {
        Array::new()
    }
}

impl<const N: usize> Intersection<Self> for Array<[u64; N]> {
    /// # Tests
    ///
    /// ```
    /// # use bitop::{Array, BitsMut, Intersection};
    /// let mut a = Array::<[u64; 4]>::new();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(128);
    ///
    /// let mut b = Array::<[u64; 4]>::new();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(192);
    ///
    /// a.intersection(&b);
    /// assert_eq!(a.as_slice().unwrap(), &[0b_0110, 0, 0, 0]);
    ///
    /// let mut c = Array::<[u64; 4]>::new();
    /// b.intersection(&c);
    /// assert_eq!(b.as_slice(), None);
    ///
    /// c.intersection(&a);
    /// assert_eq!(c.as_slice(), None);
    /// ```
    fn intersection(&mut self, that: &Self) {
        match (self.as_mut_slice(), that.as_slice()) {
            (Some(this), Some(that)) => {
                for (a, b) in this.iter_mut().zip(that) {
                    *a &= *b;
                }
            }
            (Some(_), None) => {
                *self = Array::new();
            }
            _ => {}
        }
    }
}

impl<const N: usize> Union<Self> for Array<[u64; N]> {
    /// # Tests
    ///
    /// ```
    /// # use bitop::{Array, BitsMut, Union};
    /// let mut a = Array::<[u64; 4]>::new();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(128);
    ///
    /// let mut b = Array::<[u64; 4]>::new();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(192);
    ///
    /// a.union(&b);
    /// assert_eq!(a.as_slice().unwrap(), &[0b_1111, 0, 1, 1]);
    ///
    /// let mut c = Array::<[u64; 4]>::new();
    /// c.union(&b);
    /// assert_eq!(c.as_slice().unwrap(), &[0b_1110, 0, 0, 1]);
    /// ```
    #[inline]
    fn union(&mut self, that: &Self) {
        match (self.as_mut_slice(), that.as_slice()) {
            (Some(this), Some(that)) => {
                for (a, b) in this.iter_mut().zip(that) {
                    *a |= *b;
                }
            }
            (None, Some(that)) => {
                self.or_empty().copy_from_slice(that);
            }
            _ => {}
        }
    }
}

impl<const N: usize> Difference<Self> for Array<[u64; N]> {
    /// # Tests
    ///
    /// ```
    /// # use bitop::{Array, BitsMut, Difference};
    /// let mut a = Array::<[u64; 4]>::new();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(128);
    ///
    /// let mut b = Array::<[u64; 4]>::new();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(192);
    ///
    /// a.difference(&b);
    /// assert_eq!(a.as_slice().unwrap(), &[0b_0001, 0, 1, 0]);
    ///
    /// let mut c = Array::<[u64; 4]>::new();
    /// c.difference(&a);
    /// assert_eq!(c.as_slice(), None);
    /// ```
    #[inline]
    fn difference(&mut self, that: &Self) {
        if let (Some(this), Some(that)) = (self.as_mut_slice(), that.as_slice()) {
            for (a, b) in this.iter_mut().zip(that) {
                *a &= !*b;
            }
        }
    }
}

impl<const N: usize> SymmetricDifference<Self> for Array<[u64; N]> {
    /// # Tests
    ///
    /// ```
    /// # use bitop::{Array, BitsMut, SymmetricDifference};
    /// let mut a = Array::<[u64; 4]>::new();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    /// a.set1(128);
    ///
    /// let mut b = Array::<[u64; 4]>::new();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    /// b.set1(192);
    ///
    /// a.symmetric_difference(&b);
    /// assert_eq!(a.as_slice().unwrap(), &[0b_1001, 0, 1, 1]);
    ///
    /// let mut c = Array::<[u64; 4]>::new();
    /// c.symmetric_difference(&a);
    /// assert_eq!(c.as_slice().unwrap(), &[0b_1001, 0, 1, 1]);
    /// ```
    #[inline]
    fn symmetric_difference(&mut self, that: &Self) {
        match (self.as_mut_slice(), that.as_slice()) {
            (Some(this), Some(that)) => {
                for (a, b) in this.iter_mut().zip(that) {
                    *a ^= *b;
                }
            }
            (None, Some(that)) => {
                self.or_empty().copy_from_slice(that);
            }
            _ => {}
        }
    }
}
