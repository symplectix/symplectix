#![allow(missing_docs)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use bits::block::*;
use bits::word::Word;

fn lsb<T: Select>(bs: &T) -> Option<usize> {
    bs.select1(0)
}

fn msb<T: Select>(bs: &T) -> Option<usize> {
    bs.any().then(|| bs.select1(bs.count1() - 1).unwrap())
}

#[quickcheck]
fn lsb_select1(n: u32) -> bool {
    n.lsb().select1(0) == lsb(&n)
}

#[quickcheck]
fn msb_select1(n: u32) -> bool {
    n.msb().select1(0) == msb(&n)
}

#[test]
fn msb_work() {
    assert_eq!(0b_0000_0000_u8.msb(), 0b_0000_0000_u8);
    assert_eq!(0b_0101_1101_u8.msb(), 0b_0100_0000_u8);
}
