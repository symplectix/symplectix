#![allow(missing_docs)]

use std::borrow::{Borrow, ToOwned};
use std::collections::VecDeque;
use std::iter::{empty, once};

use ano::{Fold, xf};

struct Conj<T> {
    acc: Vec<T>,
}
impl<T> Conj<T> {
    fn new(capacity: usize) -> Self {
        Conj { acc: Vec::with_capacity(capacity) }
    }
}

impl<In> ano::Fold<In, Vec<In::Owned>> for Conj<In::Owned>
where
    In: ToOwned,
{
    fn step<Q>(&mut self, input: &Q) -> ano::Step
    where
        Q: Borrow<In>,
    {
        self.acc.push(input.borrow().to_owned());
        ano::Step::Yield
    }

    #[inline]
    fn done(self) -> Vec<In::Owned> {
        self.acc
    }
}

struct Cons<T> {
    acc: VecDeque<T>,
}
impl<T> Cons<T> {
    fn new(capacity: usize) -> Self {
        Cons { acc: VecDeque::with_capacity(capacity) }
    }
}

impl<In> ano::Fold<In, VecDeque<In::Owned>> for Cons<In::Owned>
where
    In: ToOwned,
{
    fn step<Q>(&mut self, input: &Q) -> ano::Step
    where
        Q: Borrow<In>,
    {
        self.acc.push_front(input.borrow().to_owned());
        ano::Step::Yield
    }

    #[inline]
    fn done(self) -> VecDeque<In::Owned> {
        self.acc
    }
}

fn pow2(x: &i32) -> i32 {
    x.pow(2)
}

fn mul3(x: &i32) -> i32 {
    x * 3
}

fn even(x: &i32) -> bool {
    x % 2 == 0
}

#[test]
fn map() {
    let acc = xf::map(pow2).apply(Conj::new(10)).fold(empty::<i32>());
    assert_eq!(acc, vec![]);
    let acc = xf::map(pow2).apply(Conj::new(10)).fold(1..4);
    assert_eq!(acc, vec![1, 4, 9]);
}

#[test]
fn filter() {
    let acc = xf::filter(even).apply(Conj::new(10)).fold(empty::<i32>());
    assert_eq!(acc, vec![]);
    let acc = xf::filter(even).apply(Conj::new(10)).fold(once(1));
    assert_eq!(acc, vec![]);
    let acc = xf::filter(even).apply(Conj::new(10)).fold(vec![1, 3, 5]);
    assert_eq!(acc, vec![]);
    let acc = xf::filter(even).apply(Conj::new(10)).fold(1..6);
    assert_eq!(acc, vec![2, 4]);
}

#[test]
fn take() {
    let acc = xf::take(0).apply(Conj::new(10)).fold(empty::<i32>());
    assert_eq!(acc, vec![]);
    let acc = xf::take(0).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![]);
    let acc = xf::take(1).apply(Conj::new(10)).fold(empty::<i32>());
    assert_eq!(acc, vec![]);
    let acc = xf::take(0).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![]);
    let acc = xf::take(2).apply(Conj::new(10)).fold(1..3);
    assert_eq!(acc, vec![1, 2]);
    let acc = xf::take(5).apply(Conj::new(10)).fold(1..3);
    assert_eq!(acc, vec![1, 2]);
    let acc = xf::take(3).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![1, 2, 3]);
}

#[test]
fn map_filter_take() {
    let acc = xf::map(mul3).take(5).filter(even).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![6, 12]);
    let acc = xf::map(mul3).filter(even).take(5).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);

    let acc = xf::filter(even).map(mul3).take(5).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);
    let acc = xf::filter(even).take(5).map(mul3).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);

    let acc = xf::take(5).map(mul3).filter(even).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![6, 12]);
    let acc = xf::take(5).filter(even).map(mul3).apply(Conj::new(10)).fold(1..);
    assert_eq!(acc, vec![6, 12]);
}

#[test]
fn either() {
    // let acc = Cons::new(10).either(Conj::new(10)).fold(1..5);
    let acc = <Cons<i32> as Fold<i32, VecDeque<_>>>::either::<Conj<i32>>(Cons::new(10), Conj::new(10)).fold(1..5);
    assert_eq!(acc, (VecDeque::from([4, 3, 2, 1]), vec![1, 2, 3, 4]));
}
