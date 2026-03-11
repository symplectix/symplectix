use quickcheck_macros::quickcheck;

use crate::rrr04;

fn rrr04_check(mut bits: u8, class: u8, offset: u8) {
    bits &= (1 << 4) - 1;
    let (c, o) = rrr04::encode(bits);
    assert_eq!(c, class, "class does not match");
    assert_eq!(o, offset, "offset does not match");
    let decoded = rrr04::decode(c, o);
    assert_eq!(bits, decoded, "decoded does not match");
}

#[test]
fn rrr04_class_0() {
    rrr04_check(0b_0000, 0, 0);
}

#[test]
fn rrr04_class_1() {
    rrr04_check(0b_0001, 1, 0);
    rrr04_check(0b_0010, 1, 1);
    rrr04_check(0b_0100, 1, 2);
    rrr04_check(0b_1000, 1, 3);
}

#[test]
fn rrr04_class_2() {
    rrr04_check(0b_0011, 2, 0);
    rrr04_check(0b_0101, 2, 1);
    rrr04_check(0b_0110, 2, 2);
    rrr04_check(0b_1001, 2, 3);
    rrr04_check(0b_1010, 2, 4);
    rrr04_check(0b_1100, 2, 5);
}

#[test]
fn rrr04_class_3() {
    rrr04_check(0b_0111, 3, 0);
    rrr04_check(0b_1011, 3, 1);
    rrr04_check(0b_1101, 3, 2);
    rrr04_check(0b_1110, 3, 3);
}

#[test]
fn rrr04_class_4() {
    rrr04_check(0b_1111, 4, 0);
}

#[test]
fn class_values() {
    assert_eq!(rrr04::CLASS_SIZE, 3);
}

#[quickcheck]
fn encode_decode(b: u8) -> bool {
    let b = b & ((1 << 4) - 1);
    let (c, o) = rrr04::encode(b);
    let decoded = rrr04::decode(c, o);
    b == decoded
}
