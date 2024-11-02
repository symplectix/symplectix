use crate::{index, Bits, Block};

pub trait BitsMut: Bits {
    fn set_bit(&mut self, i: usize);

    fn unset_bit(&mut self, i: usize);
}

impl<B: Block> BitsMut for [B] {
    #[inline]
    fn set_bit(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = index::address::<B>(i);
        self[i].set_bit(o)
    }

    #[inline]
    fn unset_bit(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = index::address::<B>(i);
        self[i].unset_bit(o)
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
    }
}

impl<B, const N: usize> BitsMut for [B; N]
where
    [B]: BitsMut,
{
    impl_bits_mut!([B], as_mut);
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<B> BitsMut for Vec<B>
    where
        [B]: BitsMut,
    {
        impl_bits_mut!([B]);
    }

    impl<T: ?Sized + BitsMut> BitsMut for Box<T> {
        impl_bits_mut!(T);
    }

    impl<'a, T> BitsMut for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
        T::Owned: BitsMut,
    {
        impl_bits_mut!(T::Owned, to_mut);
    }
}
