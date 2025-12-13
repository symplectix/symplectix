#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

use std::borrow::Borrow;
use std::marker::PhantomData;

// xforms
mod fold;
mod xf;

// foldings
// mod either;

#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, ToOwned};
    use std::collections::VecDeque;

    use crate::fold::Fold;
    use crate::{fold, xf};

    struct Conj;

    impl<T> fold::Fold<T> for Conj
    where
        T: ToOwned,
    {
        type Acc = Vec<T::Owned>;

        fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> fold::Step<Self::Acc>
        where
            Q: Borrow<T>,
        {
            acc.push(input.borrow().to_owned());
            fold::Step::Yield(acc)
        }
    }

    struct Cons;

    impl<T> fold::Fold<T> for Cons
    where
        T: ToOwned,
    {
        type Acc = VecDeque<T::Owned>;

        fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> fold::Step<Self::Acc>
        where
            Q: Borrow<T>,
        {
            acc.push_front(input.borrow().to_owned());
            fold::Step::Yield(acc)
        }
    }

    #[test]
    fn test_map_filter_step() {
        let mut f = xf::id::<i32>()
            .map(|x: &i32| x + 1)
            .filter(|x: &i32| *x % 2 == 0)
            .apply(Cons)
            .either(xf::id::<i32>().map(|x: &i32| x - 1).filter(|x: &i32| *x % 2 != 0).apply(Conj));
        let mut acc = (VecDeque::with_capacity(10), vec![]);
        for i in 0..10 {
            match f.step(acc, &i) {
                fold::Step::Yield(ret) => {
                    acc = ret;
                }
                fold::Step::Break(ret) => {
                    acc = f.done(ret);
                    break;
                }
            }
        }
        assert_eq!(acc, (VecDeque::from([10, 8, 6, 4, 2]), vec![-1, 1, 3, 5, 7]));
    }
}
