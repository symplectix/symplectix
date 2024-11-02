use fenwicktree::{LowerBound, Nodes, Prefix};

#[test]
fn lower_bound() {
    {
        let mut tr: Vec<u32> = vec![0, 1, 0, 3, 5];
        fenwicktree::build(&mut tr);

        assert_eq!(4, tr.nodes());

        assert_eq!(0u32, tr.sum(0));
        assert_eq!(1u32, tr.sum(1));
        assert_eq!(1u32, tr.sum(2));
        assert_eq!(4u32, tr.sum(3));
        assert_eq!(9u32, tr.sum(4));

        assert_eq!(tr.lower_bound(1), 1);
        assert_eq!(tr.lower_bound(3), 3);
        assert_eq!(tr.lower_bound(4), 3);
        assert_eq!(tr.lower_bound(5), 4);
    }

    {
        let mut tr: Vec<u32> = vec![0, 0, 1, 0, 0, 3, 0, 2, 4, 2];
        fenwicktree::build(&mut tr);

        assert_eq!(9, tr.nodes());

        assert_eq!(0u32, tr.sum(0));
        assert_eq!(0u32, tr.sum(1));
        assert_eq!(1u32, tr.sum(2));
        assert_eq!(1u32, tr.sum(3));
        assert_eq!(1u32, tr.sum(4));

        assert_eq!(tr.lower_bound(1), 2);
        assert_eq!(tr.lower_bound(4), 5);
        assert_eq!(tr.lower_bound(5), 7);
        assert_eq!(tr.lower_bound(10), 8);
        assert_eq!(tr.lower_bound(11), 9);
        assert_eq!(tr.lower_bound(12), 9);
    }
}
