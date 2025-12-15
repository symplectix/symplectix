#![allow(missing_docs)]

use std::borrow::{Borrow, ToOwned};
use std::collections::VecDeque;
use std::iter::{empty, once};
use std::marker::PhantomData;
use std::ops::AddAssign;

use ano::{Fold, xf};

#[derive(Debug)]
struct Sum<T>(PhantomData<T>);
impl<A> Sum<A> {
    fn new() -> Self {
        Sum(PhantomData)
    }
}
impl<S, A> ano::Fold<A, S> for Sum<S>
where
    S: for<'a> AddAssign<&'a A>,
{
    type Acc = S;

    #[inline]
    fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> ano::Step<Self::Acc>
    where
        Q: Borrow<A>,
    {
        acc += input.borrow();
        ano::Step::Yield(acc)
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}

#[derive(Debug)]
struct Count;
impl Count {
    fn new() -> Self {
        Count
    }
}
impl<A> ano::Fold<A, usize> for Count {
    type Acc = usize;

    #[inline]
    fn step<In>(&mut self, mut acc: Self::Acc, _input: &In) -> ano::Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        acc += 1;
        ano::Step::Yield(acc)
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}

#[derive(Debug)]
struct Conj<T>(PhantomData<T>);

impl<In> ano::Fold<In, Vec<In::Owned>> for Conj<In>
where
    In: ToOwned,
{
    type Acc = Vec<In::Owned>;

    fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> ano::Step<Self::Acc>
    where
        Q: Borrow<In>,
    {
        acc.push(input.borrow().to_owned());
        ano::Step::Yield(acc)
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}

fn cons<T>() -> Cons<T> {
    Cons(PhantomData)
}

#[derive(Debug)]
struct Cons<T>(PhantomData<T>);

impl<In> ano::Fold<In, VecDeque<In::Owned>> for Cons<In>
where
    In: ToOwned,
{
    type Acc = VecDeque<In::Owned>;
    fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> ano::Step<Self::Acc>
    where
        Q: Borrow<In>,
    {
        acc.push_front(input.borrow().to_owned());
        ano::Step::Yield(acc)
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}

fn conj<T>() -> Conj<T> {
    Conj(PhantomData)
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
    let acc = vec![];
    let ret = xf::map(pow2).apply(conj()).fold(acc, empty::<i32>());
    assert_eq!(ret, vec![]);
    let acc = vec![];
    let ret = xf::map(pow2).apply(conj()).fold(acc, 1..4);
    assert_eq!(ret, vec![1, 4, 9]);
}

#[test]
fn filter() {
    let acc = vec![];
    let ret = xf::filter(even).apply(conj()).fold(acc, empty::<i32>());
    assert_eq!(ret, vec![]);
    let acc = vec![];
    let ret = xf::filter(even).apply(conj()).fold(acc, once(1));
    assert_eq!(ret, vec![]);
    let acc = vec![];
    let ret = xf::filter(even).apply(conj()).fold(acc, vec![1, 3, 5]);
    assert_eq!(ret, vec![]);
    let acc = vec![];
    let ret = xf::filter(even).apply(conj()).fold(acc, 1..6);
    assert_eq!(ret, vec![2, 4]);
}

#[test]
fn take() {
    let acc = xf::take(0).apply(conj()).fold(vec![], empty::<i32>());
    assert_eq!(acc, vec![]);
    let acc = xf::take(0).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![]);
    let acc = xf::take(1).apply(conj()).fold(vec![], empty::<i32>());
    assert_eq!(acc, vec![]);
    let acc = xf::take(0).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![]);
    let acc = xf::take(2).apply(conj()).fold(vec![], 1..3);
    assert_eq!(acc, vec![1, 2]);
    let acc = xf::take(5).apply(conj()).fold(vec![], 1..3);
    assert_eq!(acc, vec![1, 2]);
    let acc = xf::take(3).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![1, 2, 3]);
}

#[test]
fn map_filter_take() {
    let acc = xf::map(mul3).take(5).filter(even).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
    let acc = xf::map(mul3).filter(even).take(5).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);

    let acc = xf::filter(even).map(mul3).take(5).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);
    let acc = xf::filter(even).take(5).map(mul3).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);

    let acc = xf::take(5).map(mul3).filter(even).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
    let acc = xf::take(5).filter(even).map(mul3).apply(conj()).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
}

#[test]
fn count() {
    assert_eq!(0, Count::new().fold(0, empty::<i32>()));
    assert_eq!(9, Count::new().fold(0, 1..10));

    let acc = xf::take(3).apply(Count::new()).fold(0, 1..);
    assert_eq!(acc, 3);

    let f = Sum::<usize>::new().par(Count::new());
    let (sum, count) = f.fold((0, 0), 1..3);
    assert_eq!(sum, 3);
    assert_eq!(count, 2);
}

#[test]
fn sum() {
    let acc = xf::map(mul3).take(3).apply(Sum::new()).fold(0, 1..);
    assert_eq!(acc, 18);

    let f = xf::map(mul3).take(3).apply(Sum::new());
    let g = xf::map(pow2).take(3).apply(Sum::new());
    let (fsum, gsum) = f.par(g).fold((0, 0), 1..);
    assert_eq!(fsum, 18);
    assert_eq!(gsum, 14);
}

#[test]
fn par() {
    let f = cons().par(conj());
    let acc = f.fold((VecDeque::with_capacity(10), Vec::with_capacity(10)), 1..5);
    assert_eq!(acc, (VecDeque::from([4, 3, 2, 1]), vec![1, 2, 3, 4]));

    let f = xf::map(pow2).take(3).apply(cons());
    let g = xf::map(mul3).take(2).apply(conj());
    let acc = f.par(g).fold((VecDeque::new(), Vec::new()), 1..10);
    assert_eq!(acc, (VecDeque::from([9, 4, 1]), vec![3, 6]));
}

#[test]
fn either() {
    let f = cons().either(conj());
    let acc = f.fold((VecDeque::with_capacity(10), Vec::with_capacity(10)), 1..5);
    assert_eq!(acc, (VecDeque::from([4, 3, 2, 1]), vec![1, 2, 3, 4]));

    let f = xf::map(pow2).take(3).apply(cons());
    let g = xf::map(mul3).take(2).apply(conj());
    let acc = f.either(g).fold((VecDeque::new(), Vec::new()), 1..10);
    assert_eq!(acc, (VecDeque::from([4, 1]), vec![3, 6]));
}
