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
pub trait Xform<Sf>: Chain {
    /// A new step function created by apply.
    type StepFn;

    /// Creates a new [StepFn] from the given one.
    fn apply(self, step_fn: Sf) -> Self::StepFn;
}

pub trait Chain {
    fn map<F>(self, f: F) -> Comp<Self, Map<F>>
    where
        Self: Sized,
    {
        comp(self, Map::new(f))
    }

    fn filter<P>(self, predicate: P) -> Comp<Self, Filter<P>>
    where
        Self: Sized,
    {
        comp(self, Filter::new(predicate))
    }
}

pub fn map<F>(f: F) -> Map<F> {
    Map::new(f)
}

pub fn filter<P>(predicate: P) -> Filter<P> {
    Filter::new(predicate)
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
impl<A, B> Chain for Comp<A, B> {}
