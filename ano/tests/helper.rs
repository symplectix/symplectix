#![allow(dead_code)]

use std::borrow::Borrow;
use std::ops::{AddAssign, Mul};
use std::rc::Rc;

pub fn pow2(x: i32) -> i32 {
    x.pow(2)
}

pub fn pow2_rc(x: Rc<i32>) -> i32 {
    pow2(*x.borrow())
}

pub fn mul3<T: Mul<i32>>(x: T) -> T::Output {
    x * 3
}

pub fn mul3_rc(x: Rc<i32>) -> i32 {
    mul3(x.borrow())
}

pub fn even(x: &i32) -> bool {
    x % 2 == 0
}

pub fn conj<A>(mut acc: Vec<A>, item: A) -> Vec<A> {
    acc.push(item);
    acc
}

pub fn _count<T>(acc: usize, _item: T) -> usize {
    acc + 1
}

pub fn _sum<A, B>(mut acc: B, item: A) -> B
where
    B: AddAssign<A>,
{
    acc += item;
    acc
}

pub fn _sum_rc(acc: i32, item: Rc<i32>) -> i32 {
    _sum(acc, item.borrow())
}
