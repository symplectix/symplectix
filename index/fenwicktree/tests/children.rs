#[test]
fn children() {
    let indices = fenwicktree::children(1);
    assert_eq!(indices.collect::<Vec<usize>>(), []);

    let indices = fenwicktree::children(2);
    assert_eq!(indices.collect::<Vec<usize>>(), [1]);

    let indices = fenwicktree::children(4);
    assert_eq!(indices.collect::<Vec<usize>>(), [3, 2]);

    let indices = fenwicktree::children(7);
    assert_eq!(indices.collect::<Vec<usize>>(), []);

    let indices = fenwicktree::children(8);
    assert_eq!(indices.collect::<Vec<usize>>(), [7, 6, 4]);
}
