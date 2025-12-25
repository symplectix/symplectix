#![allow(missing_docs)]
//! Composable left folds.
//!
//! When you chain folds, they are evaluated in reverse order (right to left).
//!
//! ```
//! use std::ops::ControlFlow::Continue;
//! use ano::Fold;
//!
//! let sum = |acc, item| Continue(acc + item);
//! assert_eq!(4, sum.filter(|x: &i32| x % 2 != 0).take(3).fold_with(0, 1..));
//! ```

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

mod completing;
mod filter;
mod fuse;
mod map;
mod seq;
mod take;
mod with_initial_state;
mod zip;

use std::ops::ControlFlow::*;

use completing::Completing;
use filter::Filter;
use fuse::Fuse;
use map::Map;
use seq::Seq;
use take::Take;
use with_initial_state::WithInitialState;
use zip::Zip;

/// The result of [Fold.step].
pub type Step<T> = std::ops::ControlFlow<T, T>;

/// A composable left fold.
pub trait Fold<A, B> {
    /// The accumulator, used to store the intermediate result while folding.
    type State;

    /// Runs just a one step of folding.
    // TODO: consider to use Try instead of ControlFlow.
    // https://doc.rust-lang.org/std/ops/trait.Try.html
    // https://github.com/rust-lang/rust/issues/84277
    fn step(&mut self, acc: Self::State, item: A) -> Step<Self::State>;

    /// Invoked when folding is complete.
    fn complete(self, acc: Self::State) -> B;

    #[inline]
    fn fold<It>(self, iterable: It) -> B
    where
        Self: Sized + InitialState<Self::State>,
        It: IntoIterator<Item = A>,
    {
        let iter = iterable.into_iter();
        let init = self.initial_state(iter.size_hint());
        self.fold_with(init, iter)
    }

    #[inline]
    fn fold_with<It>(mut self, init: Self::State, iterable: It) -> B
    where
        Self: Sized,
        It: IntoIterator<Item = A>,
    {
        match iterable.into_iter().try_fold(init, |acc, v| self.step(acc, v)) {
            Continue(c) => self.complete(c),
            Break(b) => self.complete(b),
        }
    }

    fn completing<C, F>(self, f: F) -> Completing<Self, B, F>
    where
        Self: Sized,
        F: FnMut(B) -> C,
    {
        Completing::new(self, f)
    }

    fn with_initial_state<F>(self, f: F) -> WithInitialState<Self, F>
    where
        Self: Sized,
        F: Fn((usize, Option<usize>)) -> Self::State,
    {
        WithInitialState::new(self, f)
    }

    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
    {
        Map::new(self, f)
    }

    fn filter<P>(self, f: P) -> Filter<Self, P>
    where
        Self: Sized,
    {
        Filter::new(self, f)
    }

    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    fn seq<That>(self, that: That) -> Seq<Self, That>
    where
        Self: Sized,
    {
        Seq::new(self, that)
    }

    fn zip<'a, That>(self, that: That) -> Zip<'a, Self, That>
    where
        Self: Sized,
    {
        Zip::new(self, that)
    }
}

pub trait InitialState<T> {
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T;
}

impl<A, B, F> Fold<A, B> for F
where
    F: FnMut(B, A) -> Step<B>,
{
    type State = B;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> Step<Self::State> {
        (self)(acc, item)
    }

    #[inline]
    fn complete(self, acc: Self::State) -> B {
        acc
    }
}
