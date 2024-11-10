use std::borrow::Cow;

use crate::{Bits, Block};

pub trait BitsMut: Bits {
    /// Enables the bit at the given index `i`.
    fn set1(&mut self, i: usize);

    /// Disables the bit at the given index `i`.
    fn set0(&mut self, i: usize);
}

impl<B: Block> BitsMut for [B] {
    #[inline]
    fn set1(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = bit::addr(i, B::BITS);
        self[i].set1(o)
    }

    #[inline]
    fn set0(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = bit::addr(i, B::BITS);
        self[i].set0(o)
    }
}

macro_rules! impl_BitsMut {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn set1(&mut self, i: usize) {
            <$X as BitsMut>::set1(self$(.$method())?, i)
        }

        #[inline]
        fn set0(&mut self, i: usize) {
            <$X as BitsMut>::set0(self$(.$method())?, i)
        }
    }
}

impl<B, const N: usize> BitsMut for [B; N]
where
    [B]: BitsMut,
{
    impl_BitsMut!([B], as_mut);
}

impl<B> BitsMut for Vec<B>
where
    [B]: BitsMut,
{
    impl_BitsMut!([B]);
}

impl<T> BitsMut for Box<T>
where
    T: ?Sized + BitsMut,
{
    impl_BitsMut!(T);
}

impl<'a, T> BitsMut for Cow<'a, T>
where
    T: ?Sized + ToOwned + Bits,
    T::Owned: BitsMut,
{
    impl_BitsMut!(T::Owned, to_mut);
}
