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

pub fn conj<A>() -> impl Fold<A, Vec<A>, State = Vec<A>> + InitialState<Vec<A>> + Clone {
    let f = |mut acc: Vec<A>, item| {
        acc.push(item);
        Continue(acc)
    };
    f.beginning(|(lo, _hi)| Vec::with_capacity(lo.saturating_add(1)))
}

pub fn all<A, P>(mut pred: P) -> impl Fold<A, bool, State = bool> + InitialState<bool>
where
    P: FnMut(&A) -> bool,
{
    let f = move |_acc, item| {
        if pred(&item) { Continue(true) } else { Break(false) }
    };
    f.beginning(|_| true)
}

#[derive(Debug, Clone)]
pub struct Any<P> {
    pred: P,
}

impl<A, P> Fold<A, bool> for Any<P>
where
    P: FnMut(&A) -> bool,
{
    type State = bool;

    #[inline]
    fn step(&mut self, _acc: Self::State, item: A) -> ano::Step<Self::State> {
        if (self.pred)(&item) { Break(true) } else { Continue(false) }
    }

    #[inline]
    fn complete(self, acc: Self::State) -> bool {
        acc
    }
}

impl<P> InitialState<bool> for Any<P> {
    #[inline]
    fn initial_state(&self, _size_hint: (usize, Option<usize>)) -> bool {
        false
    }
}

pub fn any<A, P>(pred: P) -> Any<P>
where
    P: FnMut(&A) -> bool,
{
    Any { pred }
    // let f = move |_acc: bool, item: A| {
    //     if pred(&item) { Break(true) } else { Continue(false) }
    // };
    // f.using(|_| false)
}

pub fn count<A>() -> impl Fold<A, usize, State = usize> + InitialState<usize> + Clone {
    let f = |acc: usize, _item: A| Continue(acc + 1);
    f.beginning(|_| 0)
}

pub fn sum<A, B>() -> impl Fold<A, B, State = B> + InitialState<B> + Clone
where
    B: Default + Add<A, Output = B>,
{
    let f = |acc, item| Continue(acc + item);
    f.beginning(|_| B::default())
}

pub fn sum_rc<A, B>() -> impl Fold<Rc<A>, B, State = B> + InitialState<B> + Clone
where
    A: Copy,
    B: Default + Add<A, Output = B>,
{
    let f = |acc, item: Rc<A>| Continue(acc + *item.borrow());
    f.beginning(|_| B::default())
}
