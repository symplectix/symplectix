pub trait Msb: Sized + Copy {
    /// Most significant set bit (left most set bit).
    fn msb(self) -> Self;
}

macro_rules! ints_impl_msb {
    ($( $N:ty )*) => ($(
        impl Msb for $N {
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
    )*)
}
ints_impl_msb!(u8 u16 u32 u64 u128 usize);
ints_impl_msb!(i8 i16 i32 i64 i128 isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msb() {
        let tests = [
            (0b_0000_0000_u8, 0b_0000_0000_u8),
            (0b_0000_0001_u8, 0b_0000_0001_u8),
            (0b_0000_0011_u8, 0b_0000_0010_u8),
            (0b_0000_1100_u8, 0b_0000_1000_u8),
            (0b_1001_0100_u8, 0b_1000_0000_u8),
            (0b_1001_0000_u8, 0b_1000_0000_u8),
        ];

        for (n, want) in tests {
            assert_eq!(n.msb(), want);
            assert_eq!((n as i8).msb(), want as i8);
        }
    }

    #[test]
    fn uint_msb_assertion() {
        assert_eq!(0u8.msb(), 0);
        assert_eq!(1u8.msb(), 1);
        assert_eq!(2u8.msb(), 2);
        assert_eq!(3u8.msb(), 2);
        assert_eq!(4u8.msb(), 4);
        assert_eq!(5u8.msb(), 4);
        assert_eq!(6u8.msb(), 4);
        assert_eq!(7u8.msb(), 4);
        assert_eq!(8u8.msb(), 8);
        assert_eq!(9u8.msb(), 8);
        assert_eq!(10u8.msb(), 8);
        assert_eq!(15u8.msb(), 8);
        assert_eq!(16u8.msb(), 16);
        assert_eq!(18u8.msb(), 16);
        assert_eq!(30u8.msb(), 16);
        assert_eq!(33u8.msb(), 32);
    }

    #[test]
    fn sint_msb_assertion() {
        assert_eq!((-1i8).msb(), -128);

        assert_eq!(0i8.msb(), 0);
        assert_eq!(1i8.msb(), 1);
        assert_eq!(2i8.msb(), 2);
        assert_eq!(3i8.msb(), 2);
        assert_eq!(4i8.msb(), 4);
        assert_eq!(5i8.msb(), 4);
        assert_eq!(6i8.msb(), 4);
        assert_eq!(7i8.msb(), 4);
        assert_eq!(8i8.msb(), 8);
        assert_eq!(9i8.msb(), 8);
        assert_eq!(10i8.msb(), 8);
        assert_eq!(15i8.msb(), 8);
        assert_eq!(16i8.msb(), 16);
        assert_eq!(18i8.msb(), 16);
        assert_eq!(30i8.msb(), 16);
        assert_eq!(33i8.msb(), 32);
    }
}
