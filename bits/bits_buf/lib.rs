use std::ops::RangeBounds;

use bits_core::block::*;
use bits_core::mask::helper;
use bits_core::word::Word;
use bits_core::Bits;

#[derive(Debug, Default, Clone)]
pub struct Buf<T: private::Array>(Option<Box<T>>);

mod private {
    pub trait Array {}
    impl<E, const N: usize> Array for [E; N] {}
}

impl<B: Word, const N: usize> Buf<[B; N]> {
    #[inline]
    pub fn as_bits(&self) -> &Bits<B> {
        Bits::new(self.as_slice())
    }

    #[inline]
    pub fn as_bits_mut(&mut self) -> &mut Bits<B> {
        Bits::new_mut(self.as_mut_slice())
    }

    #[inline]
    fn as_slice(&self) -> &[B] {
        self.0.as_deref().map_or(&[], |a| a.as_slice())
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [B] {
        self.0.as_deref_mut().map_or(&mut [], |a| a.as_mut_slice())
    }

    #[inline]
    fn inner(&self) -> Option<&Bits<B>> {
        self.0.as_deref().map(|a| Bits::new(a.as_slice()))
    }

    #[inline]
    fn inner_mut(&mut self) -> Option<&mut Bits<B>> {
        self.0.as_deref_mut().map(|a| Bits::new_mut(a.as_mut_slice()))
    }

    #[inline]
    fn or_empty(&mut self) -> &mut Bits<B> {
        Bits::new_mut(self.0.get_or_insert_with(|| Box::new([B::empty(); N])).as_mut_slice())
    }
}

impl<B: Word, const N: usize> Block for Buf<[B; N]> {
    const BITS: usize = <[B; N]>::BITS;

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_buf::Buf;
    /// let mut b = Buf::<[u32; 16]>::empty();
    /// assert_eq!(Buf::<[u32; 16]>::BITS, 512);
    ///
    /// b.set1(100);
    /// assert_eq!(b.count1(), 1);
    /// assert_eq!(b.count0(), 511);
    /// ```
    #[inline]
    fn empty() -> Self {
        Buf(None)
    }

    #[inline]
    fn test(&self, i: usize) -> Option<bool> {
        self.inner().and_then(|b| b.test(i))
    }
}
impl<B: Word, const N: usize> BlockMut for Buf<[B; N]> {
    #[inline]
    fn set1(&mut self, i: usize) {
        assert!(i < Self::BITS);
        self.or_empty().set1(i);
    }

    #[inline]
    fn set0(&mut self, i: usize) {
        assert!(i < Self::BITS);
        self.or_empty().set0(i);
    }
}

impl<B: Word, const N: usize> Count for Buf<[B; N]> {
    #[inline]
    fn count1(&self) -> usize {
        self.inner().map_or(0, |b| b.count1())
    }
}

impl<B: Word, const N: usize> Rank for Buf<[B; N]> {
    #[inline]
    fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        self.inner().map_or(0, |b| b.rank1(r))
    }
}

impl<B: Word, const N: usize> Select for Buf<[B; N]> {
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        self.inner().and_then(|b| b.select1(n))
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_buf::Buf;
    /// let mut b = Buf::<[u64; 8]>::empty();
    /// assert_eq!(b.select1(0), None);
    /// assert_eq!(b.select0(0), Some(0));
    /// assert_eq!(b.select0(Buf::<[u64; 8]>::BITS - 1), Some(511));
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

impl<B, const N: usize> helper::Assign<Buf<[B; N]>> for Buf<[B; N]>
where
    B: Word + helper::Assign<B>,
{
    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_core::mask::helper::Assign;
    /// let mut a = bits_buf::Buf::<[u64; 4]>::empty();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    ///
    /// let mut b = bits_buf::Buf::<[u64; 4]>::empty();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    ///
    /// Assign::and(&mut a, &b);
    /// assert_eq!(a.as_bits().as_slice(), &[0b_0110, 0, 0, 0]);
    ///
    /// Assign::and(&mut a, &bits_buf::Buf::empty());
    /// assert_eq!(a.as_bits().as_slice(), &[]);
    /// ```
    fn and(a: &mut Self, b: &Buf<[B; N]>) {
        match (a.inner_mut(), b.inner()) {
            (Some(a), Some(b)) => helper::Assign::and(a, b),
            (None, _) | (_, None) => a.0 = None,
        };
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_core::mask::helper::Assign;
    /// let mut a = bits_buf::Buf::<[u64; 4]>::empty();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    ///
    /// let mut b = bits_buf::Buf::<[u64; 4]>::empty();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    ///
    /// Assign::not(&mut a, &b);
    /// assert_eq!(a.as_bits().as_slice(), &[0b_0001, 0, 0, 0]);
    /// ```
    fn not(a: &mut Self, b: &Buf<[B; N]>) {
        if let (Some(a), Some(b)) = (a.inner_mut(), b.inner()) {
            helper::Assign::not(a, b);
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_core::mask::helper::Assign;
    /// let mut a = bits_buf::Buf::<[u64; 4]>::empty();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    ///
    /// let mut b = bits_buf::Buf::<[u64; 4]>::empty();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    ///
    /// Assign::or(&mut a, &b);
    /// assert_eq!(a.as_bits().as_slice(), &[0b_1111, 0, 0, 0]);
    ///
    /// let mut c = bits_buf::Buf::<[u64; 4]>::empty();
    /// Assign::or(&mut c, &a);
    /// assert_eq!(c.as_bits().as_slice(), &[0b_1111, 0, 0, 0]);
    /// ```
    fn or(a: &mut Self, b: &Buf<[B; N]>) {
        match (a.inner_mut(), b.inner()) {
            (Some(a), Some(b)) => helper::Assign::or(a, b),
            (None, Some(b)) => a.or_empty().copy_from_slice(b),
            _ => {}
        }
    }

    /// # Tests
    ///
    /// ```
    /// # use bits_core::block::*;
    /// # use bits_core::mask::helper::Assign;
    /// let mut a = bits_buf::Buf::<[u64; 4]>::empty();
    /// a.set1(0);
    /// a.set1(1);
    /// a.set1(2);
    ///
    /// let mut b = bits_buf::Buf::<[u64; 4]>::empty();
    /// b.set1(1);
    /// b.set1(2);
    /// b.set1(3);
    ///
    /// Assign::xor(&mut a, &b);
    /// assert_eq!(a.as_bits().as_slice(), &[0b_1001, 0, 0, 0]);
    ///
    /// let mut c = bits_buf::Buf::<[u64; 4]>::empty();
    /// Assign::xor(&mut c, &a);
    /// assert_eq!(c.as_bits().as_slice(), &[0b_1001, 0, 0, 0]);
    /// ```
    fn xor(a: &mut Self, b: &Buf<[B; N]>) {
        match (a.inner_mut(), b.inner()) {
            (Some(a), Some(b)) => helper::Assign::xor(a, b),
            (None, Some(b)) => a.or_empty().copy_from_slice(b),
            _ => {}
        }
    }
}
