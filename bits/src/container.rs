use crate::index;
use crate::Bits;

pub trait Container {
    /// Returns the number of binary digits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bits::Container;
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
    /// # use bits::Container;
    /// let v: &[u64] = &[0b00000101, 0b01100011, 0b01100000];
    /// assert_eq!(v.bit(0),   Some(true));
    /// assert_eq!(v.bit(64),  Some(true));
    /// assert_eq!(v.bit(128), Some(false));
    /// assert_eq!(v.bit(200), None);
    /// ```
    fn bit(&self, i: usize) -> Option<bool>;
}

macro_rules! impls {
    ($( $Int:ty )*) => ($(
        impl Container for $Int {
            #[inline]
            fn bits(&self) -> usize {
                <Self as Bits>::BITS
            }

            #[inline]
            fn bit(&self, i: usize) -> Option<bool> {
                (i < self.bits()).then(|| (*self & (1 << i)) != 0)
            }
        }
    )*)
}
impls!(u8 u16 u32 u64 u128 usize);
impls!(i8 i16 i32 i64 i128 isize);

impl<B: Bits> Container for [B] {
    #[inline]
    fn bits(&self) -> usize {
        B::BITS * self.len()
    }

    #[inline]
    fn bit(&self, i: usize) -> Option<bool> {
        let (i, o) = index::address::<B>(i);
        self.get(i).map(|b| b.bit(o).expect("index out of bounds"))
    }
}

macro_rules! impl_bits {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn bits(&self) -> usize {
            <$X as Container>::bits(self$(.$method())?)
        }

        #[inline]
        fn bit(&self, i: usize) -> Option<bool> {
            <$X as Container>::bit(self$(.$method())?, i)
        }
    }
}

impl<'a, T: ?Sized + Container> Container for &'a T {
    impl_bits!(T);
}

impl<T, const N: usize> Container for [T; N]
where
    [T]: Container,
{
    impl_bits!([T], as_ref);
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<B> Container for Vec<B>
    where
        [B]: Container,
    {
        impl_bits!([B]);
    }

    impl<T: ?Sized + Container> Container for Box<T> {
        impl_bits!(T);
    }

    impl<'a, T> Container for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Container,
    {
        impl_bits!(T, as_ref);
    }
}
