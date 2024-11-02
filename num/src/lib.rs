#![no_std]

use core::ops::{Div, Rem};

mod int;
pub use int::{Int, TryFromInt, TryFromSint, TryFromUint};

#[inline]
pub fn cast<T, N>(this: T) -> N
where
    N: Int + TryFrom<T>,
{
    N::try_from(this).ok().expect("num::cast failed")
}

#[inline]
pub fn divrem<T, U>(t: T, u: U) -> (<T as Div<U>>::Output, <T as Rem<U>>::Output)
where
    T: Copy + Div<U> + Rem<U>,
    U: Copy,
{
    (t / u, t % u)
}
