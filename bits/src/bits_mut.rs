use crate::{Bits, Block, Word};

pub trait BitsMut: Bits {
    fn set_bit(&mut self, i: usize);

    fn unset_bit(&mut self, i: usize);

    /// Writes `n` bits in `[i, i+n)`.
    #[doc(hidden)]
    fn put_word<N: Word>(&mut self, i: usize, n: usize, mask: N) {
        for b in i..i + n {
            if mask.bit(b - i).expect("index out of bounds") {
                self.set_bit(b);
            }
        }
    }
}

impl<T: Block> BitsMut for [T] {
    #[inline]
    fn set_bit(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = crate::address::<T>(i);
        self[i].set_bit(o)
    }

    #[inline]
    fn unset_bit(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = crate::address::<T>(i);
        self[i].unset_bit(o)
    }

    #[inline]
    #[doc(hidden)]
    fn put_word<N: Word>(&mut self, i: usize, n: usize, word: N) {
        let mut cur = 0;
        crate::for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() {
                let word = word.word(cur, r.len());
                self[k].put_word::<N>(r.start, r.len(), word);
                cur += r.len();
            }
        });
    }
}

impl BitsMut for bool {
    #[inline]
    fn set_bit(&mut self, i: usize) {
        assert!(i < self.bits());
        *self = true;
    }

    #[inline]
    fn unset_bit(&mut self, i: usize) {
        assert!(i < self.bits());
        *self = false;
    }
}

macro_rules! impl_bits_mut {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn set_bit(&mut self, i: usize) {
            <$X as BitsMut>::set_bit(self$(.$method())?, i)
        }

        #[inline]
        fn unset_bit(&mut self, i: usize) {
            <$X as BitsMut>::unset_bit(self$(.$method())?, i)
        }

        #[doc(hidden)]
        #[inline]
        fn put_word<W: Word>(&mut self, i: usize, n: usize, word: W) {
            <$X as BitsMut>::put_word(self$(.$method())?, i, n, word)
        }
    }
}

impl<T, const N: usize> BitsMut for [T; N]
where
    [T]: BitsMut,
{
    impl_bits_mut!([T], as_mut);
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::borrow::{Cow, ToOwned};
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<T: ?Sized + BitsMut> BitsMut for Box<T> {
        impl_bits_mut!(T);
    }

    impl<T> BitsMut for Vec<T>
    where
        [T]: BitsMut,
    {
        impl_bits_mut!([T]);
    }

    impl<'a, T> BitsMut for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
        T::Owned: BitsMut,
    {
        impl_bits_mut!(T::Owned, to_mut);
    }
}
