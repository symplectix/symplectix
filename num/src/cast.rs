use crate::Int;

#[inline]
pub fn cast<T, N>(this: T) -> N
where
    N: Int + TryFrom<T>,
{
    N::try_from(this).ok().expect("num::cast failed")
}

pub trait TryFromInt: TryFromSint + TryFromUint {}

impl<T> TryFromInt for T where T: TryFromSint + TryFromUint {}

pub trait TryFromSint:
    Int + TryFrom<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64> + TryFrom<i128> + TryFrom<isize>
{
}

pub trait TryFromUint:
    Int + TryFrom<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128> + TryFrom<usize>
{
}

impl<T> TryFromSint for T where
    T: Int
        + TryFrom<i8>
        + TryFrom<i16>
        + TryFrom<i32>
        + TryFrom<i64>
        + TryFrom<i128>
        + TryFrom<isize>
{
}

impl<T> TryFromUint for T where
    T: Int
        + TryFrom<u8>
        + TryFrom<u16>
        + TryFrom<u32>
        + TryFrom<u64>
        + TryFrom<u128>
        + TryFrom<usize>
{
}
