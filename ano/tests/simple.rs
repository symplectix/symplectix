#![allow(missing_docs)]

use std::iter::{empty, once};
use std::rc::Rc;

use ano::{Fold, xf};

mod helper;
use helper::*;

#[test]
fn map() {
    let ret = xf::map(pow2).into_fn(conj).fold(vec![], empty::<i32>());
    assert_eq!(ret, []);

    let ret = xf::map(pow2).into_fn(conj).fold(vec![], once::<i32>(9));
    assert_eq!(ret, [81]);

    let ret = xf::map(pow2).into_fn(conj).fold(vec![], [1, 2, 3]);
    assert_eq!(ret, [1, 4, 9]);

    let ret = xf::map(mul3).into_fn(conj).fold(vec![], &[1, 2, 3]);
    assert_eq!(ret, [3, 6, 9]);
}

#[test]
fn filter() {
    let ret = xf::filter(even).into_fn(conj).fold(vec![], empty::<i32>());
    assert_eq!(ret, []);

    let ret = xf::filter(even).into_fn(conj).fold(vec![], once(1));
    assert_eq!(ret, []);

    let ret = xf::filter(even).into_fn(conj).fold(vec![], [1, 3, 5]);
    assert_eq!(ret, []);

    let ret = xf::filter(even).into_fn(conj).fold(vec![], 1..6);
    assert_eq!(ret, [2, 4]);
}

#[test]
fn take() {
    let acc = xf::take(0).into_fn(conj).fold(vec![], empty::<i32>());
    assert_eq!(acc, []);

    let acc = xf::take(0).into_fn(conj).fold(vec![], 1..);
    assert_eq!(acc, []);

    let acc = xf::take(1).into_fn(conj).fold(vec![], empty::<i32>());
    assert_eq!(acc, []);

    let acc = xf::take(2).into_fn(conj).fold(vec![], 1..3);
    assert_eq!(acc, [1, 2]);

    let acc = xf::take(3).into_fn(conj).fold(vec![], 1..);
    assert_eq!(acc, [1, 2, 3]);

    let acc = xf::take(5).into_fn(conj).fold(vec![], &[1, 2, 3]);
    assert_eq!(acc, [&1, &2, &3]);
}

#[test]
fn count() {
    assert_eq!(0, ano::from_fn(_count).fold(0, empty::<i32>()));
    assert_eq!(9, ano::from_fn(_count).fold(0, 1..10));
    assert_eq!(3, xf::take(3).into_fn(_count).fold(0, 1..));
}

#[test]
fn map_filter_take() {
    let acc = xf::map(mul3).take(5).filter(even).into_fn(conj).fold(vec![], 1..);
    assert_eq!(acc, [6, 12]);
    let acc = xf::map(mul3).filter(even).take(5).into_fn(conj).fold(vec![], 1..);
    assert_eq!(acc, [6, 12, 18, 24, 30]);

    let acc = xf::filter(even).map(mul3).take(5).into_fn(conj).fold(vec![], 1..);
    assert_eq!(acc, [6, 12, 18, 24, 30]);
    let acc = xf::filter(even).take(5).map(mul3).into_fn(conj).fold(vec![], 1..);
    assert_eq!(acc, [6, 12, 18, 24, 30]);

    let acc = xf::take(5).map(mul3).filter(even).into_fn(conj).fold(vec![], 1..);
    assert_eq!(acc, [6, 12]);
    let acc = xf::take(5).filter(even).map(mul3).into_fn(conj).fold(vec![], 1..);
    assert_eq!(acc, [6, 12]);
}

#[test]
fn sum() {
    assert_eq!(0, ano::from_fn(_sum).fold(0, empty::<i32>()));
    assert_eq!(1, ano::from_fn(_sum).fold(1, empty::<i32>()));
    assert_eq!(1, ano::from_fn(_sum).fold(0, once::<i32>(1)));
    assert_eq!(2, ano::from_fn(_sum).fold(0, once::<i32>(2)));
    assert_eq!(18, xf::map(mul3).take(3).into_fn(_sum).fold(0, 1..));
}

#[test]
fn seq() {
    let f = xf::take(3).into_fn(conj);
    let g = xf::take(5).into_fn(conj);
    let acc = f.seq(g).fold((vec![], vec![]), 1..);
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

    let f = xf::map(pow2_rc).take(3).into_fn(conj);
    let g = xf::map(mul3_rc).take(2).into_fn(_sum);
    let acc = f.par(g).fold((Vec::new(), 0), to_rcs(1..10));
    assert_eq!(acc, (vec![1, 4, 9], 9));

    let f = xf::take(5).into_fn(conj);
    let g = xf::take(5).into_fn(conj);
    let acc = f.par(g).fold((vec![], vec![]), &[1, 2, 3]);
    assert_eq!(acc, (vec![&1, &2, &3], vec![&1, &2, &3]));

    let f = ano::from_fn(_count).par(ano::from_fn(_sum_rc));
    let g = ano::from_fn(_count).par(ano::from_fn(_sum_rc));
    let (a, b) = f.seq(g).fold(((0, 0), (0, 0)), to_rcs([1, 2]));
    assert_eq!(a, (2, 3));
    assert_eq!(b, (0, 0));

    let f = ano::from_fn(_count).par(ano::from_fn(_sum_rc));
    let g = ano::from_fn(_count).par(ano::from_fn(_sum_rc));
    let (a, b) = f.par(g).fold(((0, 0), (0, 0)), to_rcs([1, 2]));
    assert_eq!(a, (2, 3));
    assert_eq!(b, (2, 3));

    let f = ano::from_fn(_count).par(ano::from_fn(_sum));
    let g = ano::from_fn(_count).par(ano::from_fn(_sum));
    let (a, b) = f.par(g).fold(((0, 0), (0, 0)), &[1, 2]);
    assert_eq!(a, (2, 3));
    assert_eq!(b, (2, 3));

    let f = xf::map(mul3_rc).take(3).into_fn(_sum);
    let g = xf::map(pow2_rc).take(3).into_fn(_sum);
    let (fsum, gsum) = f.par(g).fold((0, 0), to_rcs(1..));
    assert_eq!(fsum, 18);
    assert_eq!(gsum, 14);
}
