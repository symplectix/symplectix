use crate::{Arith, ArithAssign, Bitwise, BitwiseAssign};

pub trait Int: Sized + Copy + Eq + Ord + Arith + ArithAssign + Bitwise + BitwiseAssign {
    const ZERO: Self;

    const ONE: Self;

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

macro_rules! impl_int {
    ($( $N:ty )*) => ($(
        impl Int for $N {
            const ZERO: Self = 0;

            const ONE: Self = 1;
        }
    )*)
}
impl_int!(i8 i16 i32 i64 i128 isize);
impl_int!(u8 u16 u32 u64 u128 usize);

// impl<T: Int> Int for num::Wrapping<T> {
//     const ZERO: Self = num::Wrapping(T::ZERO);
// }
// impl<T: Int> Int for num::Saturating<T> {
//     const ZERO: Self = num::Saturating(T::ZERO);
// }

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Add;

    // use core::iter::Step;
    // fn range<T: Int + Step>(init: T, max: T) -> impl Iterator<Item = T> {
    //     init..max
    // }

    fn range<T: Int + Add<Output = T>>(start: T, end: T) -> impl Iterator<Item = T> {
        use core::iter::successors;
        successors(Some(start), move |&x| (x < end).then_some(x + T::ONE))
    }

    #[test]
    fn range_test() {
        for (x, y) in range(0, 5).zip([0, 1, 2, 3, 4]) {
            assert_eq!(x, y);
        }
    }
}
