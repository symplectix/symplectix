#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{iter, ops};

use bits::Word;
use fenwicktree::{Incr, LowerBound, Nodes, Prefix};

fn build<T: Word + ops::AddAssign>(mut vec: Vec<T>) -> Vec<T> {
    vec.insert(0, T::ZERO); // ensure vec.len() > 0
    fenwicktree::build(&mut vec);
    vec
}

#[quickcheck]
fn build_reset(vec: Vec<u32>) -> bool {
    let mut tr = build(vec.clone());
    fenwicktree::reset(&mut tr);
    tr[1..] == vec
}

#[quickcheck]
fn tree_by_incr(vec: Vec<u64>) -> bool {
    let mut tr = vec![0; vec.len() + 1];
    for (i, &d) in vec.iter().enumerate() {
        tr.incr(i + 1, d);
    }

    tr[0] == 0 && tr == build(vec)
}

#[quickcheck]
fn sum_0_is_always_zero(vec: Vec<u64>) -> bool {
    let tr = build(vec);
    let sum: u64 = tr.sum(0);
    sum == 0
}

#[quickcheck]
fn sum_x_eq_vec_sum(vec: Vec<u64>) -> bool {
    let tr = build(vec.clone());
    (0..=tr.nodes()).all(|i| {
        let sum: u64 = tr.sum(i);
        sum == vec[..i].iter().sum()
    })
}

// It takes too long to complete the test when using `Vec<u64>`.
#[quickcheck]
fn lower_bound_sum(vec: Vec<u16>) -> bool {
    let tr = build(vec.clone());
    (0..=vec.iter().sum::<u16>()).map(Into::into).all(|w| {
        let i = tr.lower_bound(w);
        let sum: u64 = fenwicktree::prefix(i).map(|i| Into::<u64>::into(tr[i])).sum();
        sum >= w.into()
    })
}

#[quickcheck]
fn pop_all_then_push_all(vec: Vec<u64>) -> bool {
    let tr = build(vec.clone());

    let mut cloned = tr.clone();
    let mut popped = iter::from_fn(|| fenwicktree::pop(&mut cloned)).collect::<Vec<_>>();

    popped.reverse();
    assert_eq!(vec, popped);

    for x in popped.into_iter() {
        fenwicktree::push(&mut cloned, x);
    }

    tr == cloned
}
