#![no_std]

use core::ops::{Div, Rem};

mod arith;
pub use arith::{Arith, ArithAssign};

mod bitwise;
pub use bitwise::{Bitwise, BitwiseAssign};

mod float;
pub use float::Float;

mod int;
pub use int::Int;

mod cast;
pub use cast::{cast, TryFromInt, TryFromSint, TryFromUint};

#[inline]
pub fn divrem<T, U>(t: T, u: U) -> (<T as Div<U>>::Output, <T as Rem<U>>::Output)
where
    T: Copy + Div<U> + Rem<U>,
    U: Copy,
{
    (t / u, t % u)
}
