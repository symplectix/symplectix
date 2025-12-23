#![allow(missing_docs)]

use std::iter::{empty, once};
use std::rc::Rc;

use ano::{Fold, xf};

mod helper;
use helper::*;

#[test]
fn test_map() {
    assert_eq!(vec![0; 0], xf::map(pow2).apply(conj()).fold(empty::<i32>()));
    assert_eq!(vec![81], xf::map(pow2).apply(conj()).fold(once::<i32>(9)));
    assert_eq!(vec![1, 4, 9], xf::map(pow2).apply(conj()).fold([1, 2, 3]));
    assert_eq!(vec![3, 6, 9], xf::map(mul3).apply(conj()).fold([1, 2, 3]));
    assert_eq!(vec![3, 6, 9], xf::map(mul3).apply(conj()).fold(&[1, 2, 3]));

    assert_eq!(vec![0; 0], conj().map(pow2).fold(empty::<i32>()));
    assert_eq!(vec![81], conj().map(pow2).fold(once::<i32>(9)));
    assert_eq!(vec![1, 4, 9], conj().map(pow2).fold([1, 2, 3]));
    assert_eq!(vec![3, 6, 9], conj().map(mul3).fold([1, 2, 3]));
    assert_eq!(vec![3, 6, 9], conj().map(mul3).fold(&[1, 2, 3]));
}

#[test]
fn test_filter() {
    assert_eq!(vec![0; 0], xf::filter(even).apply(conj()).fold(empty::<i32>()));
    assert_eq!(vec![0; 0], xf::filter(even).apply(conj()).fold(once(1)));
    assert_eq!(vec![0; 0], xf::filter(even).apply(conj()).fold([1, 3, 5]));
    assert_eq!(vec![2, 4], xf::filter(even).apply(conj()).fold(1..6));
    assert_eq!(vec![&0; 0], xf::filter(even).apply(conj()).fold(&[1, 3, 5]));
}

#[test]
fn test_take() {
    assert_eq!(vec![0; 0], xf::take(0).apply(conj()).fold(empty::<i32>()));
    assert_eq!(vec![0; 0], xf::take(0).apply(conj()).fold(1..));
    assert_eq!(vec![0; 0], xf::take(1).apply(conj()).fold(empty::<i32>()));
    assert_eq!(vec![1, 2], xf::take(2).apply(conj()).fold(1..3));
    assert_eq!(vec![1, 2, 3], xf::take(3).apply(conj()).fold(1..));
    assert_eq!(vec![&1, &2, &3], xf::take(5).apply(conj()).fold(&[1, 2, 3]));
}

#[test]
fn test_map_filter_take() {
    assert_eq!(vec![6, 12], xf::map(mul3).take(5).filter(even).apply(conj()).fold(1..));
    assert_eq!(vec![6, 12, 18, 24, 30], xf::map(mul3).filter(even).take(5).apply(conj()).fold(1..));
    assert_eq!(vec![6, 12, 18, 24, 30], xf::filter(even).map(mul3).take(5).apply(conj()).fold(1..));
    assert_eq!(vec![6, 12, 18, 24, 30], xf::filter(even).take(5).map(mul3).apply(conj()).fold(1..));
    assert_eq!(vec![6, 12, 18, 24, 30], conj().map(mul3).take(5).filter(even).fold(1..));
    assert_eq!(vec![6, 12], xf::take(5).map(mul3).filter(even).apply(conj()).fold(1..));
    assert_eq!(vec![6, 12], xf::take(5).filter(even).map(mul3).apply(conj()).fold(1..));
}

#[test]
fn test_all() {
    assert!(all(even).fold(empty::<i32>()));
    assert!(all(even).fold([2, 4]));
    assert!(all(even).fold(&[2, 4]));
    assert!(!all(even).fold([1, 2]));
    assert!(!all(even).fold(&[1, 2]));
}

#[test]
fn test_any() {
    assert!(!any(even).fold(empty::<i32>()));
    assert!(any(even).fold([2, 4]));
    assert!(any(even).fold([1, 2]));
    assert!(any(even).fold(&[2, 4]));
    assert!(any(even).fold(&[1, 2]));
}

