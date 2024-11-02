use core::fmt::Debug;

use bits::Lsb;
use num::Int;

trait Pdep {
    fn pdep(self, mask: Self) -> Self;
}

impl Pdep for u32 {
    fn pdep(self, mask: Self) -> Self {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("bmi2") {
                return unsafe { pdep_u32_bmi2(self, mask) };
            }
        }

        _pdep(self, mask)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "bmi2")]
unsafe fn pdep_u32_bmi2(n: u32, mask: u32) -> u32 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_pdep_u32;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_pdep_u32;

    unsafe { _pdep_u32(n, mask) }
}

impl Pdep for u64 {
    fn pdep(self, mask: Self) -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("bmi2") {
                return unsafe { pdep_u64_bmi2(self, mask) };
            }
        }
        _pdep(self, mask)
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "bmi2")]
unsafe fn pdep_u64_bmi2(n: u64, mask: u64) -> u64 {
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_pdep_u64;

    unsafe { _pdep_u64(n, mask) }
}

fn _pdep<T: Int + num::Arith + num::BitwiseAssign + bits::Bits + Lsb>(data: T, mut mask: T) -> T {
    let mut dest = T::ZERO;
    for i in 0..<T as bits::Bits>::BITS {
        if !mask.any() {
            break;
        }
        if data.bit(i).unwrap() {
            dest |= mask.lsb();
        }
        mask &= mask - T::ONE;
    }
    dest
}

fn pdep_test<T>(s: T, m: T, o: T)
where
    T: Int + num::Arith + num::BitwiseAssign + bits::Bits + Lsb + Pdep + Debug,
{
    assert_eq!(s.pdep(m), o);
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("bmi2") {
        assert_eq!(s.pdep(m), _pdep(s, m));
    }
}

macro_rules! pdep_test {
    ($T:ty) => {
        pdep_test::<$T>(0b_1011111010010011, 0b_0110001110000101, 0b_0000001000000101);

        pdep_test::<$T>(0b_0000000000000000, 0b_0110001110000100, 0b_0000000000000000);

        pdep_test::<$T>(0b_0000000000001000, 0b_0110001110000100, 0b_0000001000000000);

        pdep_test::<$T>(0b_0000000000010000, 0b_0110001110000100, 0b_0010000000000000);

        pdep_test::<$T>(0b_0000000000100000, 0b_0110001110000100, 0b_0100000000000000);
    };
}

#[test]
fn pdep_u32() {
    pdep_test!(u32);
}

#[test]
fn pdep_u64() {
    pdep_test!(u64);
}
