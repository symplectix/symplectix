use crate::*;
use core::{hash::Hash, ops, ops::RangeBounds};

mod private {
    pub trait Sealed {}

    macro_rules! impl_for_nums {
        ( $( $Type:ty )* ) => {
            $( impl Sealed for $Type {} )*
        };
    }

    impl_for_nums!(u8 u16 u32 u64 u128 usize);
    impl_for_nums!(i8 i16 i32 i64 i128 isize);
}

#[inline]
fn mask<T: Word>(i: usize, j: usize) -> T {
    // TODO: assert!(i <= j);
    // if i == j {
    if i >= j {
        T::NULL
    } else {
        T::FULL >> (<T as Block>::BITS - (j - i)) << i
    }
}

/// `Word` is a fixed-length group of bits that the CPU can process.
pub trait Word:
    'static
    + Copy
    + Hash
    + Eq
    + Ord
    + Block
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Rem<Output = Self>
    + ops::AddAssign
    + ops::SubAssign
    + ops::MulAssign
    + ops::DivAssign
    + ops::RemAssign
    + ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::BitXor<Output = Self>
    + ops::Shl<usize, Output = Self>
    + ops::Shr<usize, Output = Self>
    + ops::Not<Output = Self>
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign<usize>
    + ops::ShrAssign<usize>
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<usize>
    + private::Sealed
{
    /// literal 0
    #[doc(hidden)]
    const _0: Self;

    /// literal 1
    #[doc(hidden)]
    const _1: Self;

    /// An empty, no bits are enabled, `Word`.
    #[doc(hidden)]
    const NULL: Self;

    /// A full, all bits are enabled, `Word`.
    #[doc(hidden)]
    const FULL: Self;

    #[doc(hidden)]
    #[inline]
    fn cast<N>(self) -> N
    where
        N: Word + TryFrom<Self>,
    {
        N::try_from(self).ok().unwrap()
    }

    /// Returns the number of leading ones in the binary representation of self.
    fn count_l1(self) -> usize;

    /// Returns the number of leading zeros in the binary representation of self.
    fn count_l0(self) -> usize;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn count_t0(self) -> usize;

    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;

    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;
}

macro_rules! impls {
    ($( $Word:ty )*) => ($(
        impl Word for $Word {
            #[doc(hidden)]
            const _0: Self = 0;

            #[doc(hidden)]
            const _1: Self = 1;

            #[doc(hidden)]
            const NULL: Self = 0;

            #[doc(hidden)]
            const FULL: Self = !0;

            #[inline]
            fn count_l1(self) -> usize {
                self.leading_ones() as usize
            }

            #[inline]
            fn count_l0(self) -> usize {
                self.leading_zeros() as usize
            }

            #[inline]
            fn count_t0(self) -> usize {
                self.trailing_zeros() as usize
            }

            #[inline]
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }

            #[inline]
            fn msb(self) -> Self {
                if self == 0 {
                    0
                } else {
                    1 << ((<Self as Block>::BITS - 1) ^ self.count_l0())
                }
            }
        }

        impl Block for $Word {
            const BITS: usize = <$Word>::BITS as usize;

            #[inline]
            fn empty() -> Self {
                Self::NULL
            }
        }

        impl Bits for $Word {
            #[inline]
            fn bits(&self) -> usize {
                <Self as Block>::BITS
            }

            #[inline]
            fn bit(&self, i: usize) -> Option<bool> {
                (i < self.bits()).then(|| (*self & (1 << i)) != 0)
            }

            #[doc(hidden)]
            #[inline]
            fn word<N: Word>(&self, i: usize, n: usize) -> N {
                ((*self >> i) & mask::<Self>(0, n)).cast()
            }
        }

        impl BitsMut for $Word {
            #[inline]
            fn set_bit(&mut self, i: usize) {
                *self |= 1 << i;
            }
            #[inline]
            fn unset_bit(&mut self, i: usize) {
                *self &= !(1 << i);
            }
        }

        impl Count for $Word {
            #[inline]
            fn count1(&self) -> usize {
                self.count_ones() as usize
            }

            #[inline]
            fn count0(&self) -> usize {
                self.count_zeros() as usize
            }

            #[inline]
            fn all(&self) -> bool {
                *self == Self::FULL
            }

            #[inline]
            fn any(&self) -> bool {
                *self != Self::NULL
            }
        }

        impl Rank for $Word {
            #[inline]
            fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
                let (i, j) = to_range(&r, 0, self.bits());
                (*self & mask::<Self>(i, j)).count1()
            }

            #[inline]
            fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
                (!*self).rank1(r)
            }
        }

        impl Select for $Word {
            #[inline]
            fn select1(&self, n: usize) -> Option<usize> {
                <Self as SelectWord>::select_word(*self, n)
            }

            #[inline]
            fn select0(&self, n: usize) -> Option<usize> {
                <Self as SelectWord>::select_word(!self, n)
            }
        }
    )*)
}
impls!(u8 u16 u32 u64 u128);

