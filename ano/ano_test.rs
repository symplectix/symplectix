#![allow(missing_docs)]

// use ano::{StepFn, Xform};

// struct PushVec;

// impl<T> StepFn<T> for PushVec {
//     type Acc = Vec<T>;

//     fn step(&mut self, mut acc: Self::Acc, v: T) -> ano::Step<Self::Acc> {
//         acc.push(v);
//         ano::Step::Yield(acc)
//     }
// }

// #[test]
// fn test_map_filter_step() {
//     let mut acc = vec![];
//     let mut sf = ano::map(|x| x * 2 + 1).filter(|x: &i32| 10 < *x && *x < 20).apply(PushVec);
//     for i in 0..20 {
//         match sf.step(acc, i) {
//             ano::Step::Yield(ret) => {
//                 acc = ret;
//             }
//             ano::Step::Break(ret) => {
//                 acc = sf.done(ret);
//                 break;
//             }
//         }
//     }
//     assert_eq!(acc, vec![11, 13, 15, 17, 19]);
// }
