#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use fenwicktree as fw;
use fenwicktree::{Incr, LowerBound, Nodes, Prefix};
use num::Int;
use std::{iter, ops};

#[test]
fn next_index_for_prefix() {
    let indices = vec![
        0b_0110_1110_1010_1101_0000, // 453328
        0b_0110_1110_1010_1100_0000, // 453312
        0b_0110_1110_1010_1000_0000, // 453248
        0b_0110_1110_1010_0000_0000, // 453120
        0b_0110_1110_1000_0000_0000, // 452608
        0b_0110_1110_0000_0000_0000, // 450560
        0b_0110_1100_0000_0000_0000, // 442368
        0b_0110_1000_0000_0000_0000, // 425984
        0b_0110_0000_0000_0000_0000, // 393216
        0b_0100_0000_0000_0000_0000, // 262144
    ];

    assert_eq!(fw::prefix(indices[0]).collect::<Vec<_>>(), &indices[0..]);
}

#[test]
fn next_index_for_update() {
    let indices = vec![
        0b_0000_0110_1110_1010_1101_0001, // 453329
        0b_0000_0110_1110_1010_1101_0010, // 453330
        0b_0000_0110_1110_1010_1101_0100, // 453332
        0b_0000_0110_1110_1010_1101_1000, // 453336
        0b_0000_0110_1110_1010_1110_0000, // 453344
        0b_0000_0110_1110_1011_0000_0000, // 453376
        0b_0000_0110_1110_1100_0000_0000, // 453632
        0b_0000_0110_1111_0000_0000_0000, // 454656
        0b_0000_0111_0000_0000_0000_0000, // 458752
        0b_0000_1000_0000_0000_0000_0000, // 524288
        0b_0001_0000_0000_0000_0000_0000, // 1048576
        0b_0010_0000_0000_0000_0000_0000, // 2097152
    ];

    assert_eq!(
        fw::update(indices[0], indices[indices.len() - 1] + 1).collect::<Vec<_>>(),
        &indices[0..]
    );
}

#[test]
fn prefix() {
    let mut indices = fw::prefix(0);
    assert_eq!(indices.next(), None);

    let mut indices = fw::prefix(3);
    assert_eq!(indices.next(), Some(3));
    assert_eq!(indices.next(), Some(2));
    assert_eq!(indices.next(), None);

    let mut indices = fw::prefix(7);
    assert_eq!(indices.next(), Some(7));
    assert_eq!(indices.next(), Some(6));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), None);

    let mut indices = fw::prefix(8);
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);
}

#[test]
fn update() {
    let mut indices = fw::update(0, 8);
    assert_eq!(indices.next(), None);

    let mut indices = fw::update(1, 8);
    assert_eq!(indices.next(), Some(1));
    assert_eq!(indices.next(), Some(2));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);

    let mut indices = fw::update(3, 8);
    assert_eq!(indices.next(), Some(3));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);

    let mut indices = fw::update(7, 8);
    assert_eq!(indices.next(), Some(7));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);
}

#[test]
fn children() {
    let indices = fw::children(1);
    assert_eq!(indices.collect::<Vec<usize>>(), []);

    let indices = fw::children(2);
    assert_eq!(indices.collect::<Vec<usize>>(), [1]);

    let indices = fw::children(4);
    assert_eq!(indices.collect::<Vec<usize>>(), [3, 2]);

    let indices = fw::children(7);
    assert_eq!(indices.collect::<Vec<usize>>(), []);

    let indices = fw::children(8);
    assert_eq!(indices.collect::<Vec<usize>>(), [7, 6, 4]);
}

fn build<T: Int + ops::AddAssign>(mut vec: Vec<T>, zero: T) -> Vec<T> {
    vec.insert(0, zero); // ensure vec.len() > 0
    fenwicktree::build(&mut vec);
    vec
}

