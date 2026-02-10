#![allow(missing_docs)]

use quickcheck::quickcheck;

fn rrr15_check(mut bits: u16, class: u8, offset: u16) {
    bits &= (1 << 15) - 1;
    let (c, o) = rrr15::encode(bits);
    assert_eq!(c, class, "class does not match");
    assert_eq!(o, offset, "offset does not match");
    let decoded = rrr15::decode(c, o);
    assert_eq!(bits, decoded, "decoded does not match");
}

#[test]
fn rrr15_class_0() {
    rrr15_check(0b_0000_0000_0000_0000, 0, 0);
    rrr15_check(0b_1000_0000_0000_0000, 0, 0);
}

#[test]
fn rrr15_class_1() {
    rrr15_check(0b_0000_0000_0000_0001, 1, 0);
    rrr15_check(0b_0000_0000_0000_0010, 1, 1);
    rrr15_check(0b_0100_0000_0000_0000, 1, 14);
}

#[test]
fn rrr15_class_2() {
    rrr15_check(0b_0000_0000_0000_0011, 2, 0);
    // comb(15, 2) is 105
    rrr15_check(0b_0110_0000_0000_0000, 2, 104);
}

#[test]
fn rrr15_class_15() {
    rrr15_check(0b_0111_1111_1111_1111, 15, 0);
}

#[test]
fn class_values() {
    assert_eq!(rrr15::CLASS_SIZE, 4);
}

quickcheck! {
    fn rrr15(b: u16) -> bool {
        let b = b & ((1 << 15) - 1);
        let (c, o) = rrr15::encode(b);
        let decoded = rrr15::decode(c, o);
        b == decoded
    }
}
