//! Provides bits::Block impls.

use std::ops::RangeBounds;

use crate::{
    Bits,
    BitsMut,
    Block,
    Masking,
    Word,
};

mod private {
    use crate::Word;

    pub trait Repr: Sized {}

    impl<B: Word, const N: usize> Repr for [B; N] {}
}

/// Buf is a boxed array of `Word` and can be used as a fixed-size bit block.
#[derive(Debug, Clone, Default)]
pub struct Buf<A: private::Repr>(
    // Note that `None` does not imply empty bits. `None` is
    // a bit sequence all set to 0.
    //
    // Null Pointer Optimization:
    // https://doc.rust-lang.org/std/option/#representation
    Option<Box<A>>,
);

impl<B: Word, const N: usize> Buf<[B; N]> {
    pub fn new() -> Self {
        Buf(None)
    }

    // TODO: const fn when as_deref and as_deref_mut is const fn.
    // https://github.com/rust-lang/rust/issues/14387

    #[inline]
    pub fn as_ref(&self) -> Option<&[B]> {
        self.0.as_deref().map(|a| a.as_slice())
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut [B]> {
        self.0.as_deref_mut().map(|a| a.as_mut_slice())
    }

    #[inline]
    fn or_empty(&mut self) -> &mut [B; N] {
        self.0.get_or_insert_with(|| Box::new([B::empty(); N]))
    }
}

impl<B: Word, const N: usize> From<[B; N]> for Buf<[B; N]> {
    fn from(array: [B; N]) -> Self {
        Buf(Some(Box::new(array)))
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

impl<B: Word, const N: usize> Bits for Buf<[B; N]> {
    #[inline]
    fn bits(&self) -> u64 {
        <[B; N]>::BITS
    }

    #[inline]
    fn count1(&self) -> u64 {
        self.0.as_ref().map_or(0, Bits::count1)
    }
    #[inline]
    fn count0(&self) -> u64 {
        self.0.as_ref().map_or(<[B; N]>::BITS, Bits::count0)
    }

    #[inline]
    fn all(&self) -> bool {
        self.0.as_ref().is_some_and(Bits::all)
    }

    #[inline]
    fn any(&self) -> bool {
        self.0.as_ref().is_some_and(Bits::any)
    }

    #[inline]
    fn bit(&self, i: u64) -> bool {
        self.0.as_ref().is_some_and(|b| b.bit(i))
    }

    #[inline]
    fn word<T: Word>(&self, i: u64, len: u64) -> T {
        self.0.as_ref().map_or(T::empty(), |b| b.word(i, len))
    }

    #[inline]
    fn rank1<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        self.0.as_ref().map_or(0, |b| b.rank1(r))
    }
    #[inline]
    fn rank0<R: RangeBounds<u64>>(&self, r: R) -> u64 {
        let (i, j) = crate::range(&r, 0, self.bits());
        self.0.as_ref().map_or(j - i, |b| b.rank0(r))
    }

    #[inline]
    fn select1(&self, n: u64) -> Option<u64> {
        self.0.as_ref().and_then(|b| b.select1(n))
    }
    #[inline]
    fn select0(&self, n: u64) -> Option<u64> {
        (n < self.count0()).then(|| self.0.as_ref().map_or(Some(n), |b| b.select0(n))).flatten()
    }
}

impl<B: Word, const N: usize> BitsMut for Buf<[B; N]> {
    #[inline]
    fn set1(&mut self, i: u64) {
        self.or_empty().set1(i)
    }

    #[inline]
    fn set0(&mut self, i: u64) {
        self.or_empty().set0(i)
    }
}

impl<B: Word, const N: usize> Block for Buf<[B; N]> {
    const BITS: u64 = <[B; N]>::BITS;

    #[inline]
    fn empty() -> Self {
        Buf::new()
    }
}

impl<const N: usize> Masking<Self> for Buf<[u64; N]> {
    fn intersection(&mut self, that: &Self) {
        match (self.as_mut(), that.as_ref()) {
            (Some(this), Some(that)) => {
                for (a, b) in this.iter_mut().zip(that) {
                    *a &= *b;
                }
            }
            (Some(_), None) => {
                *self = Buf::new();
            }
            _ => {}
        }
    }

    #[inline]
    fn union(&mut self, that: &Self) {
        match (self.as_mut(), that.as_ref()) {
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

    #[inline]
    fn difference(&mut self, that: &Self) {
        if let (Some(this), Some(that)) = (self.as_mut(), that.as_ref()) {
            for (a, b) in this.iter_mut().zip(that) {
                *a &= !*b;
            }
        }
    }

    #[inline]
    fn symmetric_difference(&mut self, that: &Self) {
        match (self.as_mut(), that.as_ref()) {
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
