use crate::{Block, Word};

pub trait Bits {
    /// Returns the number of binary digits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u8] = &[0, 0, 0];
    /// let w: &[u8] = &[];
    /// assert_eq!(v.bits(), 24);
    /// assert_eq!(w.bits(), 0);
    /// ```
    fn bits(&self) -> usize;

    /// Returns a bit at the given index `i`.
    /// When i is out of bounds, returns **None**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.bit(0),   Some(true));
    /// assert_eq!(v.bit(64),  Some(true));
    /// assert_eq!(v.bit(128), Some(false));
    /// assert_eq!(v.bit(200), None);
    /// ```
    fn bit(&self, i: usize) -> Option<bool>;

    /// Reads `n` bits within `[i, i+n)`, and returns it as the lowest `n` bits of `Word`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Bits;
    /// let s: &[u64] = &[!0, 0, !0];
    /// assert_eq!(s.word::<u32>(  0,  4), 0b1111);
    /// assert_eq!(s.word::<u32>( 60, 20), 0b1111);
    /// assert_eq!(s.word::<u32>(188, 10), 0b1111);
    /// ```
    #[doc(hidden)]
    fn word<T: Word>(&self, i: usize, n: usize) -> T {
        let mut w = T::NULL;
        for b in i..i + n {
            if self.bit(b).expect("index out of bounds") {
                w.set_bit(b - i);
            }
        }
        w
    }
}

impl<T: Block> Bits for [T] {
    #[inline]
    fn bits(&self) -> usize {
        T::BITS * <[T]>::len(self)
    }

    #[inline]
    fn bit(&self, i: usize) -> Option<bool> {
        let (i, o) = crate::address::<T>(i);
        self.get(i).map(|b| b.bit(o).expect("index out of bounds"))
    }

    #[inline]
    #[doc(hidden)]
    fn word<N: Word>(&self, i: usize, n: usize) -> N {
        let mut cur = 0;
        let mut out = N::NULL;
        crate::for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() && cur < <N as Block>::BITS {
                out |= self[k].word::<N>(r.start, r.len()) << cur;
                cur += r.len();
            }
        });
        out
    }
}

/// ```
/// # use bits::Bits;
/// assert_eq!(Bits::bit(&true,  0), Some(true));
/// assert_eq!(Bits::bit(&true,  1), None);
/// assert_eq!(Bits::bit(&false, 0), Some(false));
/// assert_eq!(Bits::bit(&false, 1), None);
/// ```
impl Bits for bool {
    #[inline]
    fn bits(&self) -> usize {
        1
    }

    #[inline]
    fn bit(&self, i: usize) -> Option<bool> {
        (i < 1).then(|| *self)
    }
}

macro_rules! impl_bits {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bits(&self) -> usize {
            <$X as Bits>::bits(self$(.$method())?)
        }

        #[inline]
        fn bit(&self, i: usize) -> Option<bool> {
            <$X as Bits>::bit(self$(.$method())?, i)
        }

        #[doc(hidden)]
        #[inline]
        fn word<W: Word>(&self, i: usize, n: usize) -> W {
            <$X as Bits>::word(self$(.$method())?, i, n)
        }
    }
}

impl<'a, T: ?Sized + Bits> Bits for &'a T {
    impl_bits!(T);
}

impl<T, const N: usize> Bits for [T; N]
where
    [T]: Bits,
{
    impl_bits!([T], as_ref);
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::borrow::{Cow, ToOwned};
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<T: ?Sized + Bits> Bits for Box<T> {
        impl_bits!(T);
    }

    impl<T> Bits for Vec<T>
    where
        [T]: Bits,
    {
        impl_bits!([T]);
    }

    impl<'a, T> Bits for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
    {
        impl_bits!(T, as_ref);
    }
}
