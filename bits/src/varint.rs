use crate::{Bits, BitsMut, Block, Int};
use core::ops::Range;

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
pub trait Varint: Bits {
    /// Reads `n` bits from `i`, and returns it as the lowest `n` bits of `Int`.
    #[doc(hidden)]
    fn varint<T: Int>(&self, i: usize, n: usize) -> T {
        debug_assert!(i < self.bits() && n <= T::BITS);

        let mut int = T::NULL;
        for b in i..i + n {
            // unwrap_or?
            if self.bit(b).expect("index out of bounds") {
                int.set_bit(b - i);
            }
        }
        int
    }
}

pub trait PutVarint: BitsMut + Varint {
    /// Writes `N` bits in `[i, i+N)`.
    #[doc(hidden)]
    fn put_varint<T: Int>(&mut self, i: usize, n: usize, int: T) {
        debug_assert!(i < self.bits() && n <= T::BITS);

        for b in i..i + n {
            // unwrap_or?
            if int.bit(b - i).expect("index out of bounds") {
                self.set_bit(b);
            }
        }
    }
}

macro_rules! int_impls {
    ($( $Int:ty )*) => ($(
        impl Varint for $Int {
            #[inline]
            fn varint<T: Int>(&self, i: usize, n: usize) -> T {
                num::cast((*self >> i) & <$Int>::mask(0, n))
            }
        }

        impl PutVarint for $Int {
        }
    )*)
}
int_impls!(u8 u16 u32 u64 u128 usize);
int_impls!(i8 i16 i32 i64 i128 isize);

fn for_each_blocks<B, F>(s: usize, e: usize, mut f: F)
where
    B: Block,
    F: FnMut(usize, Range<usize>),
{
    assert!(s <= e);
    if s == e {
        return;
    }

    let (s, p) = crate::address::<B>(s);
    let (e, q) = crate::address::<B>(e);

    if s == e {
        f(s, p..q);
    } else {
        f(s, p..B::BITS);
        (s + 1..e).for_each(|k| f(k, 0..B::BITS));
        f(e, 0..q)
    }
}

impl<B: Block + Varint> Varint for [B] {
    #[doc(hidden)]
    fn varint<T: Int>(&self, i: usize, n: usize) -> T {
        debug_assert!(i < self.bits() && n <= T::BITS);

        let mut cur = 0;
        let mut out = T::NULL;
        for_each_blocks::<B, _>(i, i + n, |k, r| {
            if k < self.len() && cur < T::BITS {
                out |= self[k].varint::<T>(r.start, r.len()) << cur;
                cur += r.len();
            }
        });
        out
    }
}

impl<B: Block + PutVarint> PutVarint for [B] {
    #[doc(hidden)]
    fn put_varint<T: Int>(&mut self, i: usize, n: usize, int: T) {
        let mut cur = 0;
        for_each_blocks::<B, _>(i, i + n, |k, r| {
            if k < self.len() {
                self[k].put_varint::<T>(r.start, r.len(), int.varint::<T>(cur, r.len()));
                cur += r.len();
            }
        });
    }
}

macro_rules! impl_varint {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn varint<I: Int>(&self, i: usize, n: usize) -> I {
            <$X as Varint>::varint(self$(.$method())?, i, n)
        }
    }
}

macro_rules! impl_put_varint {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn put_varint<I: Int>(&mut self, i: usize, n: usize, int: I) {
            <$X as PutVarint>::put_varint(self$(.$method())?, i, n, int)
        }
    }
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

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::borrow::{Cow, ToOwned};
    use alloc::boxed::Box;
    use alloc::vec::Vec;

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
