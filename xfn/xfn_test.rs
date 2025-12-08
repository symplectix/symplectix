#![allow(missing_docs)]

use xfn::{Adapter, Chain, Reducer};

pub struct Add;

impl xfn::Reducer<i32, i32> for Add {
    // type Acc = i32;

    fn step(&mut self, acc: i32, v: i32) -> xfn::Step<i32> {
        xfn::Step::Next(acc + v)
    }

    fn done(&mut self, acc: i32) -> i32 {
        acc
    }
}

pub struct VecConj;

impl xfn::Reducer<Vec<i32>, i32> for VecConj {
    // type Acc = i32;

    fn step(&mut self, mut acc: Vec<i32>, v: i32) -> xfn::Step<Vec<i32>> {
        acc.push(v);
        xfn::Step::Next(acc)
    }

    fn done(&mut self, acc: Vec<i32>) -> Vec<i32> {
        acc
    }
}

fn inc(x: i32) -> i32 {
    x + 1
}

fn mod2(x: i32) -> i32 {
    x % 2
}

// fn adapter<T>() -> impl Adapter<T> {
//     mung::adapter().map(incr).filter(mod2)
// }

#[test]
fn test_map() {
    let conj = VecConj;

    let mut rf = xfn::map(inc).map(mod2).apply(conj);
    let mut acc = Vec::with_capacity(10);
    for i in 0..10 {
        match rf.step(acc, i) {
            xfn::Step::Next(ret) => {
                acc = ret;
            }
            xfn::Step::Done(ret) => {
                acc = ret;
                break;
            }
        }
    }
    println!("{:?}", rf.done(acc));

    assert_eq!(1, 0);
}
