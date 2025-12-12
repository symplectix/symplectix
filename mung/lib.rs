#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [transducers](https://clojure.org/reference/transducers)
// - [cgrand/xforms]: https://github.com/cgrand/xforms
// - [foldl](https://hackage.haskell.org/package/foldl)

mod filter;
mod map;

use std::borrow::Borrow;
use std::marker::PhantomData;

pub use filter::Filter;
pub use map::Map;

pub trait Fold<T> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>;

    /// Invoked when folding is complete.
    ///
    /// - By default, done just returns acc.
    /// - You must call `done` exactly once.
    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc
    where
        Self: Sized,
    {
        acc
    }
}

/// The result of [Fold.step].
#[derive(Debug, Copy, Clone)]
pub enum Step<T> {
    /// Keep folding.
    Yield(T),
    /// Stop folding.
    Break(T),
}

impl<T, A, B> Fold<T> for (A, B)
where
    A: Fold<T>,
    B: Fold<T>,
{
    type Acc = (<A as Fold<T>>::Acc, <B as Fold<T>>::Acc);

    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::Yield(a), Step::Yield(b)) => Step::Yield((a, b)),
            (Step::Break(a), Step::Yield(b)) => Step::Break((a, b)),
            (Step::Yield(a), Step::Break(b)) => Step::Break((a, b)),
            (Step::Break(a), Step::Break(b)) => Step::Break((a, b)),
        }
    }

    fn done(self, acc: Self::Acc) -> Self::Acc {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}

// Xform exists only to compose xf and an another XformFn.
#[derive(Debug)]
pub struct Xform<Xf> {
    xf: Xf,
}

/// An adapter that creates a new [Fold] from the given one.
pub trait XformFn<Sf> {
    /// A new step function created by apply.
    type Fold;

    /// Creates a new [Fold] from the given one.
    fn apply(self, sf: Sf) -> Self::Fold;
}

impl<T> Xform<Id<T>> {
    fn id() -> Self {
        Xform { xf: id() }
    }
}

impl<Xf> Xform<Xf> {
    fn apply<Sf>(self, step_fn: Sf) -> Xf::Fold
    where
        Xf: XformFn<Sf>,
    {
        self.xf.apply(step_fn)
    }

    fn comp<That>(self, that: That) -> Xform<Comp<Xf, That>> {
        Xform { xf: comp(self.xf, that) }
    }

    pub fn map<F>(self, f: F) -> Xform<Comp<Xf, Map<F>>> {
        self.comp(Map::new(f))
    }

    pub fn filter<P>(self, pred: P) -> Xform<Comp<Xf, Filter<P>>> {
        self.comp(Filter::new(pred))
    }
}

pub struct Id<T>(PhantomData<T>);

fn id<T>() -> Id<T> {
    Id(PhantomData)
}

// impl<T> StepFn<T> for Id<T> {
//     type Acc = T;
//     fn step(&mut self, _acc: Self::Acc, input: T) -> Step<Self::Acc> {
//         input
//     }
//     fn done(self, acc: Self::Acc) -> Self::Acc {
//         acc
//     }
// }

impl<Sf, T> XformFn<Sf> for Id<T>
where
    Sf: Fold<T>,
{
    type Fold = Sf;

    #[inline]
    fn apply(self, step_fn: Sf) -> Self::Fold {
        step_fn
    }
}

fn comp<A, B>(a: A, b: B) -> Comp<A, B> {
    Comp { a, b }
}

/// Comp is an adapter of [Adapter]s.
pub struct Comp<A, B> {
    a: A,
    b: B,
}

impl<Sf, A, B> XformFn<Sf> for Comp<A, B>
where
    A: XformFn<B::Fold>,
    B: XformFn<Sf>,
{
    type Fold = A::Fold;

    fn apply(self, rf: Sf) -> Self::Fold {
        self.a.apply(self.b.apply(rf))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, ToOwned};
    use std::collections::VecDeque;

    use super::{Fold, Step, Xform, XformFn};

    struct Conj;

    impl<T> Fold<T> for Conj
    where
        T: ToOwned,
    {
        type Acc = Vec<T::Owned>;

        fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> Step<Self::Acc>
        where
            Q: Borrow<T>,
        {
            acc.push(input.borrow().to_owned());
            Step::Yield(acc)
        }
    }

    struct Cons;

    impl<T> Fold<T> for Cons
    where
        T: ToOwned,
    {
        type Acc = VecDeque<T::Owned>;

        fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> Step<Self::Acc>
        where
            Q: Borrow<T>,
        {
            acc.push_front(input.borrow().to_owned());
            Step::Yield(acc)
        }
    }

    #[test]
    fn test_map_filter_step() {
        let mut fold = (
            Xform::id().map(|x: &i32| x + 1).filter(|x: &i32| *x % 2 == 0).apply(Cons),
            Xform::id().map(|x: &i32| x - 1).filter(|x: &i32| *x % 2 != 0).apply(Conj),
        );
        let mut acc = (VecDeque::with_capacity(10), vec![]);
        for i in 0..10 {
            match fold.step(acc, &i) {
                Step::Yield(ret) => {
                    acc = ret;
                }
                Step::Break(ret) => {
                    acc = fold.done(ret);
                    break;
                }
            }
        }
        assert_eq!(acc, (VecDeque::from([10, 8, 6, 4, 2]), vec![-1, 1, 3, 5, 7]));
    }
}
