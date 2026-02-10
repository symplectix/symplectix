#![allow(missing_docs)]

use quickcheck::quickcheck;

#[test]
fn class_values() {
    assert_eq!(rrr31::CLASS_SIZE, 5);
}

quickcheck! {
    fn rrr31(b: u32) -> bool {
        let b = b & ((1 << 31) - 1);
        let (c, o) = rrr31::encode(b);
        let decoded = rrr31::decode(c, o);
        b == decoded
    }
}
