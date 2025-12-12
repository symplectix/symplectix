#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [transducers](https://clojure.org/reference/transducers)
// - [cgrand/xforms]: https://github.com/cgrand/xforms
// - [foldl](https://hackage.haskell.org/package/foldl)

mod filter;
mod map;

use std::marker::PhantomData;

pub use filter::Filter;
pub use map::Map;

pub trait Fold<T> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step(&mut self, acc: Self::Acc, input: T) -> Step<Self::Acc>;

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

impl<T, A, B> Fold<T> for (A, B)
where
    T: Clone,
    A: Fold<T>,
    B: Fold<T>,
{
    type Acc = (<A as Fold<T>>::Acc, <B as Fold<T>>::Acc);

    fn step(&mut self, acc: Self::Acc, input: T) -> Step<Self::Acc> {
        let a = self.0.step(acc.0, input.clone());
        let b = self.1.step(acc.1, input);
        match (a, b) {
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

/// The result of [Fold.step].
#[derive(Debug, Copy, Clone)]
pub enum Step<T> {
    /// Keep folding.
    Yield(T),
    /// Stop folding.
    Break(T),
}

#[derive(Debug)]
pub struct Xform<Xf> {
    xf: Xf,
}

/// An adapter that creates a new [StepFn] from the given one.
pub trait XFn<Sf> {
    /// A new step function created by apply.
    type StepFn;

    /// Creates a new [StepFn] from the given one.
    fn apply(self, step_fn: Sf) -> Self::StepFn;
}

fn xform<T>() -> Xform<Id<T>> {
    Xform { xf: id() }
}

impl<Xf> Xform<Xf> {
    fn apply<Sf>(self, step_fn: Sf) -> Xf::StepFn
    where
        Xf: XFn<Sf>,
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

impl<Sf, T> XFn<Sf> for Id<T>
where
    Sf: Fold<T>,
{
    type StepFn = Sf;

    #[inline]
    fn apply(self, step_fn: Sf) -> Self::StepFn {
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

impl<Sf, A, B> XFn<Sf> for Comp<A, B>
where
    A: XFn<B::StepFn>,
    B: XFn<Sf>,
{
    type StepFn = A::StepFn;

    fn apply(self, rf: Sf) -> Self::StepFn {
        self.a.apply(self.b.apply(rf))
    }
}

#[cfg(test)]
mod tests {
    use super::{Fold, Step, XFn};

    struct PushVec;

    impl<T> Fold<T> for PushVec {
        type Acc = Vec<T>;

        fn step(&mut self, mut acc: Self::Acc, input: T) -> Step<Self::Acc> {
            acc.push(input);
            Step::Yield(acc)
        }
    }

    struct ConsVec;

    impl<T> Fold<T> for ConsVec {
        type Acc = Vec<T>;

        fn step(&mut self, mut acc: Self::Acc, input: T) -> Step<Self::Acc> {
            acc.insert(0, input);
            Step::Yield(acc)
        }
    }

    #[test]
    fn test_map_filter_step() {
        let mut fold = (
            super::xform().map(|x| x * 2 + 1).filter(|x: &i32| 10 < *x && *x < 20).apply(ConsVec),
            super::xform().map(|x| x * 2 + 1).filter(|x: &i32| 10 < *x && *x < 20).apply(PushVec),
        );
        let mut acc = (vec![], vec![]);
        for i in 0..20 {
            match fold.step(acc, i) {
                Step::Yield(ret) => {
                    acc = ret;
                }
                Step::Break(ret) => {
                    acc = fold.done(ret);
                    break;
                }
            }
        }
        assert_eq!(acc, (vec![11, 13, 15, 17, 19], vec![11, 13, 15, 17, 19]));
    }
}
