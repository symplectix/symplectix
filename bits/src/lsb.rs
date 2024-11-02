pub trait Lsb: Sized + Copy {
    /// Least significant set bit (right most set bit).
    fn lsb(self) -> Self;
}

#[inline]
pub fn lsb<T: Lsb>(t: T) -> T {
    t.lsb()
}

macro_rules! ints_impl_lsb {
    ($( $N:ty )*) => ($(
        impl Lsb for $N {
            #[inline]
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }
        }
    )*)
}
ints_impl_lsb!(u8 u16 u32 u64 u128 usize);
ints_impl_lsb!(i8 i16 i32 i64 i128 isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsb() {
        let tests = [
            (0b_0000_0000_u8, 0b0000_0000),
            (0b_0000_0001_u8, 0b0000_0001),
            (0b_0000_1100_u8, 0b0000_0100),
            (0b_1001_0100_u8, 0b0000_0100),
            (0b_1001_0000_u8, 0b0001_0000),
        ];

        for (n, want) in tests {
            assert_eq!(n.lsb(), want);
            assert_eq!((n as i8).lsb(), want as i8);
        }
    }
}
