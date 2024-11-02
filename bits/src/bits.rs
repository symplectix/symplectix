use crate::index;
use crate::Block;

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
}

macro_rules! impls {
    ($( $Int:ty )*) => ($(
        impl Bits for $Int {
            #[inline]
            fn bits(&self) -> usize {
                <Self as Block>::BITS
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

impl<B: Block> Bits for [B] {
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
            <$X as Bits>::bits(self$(.$method())?)
        }

        #[inline]
        fn bit(&self, i: usize) -> Option<bool> {
            <$X as Bits>::bit(self$(.$method())?, i)
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

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<B> Bits for Vec<B>
    where
        [B]: Bits,
    {
        impl_bits!([B]);
    }

    impl<T: ?Sized + Bits> Bits for Box<T> {
        impl_bits!(T);
    }

    impl<'a, T> Bits for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Bits,
    {
        impl_bits!(T, as_ref);
    }
}
