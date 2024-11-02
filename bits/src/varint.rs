use crate::{Bits, Container, ContainerMut};

/// # Examples
///
/// ```
/// # use bits::Varint;
/// let bits: &[u16] = &[0b_1101_0001_1010_0011, 0b_1001_1110_1110_1001];
/// let len = 4;
/// assert_eq!(bits.varint::<u8>(0, len), 0b0011);
/// assert_eq!(bits.varint::<u8>(8, len), 0b0001);
/// assert_eq!(bits.varint::<u8>(14, len), 0b0111);
/// assert_eq!(bits.varint::<u8>(30, len), 0b0010);
/// ```
pub trait Varint: Container {
    /// Reads `n` bits from `i`, and returns it as the lowest `n` bits of `Int`.
    #[doc(hidden)]
    fn varint<T: Bits>(&self, i: usize, n: usize) -> T {
        debug_assert!(i < self.bits() && n <= T::BITS);

        let mut block = T::empty();
        for b in i..i + n {
            if self.bit(b).unwrap_or_default() {
                block.set_bit(b - i);
            }
        }
        block
    }
}

pub trait PutVarint: ContainerMut + Varint {
    /// Writes `N` bits in `[i, i+N)`.
    #[doc(hidden)]
    fn put_varint<T: Bits>(&mut self, i: usize, n: usize, int: T) {
        debug_assert!(i < self.bits() && n <= T::BITS);

        for b in i..i + n {
            if int.bit(b - i).unwrap_or_default() {
                self.set_bit(b);
            }
        }
    }
}

macro_rules! int_impls {
    ($( $Int:ty )*) => ($(
        impl Varint for $Int {
            // #[inline]
            // fn varint<T: Block>(&self, i: usize, n: usize) -> T {
            //     num::cast((*self >> i) & <$Int>::mask(0, n))
            // }
        }

        impl PutVarint for $Int {
        }
    )*)
}
int_impls!(u8 u16 u32 u64 u128 usize);
int_impls!(i8 i16 i32 i64 i128 isize);

impl<B: Bits + Varint> Varint for [B] {
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

impl<B: Bits + PutVarint> PutVarint for [B] {
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

macro_rules! impl_varint {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn varint<I: Bits>(&self, i: usize, n: usize) -> I {
            <$X as Varint>::varint(self$(.$method())?, i, n)
        }
    }
}

macro_rules! impl_put_varint {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn put_varint<I: Bits>(&mut self, i: usize, n: usize, int: I) {
            <$X as PutVarint>::put_varint(self$(.$method())?, i, n, int)
        }
    }
}

impl<'a, T: ?Sized + Varint> Varint for &'a T {
    impl_varint!(T);
}

impl<B, const N: usize> Varint for [B; N]
where
    [B]: Varint,
{
    impl_varint!([B]);
}
impl<B, const N: usize> PutVarint for [B; N]
where
    [B]: PutVarint,
{
    impl_put_varint!([B]);
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<T: ?Sized + Varint> Varint for Box<T> {
        impl_varint!(T);
    }
    impl<T: ?Sized + PutVarint> PutVarint for Box<T> {
        impl_put_varint!(T);
    }

    impl<T> Varint for Vec<T>
    where
        [T]: Varint,
    {
        impl_varint!([T]);
    }
    impl<T> PutVarint for Vec<T>
    where
        [T]: PutVarint,
    {
        impl_put_varint!([T]);
    }

    impl<'a, T> Varint for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Varint,
    {
        impl_varint!(T, as_ref);
    }

    impl<'a, T> PutVarint for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Varint,
        T::Owned: PutVarint,
    {
        impl_put_varint!(T::Owned, to_mut);
    }
}
