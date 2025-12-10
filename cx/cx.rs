#![allow(missing_docs)]
//! Composable left folds.

// Refs:
// - [transducers](https://clojure.org/reference/transducers)
// - [foldl](https://hackage.haskell.org/package/foldl)

mod filter;
mod map;

pub use filter::{Filter, filter};
pub use map::{Map, map};

/// A left associative fold.
pub trait Fold<T> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step(&mut self, acc: Self::Acc, input: T) -> Step<Self::Acc>;

    /// Invoked when folding is complete.
    fn done(self, acc: Self::Acc) -> Self::Acc;
}

/// The result of [Fold.step].
#[derive(Debug, Copy, Clone)]
pub enum Step<T> {
    /// Keep folding.
    Yield(T),
    /// Folding has completed.
    Return(T),
}

/// An adapter that creates a new [Fold] from the given one.
pub trait Adapter<Fl>: Chain {
    /// An another [Fold].
    type Fold;

    /// Creates a new [Fold] from the given one.
    fn apply(self, fold: Fl) -> Self::Fold;
}

/// Provides utilities to chain [Adapter]s.
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
