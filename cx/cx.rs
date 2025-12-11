#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [transducers](https://clojure.org/reference/transducers)
// - [cgrand/xforms]: https://github.com/cgrand/xforms
// - [foldl](https://hackage.haskell.org/package/foldl)

mod filter;
mod map;

pub use filter::{Filter, filter};
pub use map::{Map, map};

/// A fold step function.
pub trait Fold<T> {
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
    /// # use cx::{Adapter, Fold, Step};
    /// # struct SomeFoldFunction();
    /// # impl Fold<i32> for SomeFoldFunction {
    /// #     type Acc = usize;
    /// #     fn step(&mut self, mut acc: Self::Acc, _i: i32) -> Step<Self::Acc> {
    /// #         Step::Yield(acc + 1)
    /// #     }
    /// # }
    /// let fold = SomeFoldFunction();
    /// fold.done(0);
    /// fold.done(0);
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

/// An adapter that creates a new [Fold] from the given one.
pub trait Adapter<F>: Chain {
    /// The output of [Adapter.apply].
    type Fold;

    /// Creates a new [Fold] from the given one.
    fn apply(self, fold: F) -> Self::Fold;
}

/// Provides combinators to chain [Adapter]s.
pub trait Chain {
    /// Composes self and [`map(f)`].
    fn map<F>(self, f: F) -> Comp<Self, Map<F>>
    where
        Self: Sized,
    {
        comp(self, map(f))
    }

    /// Composes self and [`filter(p)`].
    fn filter<P>(self, p: P) -> Comp<Self, Filter<P>>
    where
        Self: Sized,
    {
        comp(self, filter(p))
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

impl<Fl, A, B> Adapter<Fl> for Comp<A, B>
where
    A: Adapter<B::Fold>,
    B: Adapter<Fl>,
{
    type Fold = A::Fold;

    fn apply(self, rf: Fl) -> Self::Fold {
        self.a.apply(self.b.apply(rf))
    }
}
impl<A, B> Chain for Comp<A, B> {}