#[test]
fn lower_bound() {
    {
        let bit: &mut [u32] = &mut [0, 1, 0, 3, 5];
        fenwicktree::build(bit);

        assert_eq!(4, bit.nodes());

        assert_eq!(0, bit.sum::<u32>(0));
        assert_eq!(1, bit.sum::<u32>(1));
        assert_eq!(1, bit.sum::<u32>(2));
        assert_eq!(4, bit.sum::<u32>(3));
        assert_eq!(9, bit.sum::<u32>(4));

        // assert_eq!(1, bit.range_sum::<u32, _>(1..3));
        // assert_eq!(4, bit.range_sum::<u32, _>(1..4));
        // assert_eq!(9, bit.range_sum::<u32, _>(1..=4));
        // assert_eq!(9, bit.range_sum::<u32, _>(1..5));
        // assert_eq!(9, bit.range_sum::<u32, _>(..));
        // assert_eq!(9, bit.range_sum::<u32, _>(0..));
        // assert_eq!(9, bit.range_sum::<u32, _>(1..));
        // assert_eq!(0, bit.range_sum::<u32, _>(5..));

        // assert_eq!(bit.lower_bound(0), 0);
        assert_eq!(bit.lower_bound(1), 1);
        assert_eq!(bit.lower_bound(3), 3);
        assert_eq!(bit.lower_bound(4), 3);
        assert_eq!(bit.lower_bound(5), 4);
    }

    {
        let bit: &mut [u32] = &mut [0, 0, 1, 0, 0, 3, 0, 2, 4, 2];
        fenwicktree::build(bit);

        assert_eq!(9, bit.nodes());

        // assert_eq!(bit.lower_bound(0), 0);
        assert_eq!(bit.lower_bound(1), 2);
        assert_eq!(bit.lower_bound(4), 5);
        assert_eq!(bit.lower_bound(5), 7);
        assert_eq!(bit.lower_bound(10), 8);
        assert_eq!(bit.lower_bound(11), 9);
        assert_eq!(bit.lower_bound(12), 9);
    }
}

#[quickcheck]
fn build_unbuild(vec: Vec<u32>) -> bool {
    let mut bit = build(vec.clone(), 0);
    fenwicktree::unbuild(&mut bit);
    bit[1..] == vec
}

#[quickcheck]
// fn tree_by_incr(vec: Vec<num::Wrapping<u64>>) -> bool {
fn tree_by_incr(vec: Vec<u64>) -> bool {
    let mut bit = vec![0; vec.len() + 1];
    for (i, &d) in vec.iter().enumerate() {
        bit.incr(i + 1, d);
    }

    bit[0] == 0 && bit == build(vec, 0)
}

#[quickcheck]
fn sum_0_is_always_zero(vec: Vec<u64>) -> bool {
    let bit = build(vec, 0);
    bit.sum::<u64>(0) == 0
}

#[quickcheck]
fn sum_x_eq_vec_sum(vec: Vec<u64>) -> bool {
    let bit = build(vec.clone(), 0);
    (0..=bit.nodes()).all(|i| bit.sum::<u64>(i) == vec[..i].iter().sum())
}

// It takes too long to complete the test when using `Vec<u64>`.
#[quickcheck]
fn lower_bound_sum(vec: Vec<u16>) -> bool {
    let bit = build(vec.clone(), 0);
    (0..=vec.iter().sum::<u16>()).map(Into::into).all(|w| {
        let i = bit.lower_bound(w);
        bit.prefix(i).map(Into::<u64>::into).sum::<u64>() >= w.into()
    })
}

#[quickcheck]
fn pop_all_then_push_all(vec: Vec<u64>) -> bool {
    let bit = build(vec.clone(), 0);

    let mut cloned = bit.clone();
    let mut popped = iter::from_fn(|| fenwicktree::pop(&mut cloned)).collect::<Vec<_>>();

    popped.reverse();
    assert_eq!(vec, popped);

    for x in popped.into_iter() {
        fenwicktree::push(&mut cloned, x);
    }

    bit == cloned
}
