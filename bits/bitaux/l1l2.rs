use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::*;

impl Debug for L1L2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("L1L2")
            .field(&self.l1())
            .field(&self.l2_0())
            .field(&self.l2_1())
            .field(&self.l2_2())
            .finish()
    }
}

impl L1L2 {
    pub const LEN: usize = 4;

    pub const L1: u64 = 0x_FFFF_FFFF; // lowest 32 bits
    pub const L2: u64 = 0x_03FF; // lowest 10 bits

    const L2_0_SHIFT: u64 = 32;
    const L2_1_SHIFT: u64 = 42;
    const L2_2_SHIFT: u64 = 52;

    #[inline]
    pub(crate) const fn merge(mut arr: [u64; Self::LEN]) -> Self {
        debug_assert!(arr[0] < Self::L1);
        debug_assert!(arr[1] < 1024 && arr[2] < 1024 && arr[3] < 1024);

        arr[0] |= arr[1] << Self::L2_0_SHIFT;
        arr[0] |= arr[2] << Self::L2_1_SHIFT;
        arr[0] |= arr[3] << Self::L2_2_SHIFT;
        L1L2(arr[0])
    }

    #[inline]
    pub(crate) const fn split(ll: Self) -> [u64; Self::LEN] {
        [ll.l1(), ll.l2_0(), ll.l2_1(), ll.l2_2()]
    }

    #[inline]
    pub(crate) const fn l1(self) -> u64 {
        let L1L2(l1l2) = self;
        l1l2 & Self::L1
    }

    #[inline]
    pub(crate) const fn l2_0(self) -> u64 {
        let L1L2(l1l2) = self;
        // (l1l2 & Self::MASK_L2_0) >> Self::L2_0_SHIFT
        (l1l2 >> Self::L2_0_SHIFT) & Self::L2
    }

    #[inline]
    pub(crate) const fn l2_1(self) -> u64 {
        let L1L2(l1l2) = self;
        // (l1l2 & Self::MASK_L2_1) >> Self::L2_1_SHIFT
        (l1l2 >> Self::L2_1_SHIFT) & Self::L2
    }

    #[inline]
    pub(crate) const fn l2_2(self) -> u64 {
        let L1L2(l1l2) = self;
        // (l1l2 & Self::MASK_L2_2) >> Self::L2_2_SHIFT
        (l1l2 >> Self::L2_2_SHIFT) & Self::L2
    }

    // Sum of basic blocks. [0,i)
    #[inline]
    pub(crate) fn l2(self, i: usize) -> u64 {
        match i {
            0 => 0,
            1 => self.l2_0(),
            2 => self.l2_0() + self.l2_1(),
            3 => self.l2_0() + self.l2_1() + self.l2_2(),
            _ => unreachable!("basic block: index out of bounds"),
        }
    }
}

// for lower_bound
impl PartialEq<u64> for L1L2 {
    #[inline]
    fn eq(&self, u: &u64) -> bool {
        self.l1().eq(u)
    }
}
impl PartialOrd<u64> for L1L2 {
    #[inline]
    fn partial_cmp(&self, u: &u64) -> Option<Ordering> {
        self.l1().partial_cmp(u)
    }
}

// for init
impl AddAssign<L1L2> for L1L2 {
    #[inline]
    fn add_assign(&mut self, delta: L1L2) {
        self.0 += delta.l1();
    }
}
// not used for now but here to keep symmetric
impl SubAssign<L1L2> for L1L2 {
    #[inline]
    fn sub_assign(&mut self, delta: L1L2) {
        self.0 -= delta.l1();
    }
}

// for add
impl AddAssign<u64> for L1L2 {
    #[inline]
    fn add_assign(&mut self, delta: u64) {
        self.0 += delta;
    }
}
// for sub
impl SubAssign<u64> for L1L2 {
    #[inline]
    fn sub_assign(&mut self, delta: u64) {
        self.0 -= delta;
    }
}

// lower_bound
impl From<L1L2> for u64 {
    #[inline]
    fn from(ll: L1L2) -> u64 {
        ll.l1()
    }
}

// for accum
impl Add<u64> for L1L2 {
    type Output = u64;
    #[inline]
    fn add(mut self, delta: u64) -> Self::Output {
        self += delta;
        self.l1()
    }
}
impl Sub<u64> for L1L2 {
    type Output = u64;
    #[inline]
    fn sub(mut self, delta: u64) -> Self::Output {
        self -= delta;
        self.l1()
    }
}

impl Sum<L1L2> for u64 {
    #[inline]
    fn sum<I: Iterator<Item = L1L2>>(iter: I) -> Self {
        iter.map(|l1l2| l1l2.l1()).sum()
    }
}

// not used for now but here to keep symmetric
impl Add<L1L2> for u64 {
    type Output = u64;
    #[inline]
    fn add(self, delta: L1L2) -> Self::Output {
        self + delta.l1()
    }
}
impl AddAssign<L1L2> for u64 {
    #[inline]
    fn add_assign(&mut self, delta: L1L2) {
        *self += delta.l1();
    }
}

impl Sub<L1L2> for u64 {
    type Output = u64;
    #[inline]
    fn sub(self, delta: L1L2) -> Self::Output {
        self - delta.l1()
    }
}
impl SubAssign<L1L2> for u64 {
    #[inline]
    fn sub_assign(&mut self, delta: L1L2) {
        *self -= delta.l1();
    }
}
