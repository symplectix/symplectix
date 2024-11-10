use std::ops::RangeBounds;

use bits_core::{Bits, BitsMut, Block, Word};

#[derive(Debug, Default, Clone)]
pub struct BoxContainer<T>(Option<Box<T>>);

impl<B: Word, const N: usize> BoxContainer<[B; N]> {
    fn inner(&self) -> Option<&[B; N]> {
        self.0.as_deref()
    }

    // fn inner_mut(&mut self) -> Option<&mut Box<[B; N]>> {
    //     self.0.as_mut()
    // }

    fn or_empty(&mut self) -> &mut [B] {
        self.0.get_or_insert_with(|| Box::new([B::empty(); N])).as_mut_slice()
    }
}

impl<B: Word, const N: usize> Bits for BoxContainer<[B; N]> {
    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut b = roaring_block::BoxContainer::<[u64; 8]>::default();
    /// assert_eq!(b.bits(), 512);
    ///
    /// b.set1(100);
    /// assert_eq!(b.count1(), 1);
    /// assert_eq!(b.count0(), 511);
    /// ```
    #[inline]
    fn bits(&self) -> usize {
        // self.0.as_ref().map_or(0, |b| b.bits())
        <u64 as Block>::BITS * N
    }

    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        self.inner().and_then(|b| b.test(i))
    }

    #[inline]
    fn count1(&self) -> usize {
        self.inner().map_or(0, |b| b.count1())
    }

    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        self.inner().map_or(0, |b| b.rank1(r))
    }

    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        self.inner().and_then(|b| b.select1(n))
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut b = roaring_block::BoxContainer::<[u64; 8]>::default();
    /// assert_eq!(b.select1(0), None);
    /// assert_eq!(b.select0(0), Some(0));
    /// assert_eq!(b.select0(b.bits()-1), Some(511));
    ///
    /// b.set1(1);
    /// b.set1(511);
    /// assert_eq!(b.select1(0), Some(1));
    /// assert_eq!(b.select1(1), Some(511));
    /// assert_eq!(b.select0(0), Some(0));
    /// assert_eq!(b.select0(1), Some(2));
    /// ```
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        match self.inner() {
            Some(b) => b.select0(n),
            // self.count0() == Self::BITS
            None => (n < Self::BITS).then_some(n),
        }
    }
}

impl<B: Word, const N: usize> BitsMut for BoxContainer<[B; N]> {
    #[inline]
    fn set1(&mut self, i: usize) {
        assert!(i < self.bits());
        self.or_empty().set1(i);
    }

    #[inline]
    fn set0(&mut self, i: usize) {
        assert!(i < self.bits());
        self.or_empty().set0(i);
    }
}

impl<B: Word, const N: usize> Block for BoxContainer<[B; N]> {
    const BITS: usize = <[B; N]>::BITS;
    #[inline]
    fn empty() -> Self {
        BoxContainer(None)
    }
}
