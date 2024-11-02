#[test]
fn a_and_b() {
    use bits::And;
    let v1: &[u8] = &[0b_1111_0000, 0b_0000_1111, 0b_1010_1010];
    let v2: &[u8] = &[0b_0011_1100, 0b_0011_1100, 0b_0101_0101];
    let v3: &[u8] = &[0b_1100_0011, 0b_1100_0011, 0b_1010_1010];
    for (_index, bits) in v1.and(v2).and(v3) {
        assert_eq!(bits.into_owned(), 0b_0000_0000);
    }
    for (_index, bits) in v1.and(v2.and(v3)) {
        assert_eq!(bits.into_owned(), 0b_0000_0000);
    }
}

#[test]
fn a_not_b() {
    use bits::Not;
    let v1: &[u8] = &[0b_1111_1111, 0b_1111_1111, 0b_1111_1111];
    let v2: &[u8] = &[0b_0000_1111, 0b_1111_0000, 0b_0101_0101];
    for (index, bits) in v1.not(v2) {
        assert_eq!(bits.into_owned(), !v2[index]);
    }
}

#[test]
fn a_or_b() {
    use bits::Or;
    let v1: &[u8] = &[0b_1111_0000, 0b_0000_1111, 0b_1010_1010];
    let v2: &[u8] = &[0b_0000_1111, 0b_1111_0000, 0b_0101_0101];
    for (_index, bits) in v1.or(v2) {
        assert_eq!(bits.into_owned(), 0b_1111_1111);
    }
}

#[test]
fn a_xor_b() {
    use bits::Xor;
    let v1: &[u8] = &[0b_1111_0000, 0b_0000_1111, 0b_1010_1010];
    let v2: &[u8] = &[0b_0011_0011, 0b_1100_1100, 0b_0110_1001];
    for (_index, bits) in v1.xor(v2) {
        assert_eq!(bits.into_owned(), 0b_1100_0011);
    }
}
