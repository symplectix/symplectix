#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use bits::{Bits, Word};

fn lsb<T: Bits>(bs: &T) -> Option<usize> {
    bs.select1(0)
}

fn msb<T: Bits>(bs: &T) -> Option<usize> {
    Bits::any(bs).then(|| bs.select1(bs.count1() - 1).unwrap())
}

#[quickcheck]
fn lsb_select1(n: u32) -> bool {
    n.lsb().select1(0) == lsb(&n)
}

#[quickcheck]
fn msb_select1(n: u32) -> bool {
    n.msb().select1(0) == msb(&n)
}
