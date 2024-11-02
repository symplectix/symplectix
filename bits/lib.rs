//! `bits`

macro_rules! mask {
    ($( $Int: ty, $i: expr, $j: expr )*) => ($(
        if $i >= $j {
            0
        } else {
            !0 >> (<$Int>::BITS as usize - ($j - $i)) << $i
        }
    )*)
}

mod container;
mod container_mut;
mod count;
mod excess;
mod lsb;
mod msb;
mod rank;
mod select;

mod mask;

pub mod and;
pub mod not;
pub mod or;
pub mod xor;

pub use self::container::Container;
pub use self::container_mut::ContainerMut;
pub use self::count::Count;
pub use self::excess::Excess;
pub use self::mask::Mask;
pub use self::rank::Rank;
pub use self::select::Select;
pub use self::{lsb::Lsb, msb::Msb};

pub use self::{
    container::{get, is_empty, len},
    container_mut::{clear, set},
    count::{all, any, count0, count1},
    excess::{excess, excess0, excess1},
    rank::{rank0, rank1},
    select::{select0, select1},
};

pub use self::{lsb::lsb, msb::msb};

/// Constructs a new, empty `Vec<T>`.
///
/// # Examples
///
/// ```
/// let v = bits::new::<u8>(80);
/// assert_eq!(bits::len(&v), 80);
/// assert_eq!(v.len(), 10);
/// ```
pub fn new<T: Bits>(n: usize) -> Vec<T> {
    use std::iter::from_fn;
    from_fn(|| Some(T::empty())).take(bit::blocks(n, T::BITS)).collect::<Vec<T>>()
}

/// Returns a `Vec<T>` with the at least specified capacity in bits.
///
/// # Examples
///
/// ```
/// let v = bits::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(bits::len(&v), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: Bits>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(bit::blocks(capacity, T::BITS))
}

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

    impl<B: Bits> Bits for Box<B> {
        const BITS: usize = B::BITS;
        #[inline]
        fn empty() -> Self {
            Box::new(B::empty())
        }
    }

    impl<'a, B> Bits for Cow<'a, B>
    where
        B: ?Sized + Bits,
    {
        const BITS: usize = B::BITS;
        #[inline]
        fn empty() -> Self {
            Cow::Owned(B::empty())
        }
    }
}
