use std::ops::{Range, RangeBounds};

use crate::Bits;

/// Integer with a fixed-sized bits.
pub trait Word: num::PrimInt + crate::Block {
    const ZERO: Self;

    const ONE: Self;

    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;

    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;
}

/// A helper trait to implement `Select` for `Word`.
trait WordSelectHelper {
    fn select1(self, n: usize) -> Option<usize>;
}

impl WordSelectHelper for u64 {
    #[inline]
    fn select1(self, n: usize) -> Option<usize> {
        (n < self.count1()).then(|| {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            if is_x86_feature_detected!("bmi2") {
                use std::arch::x86_64::{_pdep_u64, _tzcnt_u64};
                return unsafe { _tzcnt_u64(_pdep_u64(1 << n, self)) as usize };
            }
            broadword(self, n as u64)
        })
    }
}

// Sebastiano Vigna, “Broadword Implementation of Rank/Select Queries”
// Returns 72 when not found.
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

impl WordSelectHelper for u8 {
    #[inline]
    fn select1(self, c: usize) -> Option<usize> {
        (c < self.count1()).then(|| <u64 as WordSelectHelper>::select1(self as u64, c).unwrap())
    }
}
impl WordSelectHelper for u16 {
    #[inline]
    fn select1(self, c: usize) -> Option<usize> {
        (c < self.count1()).then(|| <u64 as WordSelectHelper>::select1(self as u64, c).unwrap())
    }
}
impl WordSelectHelper for u32 {
    #[inline]
    fn select1(self, c: usize) -> Option<usize> {
        (c < self.count1()).then(|| <u64 as WordSelectHelper>::select1(self as u64, c).unwrap())
    }
}

impl WordSelectHelper for u128 {
    /// ```
    /// # use bits_core::{Bits, BitsMut};
    /// let mut n: u128 = 0;
    /// for i in (0..128).step_by(2) {
    ///     n.set1(i);
    /// }
    /// assert_eq!(n.select1(60), Some(120));
    /// assert_eq!(n.select1(61), Some(122));
    /// ```
    #[inline]
    fn select1(self, c: usize) -> Option<usize> {
        let this: [u64; 2] = [self as u64, (self >> 64) as u64];
        this.select1(c)
    }
}

impl WordSelectHelper for usize {
    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn select1(self, c: usize) -> Option<usize> {
        (self as u16).select_word(c)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn select1(self, c: usize) -> Option<usize> {
        (self as u32).select_word(c)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn select1(self, c: usize) -> Option<usize> {
        (self as u64).select1(c)
    }

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn select1(self, c: usize) -> Option<usize> {
        (self as u128).select_word(c)
    }
}

macro_rules! mask {
    ($( $Ty: ty, $i: expr, $j: expr )*) => ($(
        if $i >= $j {
            0
        } else {
            !0 >> (<$Ty>::BITS as usize - ($j - $i)) << $i
        }
    )*)
}

macro_rules! impls_for_word {
    ($( $Ty:ty )*) => ($(
        impl Word for $Ty {
            const ZERO: Self = 0;

            const ONE: Self = 1;

            #[inline]
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }

            #[inline]
            fn msb(self) -> Self {
                if self == 0 {
                    0
                } else {
                    let max = Self::BITS - 1;
                    1 << (max - self.leading_zeros())
                }
            }
        }

        impl crate::Bits for $Ty {
            #[inline]
            fn bits(&self) -> usize {
                <Self as crate::Block>::BITS
            }

            #[inline]
            fn test(&self, i: usize) -> Option<bool> {
                (i < Bits::bits(self)).then(|| (*self & (1 << i)) != 0)
            }

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
                *self == !0
            }

            #[inline]
            fn any(&self) -> bool {
                *self != 0
            }

            #[inline]
            fn rank1<R: RangeBounds<usize>>(&self, r: R) -> usize {
                let Range { start: i, end: j } = bit::bounded(&r, 0, Bits::bits(self));
                (*self & mask!($Ty, i, j)).count1()
            }

            #[inline]
            fn rank0<R: RangeBounds<usize>>(&self, r: R) -> usize {
                (!*self).rank1(r)
            }

            #[inline]
            fn select1(&self, n: usize) -> Option<usize> {
                <Self as WordSelectHelper>::select1(*self, n)
            }

            #[inline]
            fn select0(&self, n: usize) -> Option<usize> {
                <Self as WordSelectHelper>::select1(!self, n)
            }
        }

        impl crate::BitsMut for $Ty {
            #[inline]
            fn set1(&mut self, i: usize) {
                *self |= 1 << i;
            }
            #[inline]
            fn set0(&mut self, i: usize) {
                *self &= !(1 << i);
            }
        }

        impl crate::Block for $Ty {
            const BITS: usize = <$Ty>::BITS as usize;

            #[inline]
            fn empty() -> Self {
                0
            }
        }
    )*)
}
impls_for_word!(u8 u16 u32 u64 u128 usize);
