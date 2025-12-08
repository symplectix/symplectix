#![allow(missing_docs)]

use xf::{Adapter, Compose, Reducer};

pub struct Conj;

impl xf::Reducer<Vec<i32>, i32> for Conj {
    // type Acc = i32;

    fn step(&mut self, mut acc: Vec<i32>, v: i32) -> xf::Step<Vec<i32>> {
        acc.push(v);
        xf::Step::Continue(acc)
    }

    fn done(&mut self, acc: Vec<i32>) -> Vec<i32> {
        acc
    }
}

#[test]
fn test_map() {
    let mut rf = xf::map(|x| x + 1).map(|x| x + 1).map(|x| x % 2).apply(Conj);
    let mut acc = Vec::with_capacity(10);
    for i in 0..5 {
        match rf.step(acc, i) {
            xf::Step::Continue(ret) => {
                acc = ret;
            }
            xf::Step::Reduced(ret) => {
                acc = rf.done(ret);
                break;
            }
        }
    }
    assert_eq!(vec![0, 1, 0, 1, 0], acc);
}
