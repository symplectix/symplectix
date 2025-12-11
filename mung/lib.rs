#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [transducers](https://clojure.org/reference/transducers)
// - [cgrand/xforms]: https://github.com/cgrand/xforms
// - [foldl](https://hackage.haskell.org/package/foldl)

mod filter;
mod map;

pub use filter::Filter;
pub use map::Map;

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

/// An adapter that creates a new [StepFn] from the given one.
pub trait Xform<Sf> {
    /// A new step function created by apply.
    type StepFn;

    /// Creates a new [StepFn] from the given one.
    fn apply(self, step_fn: Sf) -> Self::StepFn;
}

#[derive(Debug)]
pub struct Fold<Sf> {
    sf: Sf,
}

#[derive(Debug)]
pub struct Prep<Xf> {
    xf: Xf,
}

impl<Xf> Prep<Xf> {
    pub fn apply<Sf>(self, step_fn: Sf) -> Fold<Xf::StepFn>
    where
        Xf: Xform<Sf>,
    {
        Fold { sf: self.xf.apply(step_fn) }
    }

    pub(crate) fn new(xf: Xf) -> Self {
        Prep { xf }
    }

    pub fn map<F>(self, f: F) -> Prep<Comp<Xf, Map<F>>> {
        Prep::new(comp(self.xf, Map::new(f)))
    }

    pub fn filter<P>(self, predicate: P) -> Prep<Comp<Xf, Filter<P>>> {
        Prep::new(comp(self.xf, Filter::new(predicate)))
    }
}

pub fn map<F>(f: F) -> Prep<Map<F>> {
    Prep::new(Map::new(f))
}

pub fn filter<P>(predicate: P) -> Prep<Filter<P>> {
    Prep::new(Filter::new(predicate))
}

fn comp<A, B>(a: A, b: B) -> Comp<A, B> {
    Comp { a, b }
}

/// Comp is an adapter of [Adapter]s.
pub struct Comp<A, B> {
    a: A,
    b: B,
}

impl<Fl, A, B> Xform<Fl> for Comp<A, B>
where
    A: Xform<B::StepFn>,
    B: Xform<Fl>,
{
    type StepFn = A::StepFn;

    fn apply(self, rf: Fl) -> Self::StepFn {
        self.a.apply(self.b.apply(rf))
    }
}

#[cfg(test)]
mod tests {
    use super::{Step, StepFn, Xform};

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
        let mut acc = vec![];
        let mut fold = super::map(|x| x * 2 + 1).filter(|x: &i32| 10 < *x && *x < 20).apply(PushVec);
        for i in 0..20 {
            match fold.sf.step(acc, i) {
                Step::Yield(ret) => {
                    acc = ret;
                }
                Step::Break(ret) => {
                    acc = fold.sf.done(ret);
                    break;
                }
            }
        }
        assert_eq!(acc, vec![11, 13, 15, 17, 19]);
    }
}
