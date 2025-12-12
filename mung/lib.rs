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

#[derive(Debug)]
pub struct Fold<Sf> {
    // A step function of this fold.
    sf: Sf,
}

/// A fold step function.
pub trait StepFn<T> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step(&mut self, acc: Self::Acc, input: T) -> Step<Self::Acc>;

    /// Invoked when folding is complete.
    /// By default, done just returns acc.
    ///
    /// You must call `done` exactly once.
    ///
    /// ```compile_fail
    /// # use mung::StepFn;
    /// # struct SomeStepFn();
    /// # impl StepFn<i32> for SomeStepFn {
    /// #     type Acc = usize;
    /// #     fn step(&mut self, mut acc: Self::Acc, _i: i32) -> mung::Step<Self::Acc> {
    /// #         mung::Step::Yield(acc + 1)
    /// #     }
    /// # }
    /// let f = SomeStepFn();
    /// f.done(0);
    /// f.done(0);
    /// ```
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
    Sf: StepFn<T>,
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
    use super::{Step, StepFn, XFn};

    struct PushVec;

    impl<T> StepFn<T> for PushVec {
        type Acc = Vec<T>;

        fn step(&mut self, mut acc: Self::Acc, v: T) -> Step<Self::Acc> {
            acc.push(v);
            Step::Yield(acc)
        }
    }

    #[test]
    fn test_map_filter_step() {
        let mut fold = super::xform().map(|x| x * 2 + 1).filter(|x: &i32| 10 < *x && *x < 20).apply(PushVec);
        let mut acc = vec![];
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
        assert_eq!(acc, vec![11, 13, 15, 17, 19]);
    }
}