#[test]
fn test_count() {
    assert_eq!(0, count().fold(empty::<i32>()));
    assert_eq!(9, count().fold(1..10));
    assert_eq!(3, xf::take(3).apply(count()).fold(1..));
}

#[test]
fn test_sum() {
    assert_eq!(0, sum().fold(empty::<i32>()));
    assert_eq!(1, sum().fold(once::<i32>(1)));
    assert_eq!(2, sum().fold(once::<i32>(2)));
    assert_eq!(18, xf::map(mul3).take(3).apply(sum()).fold(1..));

    assert_eq!(10, sum::<i32, i32>().completing(|acc| acc + 10).fold(empty::<i32>()));
    assert_eq!(20, xf::take(4).apply(sum::<i32, i32>().completing(|acc| acc + 10)).fold(1..5));
    assert_eq!(20, xf::take(4).apply(sum::<i32, i32>()).completing(|acc| acc + 10).fold(1..5));
}

#[test]
fn test_sum_str() {
    assert_eq!("", sum::<_, String>().fold(empty::<&str>()));
    assert_eq!("1", sum::<_, String>().fold(once::<&str>("1")));
    assert_eq!("123", sum::<_, String>().fold(["foo1".trim_start_matches("foo"), "2", "3"]));
}

#[test]
fn test_using_using() {
    assert_eq!(110, sum().using(|_| 1).using(|_| 100).fold(1..5));
}

#[test]
fn test_seq() {
    assert_eq!(
        (vec![1, 2, 3], vec![4, 5, 6, 7, 8]),
        xf::take(3).apply(conj()).seq(xf::take(5).apply(conj())).fold(1..)
    );
}

#[test]
fn test_par() {
    let f = count().par(sum());
    let g = sum().par(count());
    let (a, b) = f.par(g).fold(&[1, 2]);
    assert_eq!(a, (2, 3));
    assert_eq!(b, (3, 2));

    let f = count().par(sum());
    let g = sum().par(count());
    let (a, b) = f.seq(g).fold(&[1, 2]);
    assert_eq!(a, (2, 3));
    assert_eq!(b, (0, 0));

    let f = xf::map(mul3).take(3).apply(conj());
    let g = xf::map(mul3).take(2).apply(sum());
    let acc = f.par(g).fold(&[1, 2, 3, 4]);
    assert_eq!(acc, (vec![3, 6, 9], 9));

    let f = xf::take(5).apply(conj());
    let g = xf::take(5).apply(conj());
    let acc = f.par(g).fold(&[1, 2, 3]);
    assert_eq!(acc, (vec![&1, &2, &3], vec![&1, &2, &3]));
}

#[test]
fn test_par_rc() {
    fn to_rcs<I>(iterable: I) -> impl Iterator<Item = Rc<I::Item>>
    where
        I: IntoIterator,
    {
        iterable.into_iter().map(Rc::new)
    }

    let f = xf::map(pow2_rc).take(3).apply(conj());
    let g = xf::map(mul3_rc).take(2).apply(sum());
    let acc = f.par(g).fold(to_rcs(1..10));
    assert_eq!(acc, (vec![1, 4, 9], 9));

    let f = count().par(sum_rc());
    let g = count().par(sum_rc());
    let ret = f.seq(g).fold(to_rcs([1, 2]));
    assert_eq!(ret, ((2, 3), (0, 0)));

    let f = count().par(sum_rc());
    let g = count().par(sum_rc());
    let (a, b) = f.par(g).fold(to_rcs([1, 2]));
    assert_eq!(a, (2, 3));
    assert_eq!(b, (2, 3));

    let f = xf::map(mul3_rc).take(3).apply(sum::<_, i32>());
    let g = xf::map(pow2_rc).take(3).apply(sum::<_, i32>());
    let (fsum, gsum) = f.par(g).fold(to_rcs(1..));
    assert_eq!(fsum, 18);
    assert_eq!(gsum, 14);
}
