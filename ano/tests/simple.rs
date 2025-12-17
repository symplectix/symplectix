#![allow(missing_docs)]

use std::iter::{empty, once};

use ano::{count, sum, xf, Fold};

mod helper;
use helper::*;

#[test]
fn map_empty() {
    let ret = xf::map(pow2).apply(conj).fold(vec![], empty::<i32>());
    assert_eq!(ret, vec![]);
}

#[test]
fn map_once() {
    let ret = xf::map(pow2).apply(conj).fold(vec![], once::<i32>(9));
    assert_eq!(ret, vec![81]);
}

#[test]
fn map_range() {
    let ret = xf::map(pow2).apply(conj).fold(vec![], 1..4);
    assert_eq!(ret, vec![1, 4, 9]);
}

#[test]
fn filter_empty() {
    let acc = vec![];
    let ret = xf::filter(even).apply(conj).fold(acc, empty::<i32>());
    assert_eq!(ret, vec![]);
}

#[test]
fn filter_once() {
    let acc = vec![];
    let ret = xf::filter(even).apply(conj).fold(acc, once(1));
    assert_eq!(ret, vec![]);
}

#[test]
fn filter_even_from_odd_nums() {
    let ret = xf::filter(even).apply(conj).fold(vec![], vec![1, 3, 5]);
    assert_eq!(ret, vec![]);
}

#[test]
fn filter_even_from_range() {
    let ret = xf::filter(even).apply(conj).fold(vec![], 1..6);
    assert_eq!(ret, vec![2, 4]);
}

#[test]
fn take_0_empty() {
    let acc = xf::take(0).apply(conj).fold(vec![], empty::<i32>());
    assert_eq!(acc, vec![]);
}

#[test]
fn take_0_inf() {
    let acc = xf::take(0).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![]);
}

#[test]
fn take_1_empty() {
    let acc = xf::take(1).apply(conj).fold(vec![], empty::<i32>());
    assert_eq!(acc, vec![]);
}

#[test]
fn take_all() {
    let acc = xf::take(2).apply(conj).fold(vec![], 1..3);
    assert_eq!(acc, vec![1, 2]);
}

#[test]
fn take_gt() {
    let acc = xf::take(5).apply(conj).fold(vec![], 1..3);
    assert_eq!(acc, vec![1, 2]);
}

#[test]
fn take_3_inf() {
    let acc = xf::take(3).apply(conj).fold(vec![], 1..);
    assert_eq!(acc, vec![1, 2, 3]);
}

#[test]
fn count_empty() {
    assert_eq!(0, count.fold(0, empty::<i32>()));
}

#[test]
fn count_all() {
    assert_eq!(9, count.fold(0, 1..10));
}

#[test]
fn count_take() {
    let acc = xf::take(3).apply(count).fold(0, 1..);
    assert_eq!(acc, 3);
}

#[test]
fn count_par_sum() {
    let f = count.par(sum);
    let (count, sum) = f.fold((0, 0), 1..3);
    assert_eq!(count, 2);
    assert_eq!(sum, 3);
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
fn sum_empty() {
    assert_eq!(0, sum.fold(0, empty::<i32>()));
    assert_eq!(1, sum.fold(1, empty::<i32>()));
}

#[test]
fn sum_once() {
    assert_eq!(1, sum.fold(0, once::<i32>(1)));
    assert_eq!(2, sum.fold(0, once::<i32>(2)));
}

#[test]
fn sum_take() {
    let acc = xf::map(mul3).take(3).apply(sum).fold(0, 1..);
    assert_eq!(acc, 18);
}

#[test]
fn sum_take_filter_map() {
    assert_eq!(4, sum.filter(even).map(pow2).take(3).fold(0, 1..));
}

#[test]
fn sum_par_sum() {
    let f = xf::map(mul3).take(3).apply(sum);
    let g = xf::map(pow2).take(3).apply(sum);
    let (fsum, gsum) = f.par(g).fold((0, 0), 1..);
    assert_eq!(fsum, 18);
    assert_eq!(gsum, 14);
}

#[test]
fn par_dup() {
    let f = conj.par(conj);
    let acc = f.fold((Vec::with_capacity(10), Vec::with_capacity(10)), 1..5);
    assert_eq!(acc, (vec![1, 2, 3, 4], vec![1, 2, 3, 4]));
}

#[test]
fn par_conj_sum() {
    let f = xf::map(pow2).take(3).apply(conj);
    let g = xf::map(mul3).take(2).apply(sum);
    let acc = f.par(g).fold((Vec::new(), 0), 1..10);
    assert_eq!(acc, (vec![1, 4, 9], 9));
}

#[test]
fn either_dup() {
    let f = conj.either(conj);
    let acc = f.fold((Vec::with_capacity(10), Vec::with_capacity(10)), 1..5);
    assert_eq!(acc, (vec![1, 2, 3, 4], vec![1, 2, 3, 4]));
}

#[test]
fn either_conj_sum() {
    let f = xf::map(pow2).take(3).apply(conj);
    let g = xf::map(mul3).take(2).apply(sum);
    let acc = f.either(g).fold((Vec::new(), 0), 1..10);
    assert_eq!(acc, (vec![1, 4], 9));
}
