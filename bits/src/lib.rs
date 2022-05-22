//! `bits`

pub mod bit_all;
pub mod bit_any;
pub mod bit_count;
pub mod bit_get;
pub mod bit_len;
pub mod bit_put;
pub mod bit_rank;
pub mod bit_select;
mod bits;
pub mod ops;

pub use self::bits::Word;

pub trait Bits:
    Clone
    + ops::BitLen
    + ops::BitCount
    + ops::BitAll
    + ops::BitAny
    + ops::BitRank
    + ops::BitSelect
    + ops::BitGet
    + ops::BitPut
{
    const BITS: usize;

    #[doc(hidden)]
    const SIZE: usize = Self::BITS / 8;

    fn null() -> Self;
}

impl Bits for bool {
    const BITS: usize = 1;

    #[inline]
    fn null() -> Self {
        false
    }
}

impl<T, const N: usize> Bits for [T; N]
where
    T: Copy + Bits,
{
    const BITS: usize = T::BITS * N;

    #[inline]
    fn null() -> Self {
        [T::null(); N]
    }
}

mod alloc {
    use super::Bits;
    use std::borrow::Cow;

    impl<T: Bits> Bits for Box<T> {
        const BITS: usize = T::BITS;
        #[inline]
        fn null() -> Self {
            Box::new(T::null())
        }
    }

    impl<'a, T> Bits for Cow<'a, T>
    where
        T: ?Sized + Bits,
    {
        const BITS: usize = T::BITS;
        #[inline]
        fn null() -> Self {
            Cow::Owned(T::null())
        }
    }
}

#[inline]
fn address<T: Bits>(i: usize) -> (usize, usize) {
    use core::ops::{Div, Rem};
    fn divrem<T, U>(t: T, u: U) -> (<T as Div<U>>::Output, <T as Rem<U>>::Output)
    where
        T: Copy + Div<U> + Rem<U>,
        U: Copy,
    {
        (t / u, t % u)
    }

    divrem(i, T::BITS)
}

/// A utility to clamp the given range into a valid one.
/// Panics if debug is enabled and `min <= i && i <= j && j <= max`.
fn to_range<R: core::ops::RangeBounds<usize>>(r: &R, min: usize, max: usize) -> (usize, usize) {
    use core::ops::Bound::*;

    let (i, j) = (
        match r.start_bound() {
            Included(&s) => s,
            Excluded(&s) => s + 1,
            Unbounded => min,
        },
        match r.end_bound() {
            Included(&e) => e + 1,
            Excluded(&e) => e,
            Unbounded => max,
        },
    );

    debug_assert!(min <= i && i <= j && j <= max);
    (i, j)
}

/// Calculates the minimum number of blocks to store `n` bits.
const fn blocks(n: usize, b: usize) -> usize {
    n / b + (n % b > 0) as usize
}

/// Returns an empty `Vec<T>` with the at least specified capacity in bits.
///
/// ```
/// # use bits::ops::BitLen;
/// let v = bits::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(v.bit_len(), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: Bits>(n: usize) -> Vec<T> {
    let size = blocks(n, T::BITS);
    Vec::with_capacity(size)
}

// pub fn null<T: Block>(n: usize) -> Vec<T> {
//     use core::iter::from_fn;
//     let size = blocks(n, T::BITS);
//     from_fn(|| Some(T::empty())).take(size).collect()
// }
