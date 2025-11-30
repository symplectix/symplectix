#![allow(missing_docs)]

use mung::Adapter;

fn incr(x: i32) -> i32 {
    x + 1
}

fn mod2(x: &i32) -> bool {
    *x % 2 == 0
}

#[test]
fn test_iter() {
    let map = mung::Map::new(incr);
    let filter = mung::Filter::new(mod2);
    let adapter = <mung::Map<_> as Adapter<std::vec::IntoIter<i32>>>::compose(map, filter);
    let vec: Vec<i32> = vec![0, 1, 2];
    let iter = adapter.apply(vec.into_iter());
    println!("{:?}", iter.collect::<Vec<i32>>());
    assert!(1 == 0)
}
