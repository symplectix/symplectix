#[test]
fn next_index_for_update() {
    let indices = [
        0b_0000_0110_1110_1010_1101_0001, // 453329
        0b_0000_0110_1110_1010_1101_0010, // 453330
        0b_0000_0110_1110_1010_1101_0100, // 453332
        0b_0000_0110_1110_1010_1101_1000, // 453336
        0b_0000_0110_1110_1010_1110_0000, // 453344
        0b_0000_0110_1110_1011_0000_0000, // 453376
        0b_0000_0110_1110_1100_0000_0000, // 453632
        0b_0000_0110_1111_0000_0000_0000, // 454656
        0b_0000_0111_0000_0000_0000_0000, // 458752
        0b_0000_1000_0000_0000_0000_0000, // 524288
        0b_0001_0000_0000_0000_0000_0000, // 1048576
        0b_0010_0000_0000_0000_0000_0000, // 2097152
    ];

    assert_eq!(
        fenwicktree::update(indices[0], indices[indices.len() - 1] + 1).collect::<Vec<_>>(),
        &indices[0..]
    );
}

#[test]
fn update() {
    let mut indices = fenwicktree::update(0, 8);
    assert_eq!(indices.next(), None);

    let mut indices = fenwicktree::update(1, 8);
    assert_eq!(indices.next(), Some(1));
    assert_eq!(indices.next(), Some(2));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);

    let mut indices = fenwicktree::update(3, 8);
    assert_eq!(indices.next(), Some(3));
    assert_eq!(indices.next(), Some(4));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);

    let mut indices = fenwicktree::update(7, 8);
    assert_eq!(indices.next(), Some(7));
    assert_eq!(indices.next(), Some(8));
    assert_eq!(indices.next(), None);
}
