#[test]
fn next_index_for_prefix() {
    let indices = [
        0b_0110_1110_1010_1101_0000, // 453328
        0b_0110_1110_1010_1100_0000, // 453312
        0b_0110_1110_1010_1000_0000, // 453248
        0b_0110_1110_1010_0000_0000, // 453120
        0b_0110_1110_1000_0000_0000, // 452608
        0b_0110_1110_0000_0000_0000, // 450560
        0b_0110_1100_0000_0000_0000, // 442368
        0b_0110_1000_0000_0000_0000, // 425984
        0b_0110_0000_0000_0000_0000, // 393216
        0b_0100_0000_0000_0000_0000, // 262144
    ];

    assert_eq!(fenwicktree::prefix(indices[0]).collect::<Vec<_>>(), &indices[0..]);
}

#[test]
fn prefix() {
    let mut indices = fenwicktree::prefix(0);
    assert_eq!(indices.next(), None);

    let mut indices = fenwicktree::prefix(3);
    assert_eq!(indices.next(), Some(3));
    assert_eq!(indices.next(), Some(2));
    assert_eq!(indices.next(), None);

    let mut indices = fenwicktree::prefix(7);
    assert_eq!(indices.next(), Some(7));
    assert_eq!(indices.next(), Some(6));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), None);

    let mut indices = fenwicktree::prefix(8);
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);
}