/// A helper trait to implement `Select` for u64.
trait SelectWord {
    fn select_word(self, n: usize) -> Option<usize>;
}

impl SelectWord for u64 {
    // Need the `std` crate to use `is_x86_feature_detected`.
    //
    // ```
    // #[cfg(all(feature = "std", any(target_arch = "x86", target_arch = "x86_64")))]
    // if is_x86_feature_detected!("bmi2") {
    //     use std::arch::x86_64::{_pdep_u64, _tzcnt_u64};
    //     return unsafe { _tzcnt_u64(_pdep_u64(1 << n, self)) as usize };
    // }
    // ```

    #[cfg(target_feature = "bmi2")]
    #[inline]
    fn select_word(self, n: usize) -> Option<usize> {
        (n < self.count1()).then(|| {
            use core::arch::x86_64::{_pdep_u64, _tzcnt_u64};
            unsafe { _tzcnt_u64(_pdep_u64(1 << n, self)) as usize }
        })
    }

    #[cfg(not(target_feature = "bmi2"))]
    #[inline]
    fn select_word(self, n: usize) -> Option<usize> {
        (n < self.count1()).then(|| broadword(self, n as u64))
    }
}

// Sebastiano Vigna, “Broadword Implementation of Rank/Select Queries”
// Returns 72 when not found.
#[cfg(not(target_feature = "bmi2"))]
#[allow(clippy::many_single_char_names)]
fn broadword(x: u64, n: u64) -> usize {
    const L8: u64 = 0x0101_0101_0101_0101; // has the lowest bit of every bytes
    const H8: u64 = 0x8080_8080_8080_8080; // has the highest bit of every bytes

    #[inline]
    const fn le8(x: u64, y: u64) -> u64 {
        (((y | H8) - (x & !H8)) ^ x ^ y) & H8
    }

    #[inline]
    const fn lt8(x: u64, y: u64) -> u64 {
        (((x | H8) - (y & !H8)) ^ x ^ !y) & H8
    }

    #[inline]
    const fn nz8(x: u64) -> u64 {
        lt8(0, x)
    }

    let mut s = x - ((x & 0xAAAA_AAAA_AAAA_AAAA) >> 1);
    s = (s & 0x3333_3333_3333_3333) + ((s >> 2) & 0x3333_3333_3333_3333);
    s = ((s + (s >> 4)) & 0x0F0F_0F0F_0F0F_0F0F).wrapping_mul(L8);

    let b = ((le8(s, n.wrapping_mul(L8)) >> 7).wrapping_mul(L8) >> 53) & !7;
    let l = n - ((s << 8).wrapping_shr(b as u32) & 0xFF);

    s = nz8((x.wrapping_shr(b as u32) & 0xFF).wrapping_mul(L8) & 0x8040_2010_0804_0201);
    s = (s >> 7).wrapping_mul(L8);

    (((le8(s, l * L8) >> 7).wrapping_mul(L8) >> 56) + b) as usize
}

macro_rules! impl_select_word_as_u64 {
    ( $( $Ty:ty )* ) => ($(
        impl SelectWord for $Ty {
            #[inline]
            fn select_word(self, c: usize) -> Option<usize> {
                (c < self.count1()).then(|| <u64 as SelectWord>::select_word(self as u64, c).unwrap())
            }
        }
    )*)
}
impl_select_word_as_u64!(u8 u16 u32);

impl SelectWord for u128 {
    /// ```
    /// # use bits::{BitsMut, Select};
    /// let mut n: u128 = 0;
    /// for i in (0..128).step_by(2) {
    ///     n.set_bit(i);
    /// }
    /// assert_eq!(n.select1(60), Some(120));
    /// assert_eq!(n.select1(61), Some(122));
    /// ```
    #[inline]
    fn select_word(self, c: usize) -> Option<usize> {
        let this: [u64; 2] = [self as u64, (self >> 64) as u64];
        this.select1(c)
    }
}
