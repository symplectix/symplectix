use crate::{Bits, Container};

pub trait ContainerMut: Container {
    fn set_bit(&mut self, i: usize);

    fn unset_bit(&mut self, i: usize);
}

macro_rules! ints_impl_container_mut {
    ($( $Int:ty )*) => ($(
        impl ContainerMut for $Int {
            #[inline]
            fn set_bit(&mut self, i: usize) {
                *self |= 1 << i;
            }
            #[inline]
            fn unset_bit(&mut self, i: usize) {
                *self &= !(1 << i);
            }
        }
    )*)
}
ints_impl_container_mut!(u8 u16 u32 u64 u128 usize);
ints_impl_container_mut!(i8 i16 i32 i64 i128 isize);

impl<B: Bits> ContainerMut for [B] {
    #[inline]
    fn set_bit(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = bit::addr(i, B::BITS);
        self[i].set_bit(o)
    }

    #[inline]
    fn unset_bit(&mut self, i: usize) {
        assert!(i < self.bits());
        let (i, o) = bit::addr(i, B::BITS);
        self[i].unset_bit(o)
    }
}

macro_rules! impl_bits_mut {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn set_bit(&mut self, i: usize) {
            <$X as ContainerMut>::set_bit(self$(.$method())?, i)
        }

        #[inline]
        fn unset_bit(&mut self, i: usize) {
            <$X as ContainerMut>::unset_bit(self$(.$method())?, i)
        }
    }
}

impl<B, const N: usize> ContainerMut for [B; N]
where
    [B]: ContainerMut,
{
    impl_bits_mut!([B], as_mut);
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<B> ContainerMut for Vec<B>
    where
        [B]: ContainerMut,
    {
        impl_bits_mut!([B]);
    }

    impl<T: ?Sized + ContainerMut> ContainerMut for Box<T> {
        impl_bits_mut!(T);
    }

    impl<'a, T> ContainerMut for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Container,
        T::Owned: ContainerMut,
    {
        impl_bits_mut!(T::Owned, to_mut);
    }
}
