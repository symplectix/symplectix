#![allow(missing_docs)]

use xf::{Adapter, Compose, Reducer};

struct PushVec;

impl<T> xf::Reducer<T> for PushVec {
    type Acc = Vec<T>;

    fn step(&mut self, mut acc: Self::Acc, v: T) -> xf::Step<Self::Acc> {
        acc.push(v);
        xf::Step::Continue(acc)
    }

    fn done(&mut self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}

#[test]
fn test_map_filter() {
    let mut rf = xf::map(|x| x * 2).map(|x| x + 1).filter(|x: &i32| x % 3 != 0).apply(PushVec);
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
    assert_eq!(vec![1, 2, 1], acc);
}
