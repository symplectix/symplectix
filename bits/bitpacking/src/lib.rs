//! Provides helper methods to read/write auxiliary data for Rank and Select.
//! This library should not be used to compress/decompress a large array.
//! Consider using [`quickwit-oss/bitpacking`](https://github.com/quickwit-oss/bitpacking) in such cases.

use bits::{Bits, BitsMut, Word};

pub trait Pack: BitsMut {
    /// Writes `N` bits in `[i, i+N)`.
    #[doc(hidden)]
    fn pack<T: Word>(&mut self, i: usize, n: usize, bits: T) {
        debug_assert!(i < self.bits() && n <= T::BITS);

        for b in i..i + n {
            if bits.bit(b - i).unwrap_or_default() {
                self.bit_set(b);
            }
        }
    }
}

pub trait Unpack: Bits {
    /// Reads `n` bits from `i`, and returns it as the lowest `n` bits of `Int`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bitpacking::Unpack;
    /// let bits: &[u16] = &[0b_1101_0001_1010_0011, 0b_1001_1110_1110_1001];
    /// let len = 4;
    /// assert_eq!(bits.unpack::<u8>(0, len), 0b0011);
    /// assert_eq!(bits.unpack::<u8>(8, len), 0b0001);
    /// assert_eq!(bits.unpack::<u8>(14, len), 0b0111);
    /// assert_eq!(bits.unpack::<u8>(30, len), 0b0010);
    /// ```
    #[doc(hidden)]
    fn unpack<T: Word>(&self, i: usize, n: usize) -> T {
        debug_assert!(i < self.bits() && n <= T::BITS);

        let mut bits = T::empty();
        for b in i..i + n {
            if self.bit(b).unwrap_or_default() {
                bits.bit_set(b - i);
            }
        }
        bits
    }
}

macro_rules! ints_impl_packing {
    ($( $Int:ty )*) => ($(
        impl Unpack for $Int {
            // #[inline]
            // fn unpack<T: Block>(&self, i: usize, n: usize) -> T {
            //     num::cast((*self >> i) & <$Int>::mask(0, n))
            // }
        }

        impl Pack for $Int {
        }
    )*)
}
ints_impl_packing!(u8 u16 u32 u64 u128 usize);

impl<B: bits::Block + Unpack> Unpack for [B] {
    // #[doc(hidden)]
    // fn varint<T: Block>(&self, i: usize, n: usize) -> T {
    //     use crate::index;
    //     debug_assert!(i < self.bits() && n <= T::BITS);

    //     let mut cur = 0;
    //     let mut out = T::empty();
    //     index::between::<B>(i, i + n).for_each(|(i, r)| {
    //         if i < self.len() && cur < T::BITS {
    //             out |= self[i].varint::<T>(r.start, r.len()) << cur;
    //             cur += r.len();
    //         }
    //     });
    //     out
    // }
}

impl<B: bits::Block + Pack> Pack for [B] {
    // #[doc(hidden)]
    // fn put_varint<T: Block>(&mut self, i: usize, n: usize, int: T) {
    //     use crate::index;
    //     debug_assert!(i < self.bits() && n <= T::BITS);

    //     let mut cur = 0;
    //     index::between::<B>(i, i + n).for_each(|(i, r)| {
    //         if i < self.len() {
    //             self[i].put_varint::<T>(r.start, r.len(), int.varint::<T>(cur, r.len()));
    //             cur += r.len();
    //         }
    //     });
    // }
}

macro_rules! impl_unpack {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn unpack<I: Word>(&self, i: usize, n: usize) -> I {
            <$X as Unpack>::unpack(self$(.$method())?, i, n)
        }
    }
}

macro_rules! impl_pack {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn pack<I: Word>(&mut self, i: usize, n: usize, int: I) {
            <$X as Pack>::pack(self$(.$method())?, i, n, int)
        }
    }
}

impl<'a, T: ?Sized + Unpack> Unpack for &'a T {
    impl_unpack!(T);
}

impl<B, const N: usize> Unpack for [B; N]
where
    [B]: Unpack,
{
    impl_unpack!([B]);
}
impl<B, const N: usize> Pack for [B; N]
where
    [B]: Pack,
{
    impl_pack!([B]);
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<T: ?Sized + Unpack> Unpack for Box<T> {
        impl_unpack!(T);
    }
    impl<T: ?Sized + Pack> Pack for Box<T> {
        impl_pack!(T);
    }

    impl<T> Unpack for Vec<T>
    where
        [T]: Unpack,
    {
        impl_unpack!([T]);
    }
    impl<T> Pack for Vec<T>
    where
        [T]: Pack,
    {
        impl_pack!([T]);
    }

    impl<'a, T> Unpack for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Unpack,
    {
        impl_unpack!(T, as_ref);
    }

    impl<'a, T> Pack for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Unpack,
        T::Owned: Pack,
    {
        impl_pack!(T::Owned, to_mut);
    }
}
