#![allow(missing_docs)]

use cx::{Adapter, Chain, Fold};

struct PushVec;

impl<T> Fold<T> for PushVec {
    type Acc = Vec<T>;

    fn step(&mut self, mut acc: Self::Acc, v: T) -> cx::Step<Self::Acc> {
        acc.push(v);
        cx::Step::Yield(acc)
    }

    fn done(self, acc: Self::Acc) -> Self::Acc {
        acc
    }
}

#[test]
fn test_map_filter() {
    let mut rf = cx::filter(|x: &i32| *x > 5).map(|x| x * 2).map(|x| x + 1).filter(|x: &i32| *x < 20).apply(PushVec);
    let mut acc = vec![0];
    for i in 0..20 {
        match rf.step(acc, i) {
            cx::Step::Yield(ret) => {
                acc = ret;
            }
            cx::Step::Return(ret) => {
                acc = rf.done(ret);
                break;
            }
        }
    }
    assert_eq!(acc, vec![0, 13, 15, 17, 19]);
}
