#![allow(missing_docs)]

use mung::{Chain, StepFn, Xform};

struct PushVec;

impl<T> StepFn<T> for PushVec {
    type Acc = Vec<T>;

    fn step(&mut self, mut acc: Self::Acc, v: T) -> mung::Step<Self::Acc> {
        acc.push(v);
        mung::Step::Yield(acc)
    }
}

#[test]
fn test_map_filter_step() {
    let mut acc = vec![];
    let mut sf = mung::map(|x| x * 2 + 1).filter(|x: &i32| 10 < *x && *x < 20).apply(PushVec);
    for i in 0..20 {
        match sf.step(acc, i) {
            mung::Step::Yield(ret) => {
                acc = ret;
            }
            mung::Step::Break(ret) => {
                acc = sf.done(ret);
                break;
            }
        }
    }
    assert_eq!(acc, vec![11, 13, 15, 17, 19]);
}
