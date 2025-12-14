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
}

#[test]
fn test_map_filter_step() {
    let mut f = xf::map(|x: &i32| x + 1)
        .filter(|x: &i32| *x % 2 == 0)
        .apply(Cons)
        .either(xf::map(|x: &i32| x - 1).filter(|x: &i32| *x % 2 != 0).apply(Conj));
    let mut acc = (VecDeque::with_capacity(10), vec![]);
    for i in 0..10 {
        match f.step(acc, &i) {
            ano::Step::Yield(ret) => {
                acc = ret;
            }
            ano::Step::Break(ret) => {
                acc = f.done(ret);
                break;
            }
        }
    }
    assert_eq!(acc, (VecDeque::from([10, 8, 6, 4, 2]), vec![-1, 1, 3, 5, 7]));
}
