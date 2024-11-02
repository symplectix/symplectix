use core::{
    ops::{Add, Div, Mul, Rem, Sub},
    ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign},
};

pub trait Arith<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

impl<T, U, O> Arith<U, O> for T where
    T: Add<U, Output = O>
        + Div<U, Output = O>
        + Mul<U, Output = O>
        + Rem<U, Output = O>
        + Sub<U, Output = O>
{
}

pub trait ArithAssign<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

impl<T, U> ArithAssign<U> for T where
    T: AddAssign<U> + SubAssign<U> + MulAssign<U> + DivAssign<U> + RemAssign<U>
{
}
