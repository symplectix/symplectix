#![allow(dead_code)]

use std::borrow::Borrow;
use std::ops::{AddAssign, Mul, Rem};
use std::rc::Rc;

use ano::{Fold, InitialState};

pub fn pow2(x: i32) -> i32 {
    x.pow(2)
}

pub fn pow2_rc(x: Rc<i32>) -> i32 {
    pow2(*x.borrow())
}

pub fn mul3<T>(x: T) -> T::Output
where
    T: Mul<i32>,
{
    x.mul(3)
}

pub fn mul3_rc(x: Rc<i32>) -> i32 {
    mul3::<i32>(*x.borrow())
}

pub fn even<T>(x: &T) -> bool
where
    T: Copy + Rem<i32, Output = i32>,
{
    x.rem(2) == 0
}

pub fn conj<A>() -> impl Fold<A, Vec<A>, Acc = Vec<A>> + InitialState<Vec<A>> {
    fn _conj<A>(mut acc: Vec<A>, item: A) -> Vec<A> {
        acc.push(item);
        acc
    }

    _conj.using(|(lo, _hi)| Vec::with_capacity(lo.saturating_add(1)))
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
