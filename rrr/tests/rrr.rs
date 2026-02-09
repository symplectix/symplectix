#![allow(missing_docs)]

use quickcheck::quickcheck;

fn rrr4_check(mut bits: u8, class: u32, offset: u8) {
    bits &= (1 << 4) - 1;
    let (c, o) = rrr4::encode(bits);
    assert_eq!(c, class);
    assert_eq!(o, offset);
    let decoded = rrr4::decode(c, o);
    assert_eq!(bits, decoded);
}

#[test]
fn rrr4_encode_decode() {
    rrr4_check(0b_0000, 0, 0);

    rrr4_check(0b_0001, 1, 0);
    rrr4_check(0b_0010, 1, 1);
    rrr4_check(0b_0100, 1, 2);
    rrr4_check(0b_1000, 1, 3);

    rrr4_check(0b_0011, 2, 0);
    rrr4_check(0b_0101, 2, 1);
    rrr4_check(0b_0110, 2, 2);
    rrr4_check(0b_1001, 2, 3);
    rrr4_check(0b_1010, 2, 4);
    rrr4_check(0b_1100, 2, 5);
}

fn rrr15_check(mut bits: u16, class: u32, offset: u16) {
    bits &= (1 << 15) - 1;
    let (c, o) = rrr15::encode(bits);
    assert_eq!(c, class, "class does not match");
    assert_eq!(o, offset, "offset does not match");
    let decoded = rrr15::decode(c, o);
    assert_eq!(bits, decoded, "decoded does not match");
}

#[test]
fn rrr15_encode_decode() {
    rrr15_check(0b_0000_0000_0000_0000, 0, 0);
    rrr15_check(0b_1000_0000_0000_0000, 0, 0);

    rrr15_check(0b_0000_0000_0000_0001, 1, 0);
    rrr15_check(0b_0000_0000_0000_0010, 1, 1);
    rrr15_check(0b_0100_0000_0000_0000, 1, 14);

    rrr15_check(0b_0000_0000_0000_0011, 2, 0);

    rrr15_check(0b_0111_1111_1111_1111, 15, 0);
}

#[test]
fn class_values() {
    assert_eq!(rrr15::CLASS_SIZE, 4);
    assert_eq!(rrr31::CLASS_SIZE, 5);
    assert_eq!(rrr63::CLASS_SIZE, 6);
}

quickcheck! {
    fn rrr15(b: u16) -> bool {
        let b = b & ((1 << 15) - 1);
        let (c, o) = rrr15::encode(b);
        let decoded = rrr15::decode(c, o);
        b == decoded
    }

    fn rrr31(b: u32) -> bool {
        let b = b & ((1 << 31) - 1);
        let (c, o) = rrr31::encode(b);
        let decoded = rrr31::decode(c, o);
        b == decoded
    }

    fn rrr63(b: u64) -> bool {
        let b = b & ((1 << 63) - 1);
        let (c, o) = rrr63::encode(b);
        let decoded = rrr63::decode(c, o);
        b == decoded
    }
}
