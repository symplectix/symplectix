use crate::*;

pub trait Bits: Clone + Container + ContainerMut + Count + Rank + Excess + Select {
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn empty() -> Self;
}

macro_rules! ints_impl_bits {
    ($( $Int:ty )*) => ($(
        impl Bits for $Int {
            const BITS: usize = <$Int>::BITS as usize;

            #[inline]
            fn empty() -> Self {
                0
            }
        }

    )*)
}
ints_impl_bits!(u8 u16 u32 u64 u128 usize);
ints_impl_bits!(i8 i16 i32 i64 i128 isize);

impl<B, const N: usize> Bits for [B; N]
where
    B: Copy + Bits,
{
    const BITS: usize = B::BITS * N;

    #[inline]
    fn empty() -> Self {
        [B::empty(); N]
    }
}

mod impl_bits {
    use super::*;
    use std::borrow::Cow;

    impl<T: Bits> Bits for Box<T> {
        const BITS: usize = T::BITS;
        #[inline]
        fn empty() -> Self {
            Box::new(T::empty())
        }
    }

    impl<'a, T> Bits for Cow<'a, T>
    where
        T: ?Sized + Bits,
    {
        const BITS: usize = T::BITS;
        #[inline]
        fn empty() -> Self {
            Cow::Owned(T::empty())
        }
    }
}
