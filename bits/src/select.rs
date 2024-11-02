use crate::{Bits, Rank};

pub trait Select: Rank {
    /// Returns the position of the n-th 1, indexed starting from zero.
    /// `n` must be less than `self.count1()`, orherwise returns `None`.
    #[inline]
    fn select1(&self, n: usize) -> Option<usize> {
        helper::search1(self, n)
    }

    /// Returns the position of the n-th 0, indexed starting from zero.
    /// `n` must be less than `self.count0()`, orherwise returns `None`.
    #[inline]
    fn select0(&self, n: usize) -> Option<usize> {
        helper::search0(self, n)
    }

    // #[inline]
    // fn select1_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select1(self.rank1(..i) + n).map(|pos| pos - i)
    // }

    // #[inline]
    // fn select0_from(&self, i: usize, n: usize) -> Option<usize> {
    //     self.select0(self.rank0(..i) + n).map(|pos| pos - i)
    // }
}

mod helper {
    use crate::Rank;

    /// Binary search to find and return the smallest index k in `[i, j)` at which f(k) is true,
    /// assuming that on the range `[i, j)`, f(k) == true implies f(k+1) == true.
    ///
    /// Returns the first true index, if there is no such index, returns `j`.
    fn binary_search<P: Fn(usize) -> bool>(mut l: usize, mut r: usize, f: P) -> usize {
        while l < r {
            let m = l + (r - l) / 2;
            if f(m) {
                r = m; // -> f(r) == true
            } else {
                l = m + 1; // -> f(l-1) == false
            }
        }
        l // f(l-1) == false && f(l) (= f(r)) == true
    }

    #[inline]
    pub fn search1<T: ?Sized + Rank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count1()).then(|| binary_search(0, bs.bits(), |k| bs.rank1(..k) > n) - 1)
    }

    #[inline]
    pub fn search0<T: ?Sized + Rank>(bs: &T, n: usize) -> Option<usize> {
        (n < bs.count0()).then(|| binary_search(0, bs.bits(), |k| bs.rank0(..k) > n) - 1)
    }
}

macro_rules! ints_impl_select {
    ($( $Int:ty )*) => ($(
        impl Select for $Int {
            #[inline]
            fn select1(&self, n: usize) -> Option<usize> {
                <Self as int_select_helper::IntSelectHelper>::select1(*self, n)
            }

            #[inline]
            fn select0(&self, n: usize) -> Option<usize> {
                <Self as int_select_helper::IntSelectHelper>::select1(!self, n)
            }
        }
    )*)
}
ints_impl_select!(u8 u16 u32 u64 u128 usize);
ints_impl_select!(i8 i16 i32 i64 i128 isize);

mod int_select_helper {
    use crate::{Count, Select};

    /// A helper trait to implement `Select` for u64.
    pub(crate) trait IntSelectHelper {
        fn select1(self, n: usize) -> Option<usize>;
    }

    impl IntSelectHelper for u64 {
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

    macro_rules! impl_select_helper_as_u64 {
        ( $( $Ty:ty )* ) => ($(
            impl IntSelectHelper for $Ty {
                #[inline]
                fn select1(self, c: usize) -> Option<usize> {
                    (c < self.count1()).then(|| <u64 as IntSelectHelper>::select1(self as u64, c).unwrap())
                }
            }
        )*)
    }
    impl_select_helper_as_u64!(u8 u16 u32);

    impl IntSelectHelper for u128 {
        /// ```
        /// # use bits::{ContainerMut, Select};
        /// let mut n: u128 = 0;
        /// for i in (0..128).step_by(2) {
        ///     n.set_bit(i);
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

    impl IntSelectHelper for usize {
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

    macro_rules! sint_impl_select_helper {
        ( $( ($T1:ty, $T2:ty), )* ) => ($(
            impl IntSelectHelper for $T1 {
                #[inline]
                fn select1(self, c: usize) -> Option<usize> {
                    (c < self.count1()).then(|| <$T2 as IntSelectHelper>::select1(self as $T2, c).unwrap())
                }
            }
        )*)
    }
    sint_impl_select_helper!(
        (i8, u8),
        (i16, u16),
        (i32, u32),
        (i64, u64),
        (i128, u128),
        (isize, usize),
    );
}

impl<B: Bits> Select for [B] {
    #[inline]
    fn select1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count1();
            if n < count {
                return Some(i * B::BITS + b.select1(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }

    #[inline]
    fn select0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count0();
            if n < count {
                return Some(i * B::BITS + b.select0(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }
}

macro_rules! impl_select {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn select1(&self, n: usize) -> Option<usize> {
            <$X as Select>::select1(self$(.$method())?, n)
        }

        #[inline]
        fn select0(&self, n: usize) -> Option<usize> {
            <$X as Select>::select0(self$(.$method())?, n)
        }
    }
}

impl<'a, T: ?Sized + Select> Select for &'a T {
    impl_select!(T);
}

impl<B, const N: usize> Select for [B; N]
where
    [B]: Select,
{
    impl_select!([B], as_ref);
}

mod impl_alloc {
    use super::*;
    use std::borrow::{Cow, ToOwned};
    use std::boxed::Box;
    use std::vec::Vec;

    impl<B> Select for Vec<B>
    where
        [B]: Select,
    {
        impl_select!([B]);
    }

    impl<T: ?Sized + Select> Select for Box<T> {
        impl_select!(T);
    }

    impl<'a, T> Select for Cow<'a, T>
    where
        T: ?Sized + ToOwned + Select,
    {
        impl_select!(T, as_ref);
    }
}
