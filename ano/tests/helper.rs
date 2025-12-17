#![allow(dead_code)]

use std::borrow::ToOwned;

pub fn pow2(x: &i32) -> i32 {
    x.pow(2)
}

pub fn mul3(x: &i32) -> i32 {
    x * 3
}

pub fn even(x: &i32) -> bool {
    x % 2 == 0
}

#[inline]
pub fn conj<A>(mut acc: Vec<A::Owned>, input: &A) -> Vec<A::Owned>
where
    A: ToOwned,
{
    acc.push(input.to_owned());
    acc
}

#[inline]
pub fn count<T>(acc: usize, _input: &T) -> usize {
    acc + 1
}

#[inline]
pub fn sum<A, B>(mut acc: B, input: &A) -> B
where
    B: for<'a> std::ops::AddAssign<&'a A>,
{
    acc += input;
    acc
}
