use crate as bits;
use crate::Word;

pub trait BitPut: bits::ops::BitGet {
    fn put_1(&mut self, i: usize);

    fn put_0(&mut self, i: usize);

    #[doc(hidden)]
    fn put_word<N: bits::Word>(&mut self, i: usize, n: usize, mask: N) {
        for b in i..i + n {
            if bits::get(&mask, b - i).expect("index out of bounds") {
                self.put_1(b);
            }
        }
    }
}

macro_rules! impl_bit_put {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn put_1(&mut self, i: usize) {
            // <$X as BitPut>::put_1(self$(.$method())?, i)
            bits::put_1::<$X>(self$(.$method())?, i)
        }

        #[inline]
        fn put_0(&mut self, i: usize) {
            // <$X as BitPut>::put_0(self$(.$method())?, i)
            bits::put_0::<$X>(self$(.$method())?, i)
        }

        #[doc(hidden)]
        #[inline]
        fn put_word<W: Word>(&mut self, i: usize, n: usize, word: W) {
            // <$X as BitPut>::put_word(self$(.$method())?, i, n, word)
            bits::put_word::<$X, W>(self$(.$method())?, i, n, word)
        }
    }
}

impl<T, const N: usize> BitPut for [T; N]
where
    [T]: BitPut,
{
    impl_bit_put!([T], as_mut);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T: ?Sized + BitPut> BitPut for Box<T> {
        impl_bit_put!(T);
    }

    impl<T> BitPut for Vec<T>
    where
        [T]: BitPut,
    {
        impl_bit_put!([T]);
    }

    impl<'a, T> BitPut for Cow<'a, T>
    where
        T: ?Sized + ToOwned + bits::ops::BitGet,
        T::Owned: BitPut,
    {
        impl_bit_put!(T::Owned, to_mut);
    }
}