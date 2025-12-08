#![allow(missing_docs)]

use xfn::{Adapter, Reducer};

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

fn inc(x: i32) -> i32 {
    x + 1
}

fn dec(x: i32) -> i32 {
    x - 1
}

// fn adapter<T>() -> impl Adapter<T> {
//     mung::adapter().map(incr).filter(mod2)
// }

#[test]
fn test_map() {
    let add = Add;
    let mut map = xfn::map(inc).apply(add);
    // let mut map = xfn::map(inc).map(dec).apply(add);
    // let mut map = <xfn::Map<_> as xfn::Adapter<Add>>::map::<_>(xfn::map(inc), inc).apply(add);
    println!("{:?}", map.step(0, 1));
    assert_eq!(1, 0);
}
