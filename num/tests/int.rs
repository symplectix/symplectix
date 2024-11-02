#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use core::iter::successors;
use core::ops::Add;

use bits::Bits;
use num::*;

// use core::iter::Step;
// fn range<T: Int + Step>(init: T, max: T) -> impl Iterator<Item = T> {
//     init..max
// }

fn range<T: Int + Add<Output = T>>(start: T, end: T) -> impl Iterator<Item = T> {
    successors(Some(start), move |&x| (x < end).then_some(x + T::ONE))
}

#[test]
fn range_test() {
    for (x, y) in range(0, 5).zip([0, 1, 2, 3, 4]) {
        assert_eq!(x, y);
    }
}

#[quickcheck]
fn prop_lsb(u: u32) -> bool {
    let i = u as i32;
    u.lsb() == (i & -i) as u32
}

#[quickcheck]
fn next_set_bit(n: u32) -> bool {
    let mut set_bit = successors(Some(n), |&n| {
        let m = n & !n.lsb();
        m.any().then_some(m)
    })
    .map(|x| u32::trailing_zeros(x) as usize);

    for c in 0..n.count1() {
        assert_eq!(set_bit.next(), n.select1(c));
    }

    true
}

#[test]
fn lsb() {
    let tests = [
        (0b_0000_0000_u8, 0b0000_0000),
        (0b_0000_0001_u8, 0b0000_0001),
        (0b_0000_1100_u8, 0b0000_0100),
        (0b_1001_0100_u8, 0b0000_0100),
        (0b_1001_0000_u8, 0b0001_0000),
    ];

    for (n, want) in tests {
        assert_eq!(n.lsb(), want);
        assert_eq!((n as i8).lsb(), want as i8);
    }
}

#[test]
fn msb() {
    let tests = [
        (0b_0000_0000_u8, 0b_0000_0000_u8),
        (0b_0000_0001_u8, 0b_0000_0001_u8),
        (0b_0000_0011_u8, 0b_0000_0010_u8),
        (0b_0000_1100_u8, 0b_0000_1000_u8),
        (0b_1001_0100_u8, 0b_1000_0000_u8),
        (0b_1001_0000_u8, 0b_1000_0000_u8),
    ];

    for (n, want) in tests {
        assert_eq!(n.msb(), want);
        assert_eq!((n as i8).msb(), want as i8);
    }
}

#[test]
fn uint_msb() {
    assert_eq!(0u8.msb(), 0);
    assert_eq!(1u8.msb(), 1);
    assert_eq!(2u8.msb(), 2);
    assert_eq!(3u8.msb(), 2);
    assert_eq!(4u8.msb(), 4);
    assert_eq!(5u8.msb(), 4);
    assert_eq!(6u8.msb(), 4);
    assert_eq!(7u8.msb(), 4);
    assert_eq!(8u8.msb(), 8);
    assert_eq!(9u8.msb(), 8);
    assert_eq!(10u8.msb(), 8);
    assert_eq!(15u8.msb(), 8);
    assert_eq!(16u8.msb(), 16);
    assert_eq!(18u8.msb(), 16);
    assert_eq!(30u8.msb(), 16);
    assert_eq!(33u8.msb(), 32);
}

#[test]
fn sint_msb() {
    assert_eq!((-1i8).msb(), -128);

    assert_eq!(0i8.msb(), 0);
    assert_eq!(1i8.msb(), 1);
    assert_eq!(2i8.msb(), 2);
    assert_eq!(3i8.msb(), 2);
    assert_eq!(4i8.msb(), 4);
    assert_eq!(5i8.msb(), 4);
    assert_eq!(6i8.msb(), 4);
    assert_eq!(7i8.msb(), 4);
    assert_eq!(8i8.msb(), 8);
    assert_eq!(9i8.msb(), 8);
    assert_eq!(10i8.msb(), 8);
    assert_eq!(15i8.msb(), 8);
    assert_eq!(16i8.msb(), 16);
    assert_eq!(18i8.msb(), 16);
    assert_eq!(30i8.msb(), 16);
    assert_eq!(33i8.msb(), 32);
}
