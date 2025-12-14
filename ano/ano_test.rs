#![allow(missing_docs)]

use std::borrow::{Borrow, ToOwned};
use std::collections::VecDeque;

use ano::{Fold, xf};

struct Conj;

impl<T> ano::Fold<T> for Conj
where
    T: ToOwned,
{
    type Acc = Vec<T::Owned>;

    fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> ano::Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        acc.push(input.borrow().to_owned());
        ano::Step::Yield(acc)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}

struct Cons;

impl<T> ano::Fold<T> for Cons
where
    T: ToOwned,
{
    type Acc = VecDeque<T::Owned>;

    fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> ano::Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        acc.push_front(input.borrow().to_owned());
        ano::Step::Yield(acc)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}

#[test]
fn map_filter_take() {
    let mul3 = |x: &i32| x * 3;
    let even = |x: &i32| x % 2 == 0;

    let acc = xf::map(mul3).take(5).filter(even).apply(Conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
    let acc = xf::map(mul3).filter(even).take(5).apply(Conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);

    let acc = xf::filter(even).map(mul3).take(5).apply(Conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);
    let acc = xf::filter(even).take(5).map(mul3).apply(Conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);

    let acc = xf::take(5).map(mul3).filter(even).apply(Conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
    let acc = xf::take(5).filter(even).map(mul3).apply(Conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
}

#[test]
fn map_filter_either() {
    let acc = xf::map(|x: &i32| x + 1)
        .filter(|x: &i32| *x % 2 == 0)
        .apply(Cons)
        .either(xf::folding().map(|x: &i32| x - 1).filter(|x: &i32| *x % 2 != 0).apply(Conj))
        .fold((VecDeque::with_capacity(10), vec![]), 0..10);
    assert_eq!(acc, (VecDeque::from([10, 8, 6, 4, 2]), vec![-1, 1, 3, 5, 7]));
}
