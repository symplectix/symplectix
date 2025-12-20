#![allow(missing_docs)]

use std::iter::{empty, once};
use std::rc::Rc;

use ano::{Fold, xf};

mod helper;
use helper::*;

#[test]
fn map() {
    assert_eq!(vec![0; 0], xf::map(pow2).apply(conj()).fold(empty::<i32>()));
    assert_eq!(vec![81], xf::map(pow2).apply(conj()).fold(once::<i32>(9)));
    assert_eq!(vec![1, 4, 9], xf::map(pow2).apply(conj()).fold([1, 2, 3]));
    assert_eq!(vec![3, 6, 9], xf::map(mul3).apply(conj()).fold([1, 2, 3]));
    assert_eq!(vec![3, 6, 9], xf::map(mul3).apply(conj()).fold(&[1, 2, 3]));
}

#[test]
fn filter() {
    assert_eq!(vec![0; 0], xf::filter(even).apply(conj()).fold(empty::<i32>()));
    assert_eq!(vec![0; 0], xf::filter(even).apply(conj()).fold(once(1)));
    assert_eq!(vec![0; 0], xf::filter(even).apply(conj()).fold([1, 3, 5]));
    assert_eq!(vec![2, 4], xf::filter(even).apply(conj()).fold(1..6));
    assert_eq!(vec![&0; 0], xf::filter(even).apply(conj()).fold(&[1, 3, 5]));
}

#[test]
fn take() {
    let acc = xf::take(0).apply(conj()).fold(empty::<i32>());
    assert_eq!(acc, []);

    let acc = xf::take(0).apply(conj()).fold(1..);
    assert_eq!(acc, []);

    let acc = xf::take(1).apply(conj()).fold(empty::<i32>());
    assert_eq!(acc, []);

    let acc = xf::take(2).apply(conj()).fold(1..3);
    assert_eq!(acc, [1, 2]);

    let acc = xf::take(3).apply(conj()).fold(1..);
    assert_eq!(acc, [1, 2, 3]);

    let acc = xf::take(5).apply(conj()).fold(&[1, 2, 3]);
    assert_eq!(acc, [&1, &2, &3]);
}

#[test]
fn map_filter_take() {
    let acc = xf::map(mul3).take(5).filter(even).apply(conj()).fold_with(vec![], 1..);
    assert_eq!(acc, [6, 12]);
    let acc = xf::map(mul3).filter(even).take(5).apply(conj()).fold_with(vec![], 1..);
    assert_eq!(acc, [6, 12, 18, 24, 30]);

    let acc = xf::filter(even).map(mul3).take(5).apply(conj()).fold_with(vec![], 1..);
    assert_eq!(acc, [6, 12, 18, 24, 30]);
    let acc = xf::filter(even).take(5).map(mul3).apply(conj()).fold_with(vec![], 1..);
    assert_eq!(acc, [6, 12, 18, 24, 30]);

    let acc = xf::take(5).map(mul3).filter(even).apply(conj()).fold_with(vec![], 1..);
    assert_eq!(acc, [6, 12]);
    let acc = xf::take(5).filter(even).map(mul3).apply(conj()).fold_with(vec![], 1..);
    assert_eq!(acc, [6, 12]);
}

#[test]
fn count() {
    let f = || _count.using(|_| 0);
    assert_eq!(0, f().fold(empty::<i32>()));
    assert_eq!(9, f().fold(1..10));
    assert_eq!(3, xf::take(3).apply(f()).fold(1..));
}

#[test]
fn sum() {
    assert_eq!(0, _sum.fold_with(0, empty::<i32>()));
    assert_eq!(1, _sum.fold_with(1, empty::<i32>()));
    assert_eq!(1, _sum.fold_with(0, once::<i32>(1)));
    assert_eq!(2, _sum.fold_with(0, once::<i32>(2)));
    assert_eq!(18, xf::map(mul3).take(3).apply(_sum).fold_with(0, 1..));
}

#[test]
fn sum_using() {
    let f = |n| _sum.using(move |_| n);
    assert_eq!(0, f(0).fold(empty::<i32>()));
    assert_eq!(10, f(0).fold(1..5));
    assert_eq!(11, f(1).fold(1..5));
}

#[test]
fn sum_completing() {
    let f = |n| _sum.using(move |_| n).completing(|acc| acc + 10);
    assert_eq!(10, f(0).fold(empty::<i32>()));
    assert_eq!(20, f(0).fold(1..5));
    assert_eq!(21, f(1).fold(1..5));
}

#[test]
fn seq() {
    let f = xf::take(3).apply(conj());
    let g = xf::take(5).apply(conj());
    let acc = f.seq(g).fold_with((vec![], vec![]), 1..);
    assert_eq!(acc, (vec![1, 2, 3], vec![4, 5, 6, 7, 8]));
}

#[test]
fn par() {
    fn to_rcs<I>(iterable: I) -> impl Iterator<Item = Rc<I::Item>>
    where
        I: IntoIterator,
    {
        iterable.into_iter().map(Rc::new)
    }

    let f = xf::map(pow2_rc).take(3).apply(conj());
    let g = xf::map(mul3_rc).take(2).apply(_sum);
    let acc = f.par(g).fold_with((Vec::new(), 0), to_rcs(1..10));
    assert_eq!(acc, (vec![1, 4, 9], 9));

    let f = xf::take(5).apply(conj());
    let g = xf::take(5).apply(conj());
    let acc = f.par(g).fold_with((vec![], vec![]), &[1, 2, 3]);
    assert_eq!(acc, (vec![&1, &2, &3], vec![&1, &2, &3]));

    let f = _count.par(_sum_rc);
    let g = _count.par(_sum_rc);
    let (a, b) = f.seq(g).fold_with(((0, 0), (0, 0)), to_rcs([1, 2]));
    assert_eq!(a, (2, 3));
    assert_eq!(b, (0, 0));

    let f = _count.par(_sum_rc);
    let g = _count.par(_sum_rc);
    let (a, b) = f.par(g).fold_with(((0, 0), (0, 0)), to_rcs([1, 2]));
    assert_eq!(a, (2, 3));
    assert_eq!(b, (2, 3));

    let f = _count.par(_sum);
    let g = _count.par(_sum);
    let (a, b) = f.par(g).fold_with(((0, 0), (0, 0)), &[1, 2]);
    assert_eq!(a, (2, 3));
    assert_eq!(b, (2, 3));

    let f = xf::map(mul3_rc).take(3).apply(_sum);
    let g = xf::map(pow2_rc).take(3).apply(_sum);
    let (fsum, gsum) = f.par(g).fold_with((0, 0), to_rcs(1..));
    assert_eq!(fsum, 18);
    assert_eq!(gsum, 14);
}
