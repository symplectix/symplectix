#![allow(missing_docs)]

use std::iter::{empty, once};

mod helper;
use helper::*;

#[test]
fn map() {
    let ret = xf::map(pow2).apply(conj).fold(vec![], empty::<i32>());
    assert_eq!(ret, vec![]);
    let ret = xf::map(pow2).apply(conj).fold(vec![], once::<i32>(9));
    assert_eq!(ret, vec![81]);
    let ret = xf::map(pow2).apply(conj).fold(vec![], 1..4);
    assert_eq!(ret, vec![1, 4, 9]);
}

#[test]
fn filter() {
    let acc = vec![];
    let ret = xf::filter(even).apply(conj).fold(acc, empty::<i32>());
    assert_eq!(ret, vec![]);
    let acc = vec![];
    let ret = xf::filter(even).apply(conj).fold(acc, once(1));
    assert_eq!(ret, vec![]);
    let acc = vec![];
    let ret = xf::filter(even).apply(conj).fold(acc, vec![1, 3, 5]);
    assert_eq!(ret, vec![]);
    let acc = vec![];
    let ret = xf::filter(even).apply(conj).fold(acc, 1..6);
    assert_eq!(ret, vec![2, 4]);
}

#[test]
fn take() {
    let acc = xf::take(0).apply(conj).fold(vec![], empty::<i32>());
    assert_eq!(acc, vec![]);
    let acc = xf::take(0).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![]);
    let acc = xf::take(1).apply(conj).fold(vec![], empty::<i32>());
    assert_eq!(acc, vec![]);
    let acc = xf::take(0).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![]);
    let acc = xf::take(2).apply(conj).fold(vec![], 1..3);
    assert_eq!(acc, vec![1, 2]);
    let acc = xf::take(5).apply(conj).fold(vec![], 1..3);
    assert_eq!(acc, vec![1, 2]);
    let acc = xf::take(3).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![1, 2, 3]);
}

#[test]
fn count() {
    assert_eq!(0, xf::count.fold(0, empty::<i32>()));
    assert_eq!(9, xf::count.fold(0, 1..10));

    let acc = xf::take(3).apply(xf::count).fold(0, 1..);
    assert_eq!(acc, 3);

    let f = xf::sum.par(xf::count);
    let (sum, count) = f.fold((0, 0), 1..3);
    assert_eq!(sum, 3);
    assert_eq!(count, 2);
}

#[test]
fn map_filter_take() {
    let acc = xf::map(mul3).take(5).filter(even).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
    let acc = xf::map(mul3).filter(even).take(5).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);

    let acc = xf::filter(even).map(mul3).take(5).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);
    let acc = xf::filter(even).take(5).map(mul3).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12, 18, 24, 30]);

    let acc = xf::take(5).map(mul3).filter(even).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
    let acc = xf::take(5).filter(even).map(mul3).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![6, 12]);
}

#[test]
fn sum() {
    let acc = xf::map(mul3).take(3).apply(xf::sum).fold(0, 1..);
    assert_eq!(acc, 18);

    let f = xf::map(mul3).take(3).apply(xf::sum);
    let g = xf::map(pow2).take(3).apply(xf::sum);
    let (fsum, gsum) = f.par(g).fold((0, 0), 1..);
    assert_eq!(fsum, 18);
    assert_eq!(gsum, 14);
}

#[test]
fn par() {
    let f = conj.par(conj);
    let acc = f.fold((Vec::with_capacity(10), Vec::with_capacity(10)), 1..5);
    assert_eq!(acc, (vec![1, 2, 3, 4], vec![1, 2, 3, 4]));

    let f = xf::map(pow2).take(3).apply(conj);
    let g = xf::map(mul3).take(2).apply(conj);
    let acc = f.par(g).fold((Vec::new(), Vec::new()), 1..10);
    assert_eq!(acc, (vec![1, 4, 9], vec![3, 6]));
}

#[test]
fn either() {
    let f = conj.either(conj);
    let acc = f.fold((Vec::with_capacity(10), Vec::with_capacity(10)), 1..5);
    assert_eq!(acc, (vec![1, 2, 3, 4], vec![1, 2, 3, 4]));

    let f = xf::map(pow2).take(3).apply(conj);
    let g = xf::map(mul3).take(2).apply(conj);
    let acc = f.either(g).fold((Vec::new(), Vec::new()), 1..10);
    assert_eq!(acc, (vec![1, 4], vec![3, 6]));
}
