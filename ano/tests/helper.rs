#![allow(dead_code)]

use std::borrow::Borrow;
use std::ops::ControlFlow::*;
use std::ops::{Add, Mul, Rem};
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
    let f = |mut acc: Vec<A>, item| {
        acc.push(item);
        Continue(acc)
    };
    f.using(|(lo, _hi)| Vec::with_capacity(lo.saturating_add(1)))
}

pub fn count<A>() -> impl Fold<A, usize, Acc = usize> + InitialState<usize> {
    let f = |acc: usize, _item: A| Continue(acc + 1);
    f.using(|_| 0)
}

pub fn sum<A, B>() -> impl Fold<A, B, Acc = B> + InitialState<B>
where
    B: Default + Add<A, Output = B>,
{
    let f = |acc, item| Continue(acc + item);
    f.using(|_| B::default())
}

pub fn sum_rc<A, B>() -> impl Fold<Rc<A>, B, Acc = B> + InitialState<B>
where
    A: Copy,
    B: Default + Add<A, Output = B>,
{
    let f = |acc, item: Rc<A>| Continue(acc + *item.borrow());
    f.using(|_| B::default())
}
