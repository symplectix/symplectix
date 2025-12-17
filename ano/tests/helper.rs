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
pub fn conj<A>(mut acc: Vec<A::Owned>, item: &A) -> Vec<A::Owned>
where
    A: ToOwned,
{
    acc.push(item.to_owned());
    acc
}

#[inline]
pub fn _count<T>(acc: usize, _item: &T) -> usize {
    acc + 1
}

#[inline]
pub fn _sum<A, B>(mut acc: B, item: &A) -> B
where
    B: for<'a> std::ops::AddAssign<&'a A>,
{
    acc += item;
    acc
}
