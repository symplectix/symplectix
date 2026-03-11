use quickcheck_macros::quickcheck;

use crate::rrr63;

#[test]
fn class_values() {
    assert_eq!(rrr63::CLASS_SIZE, 6);
}

#[quickcheck]
fn encode_decode(b: u64) -> bool {
    let b = b & ((1 << 63) - 1);
    let (c, o) = rrr63::encode(b);
    let decoded = rrr63::decode(c, o);
    b == decoded
}
