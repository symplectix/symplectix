#![allow(missing_docs)]

use quickcheck::quickcheck;

#[test]
fn rrr15_encode_decode() {
    let b = 0b_0000_0000_0000_0000;
    let (c, o) = rrr15::encode(b);
    assert_eq!(c, 0);
    assert_eq!(o, 0);
    let decoded = rrr15::decode(c, o);
    assert_eq!(b, decoded);

    let b = 0b_1000_0000_0000_0000;
    let (c, o) = rrr15::encode(b);
    assert_eq!(c, 0);
    assert_eq!(o, 0);
    let decoded = rrr15::decode(c, o);
    assert_eq!(0, decoded);

    let b = 0b_0100_0000_0000_0000;
    let (c, o) = rrr15::encode(b);
    assert_eq!(c, 1);
    assert_eq!(o, 14);
    let decoded = rrr15::decode(c, o);
    assert_eq!(b, decoded);

    let b = 0b_0000_0000_0000_0010;
    let (c, o) = rrr15::encode(b);
    assert_eq!(c, 1);
    assert_eq!(o, 1);
    let decoded = rrr15::decode(c, o);
    assert_eq!(b, decoded);

    let b = 0b_0000_0000_0000_0011;
    let (c, o) = rrr15::encode(b);
    assert_eq!(c, 2);
    assert_eq!(o, 0);
    let decoded = rrr15::decode(c, o);
    assert_eq!(b, decoded);

    let b = 0b_0111_1111_1111_1111;
    let (c, o) = rrr15::encode(b);
    assert_eq!(c, 15);
    assert_eq!(o, 0);
    let decoded = rrr15::decode(c, o);
    assert_eq!(b, decoded);
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
