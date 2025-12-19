#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

mod filter;
mod map;
mod take;
use filter::Filter;
use map::Map;
use take::Take;

mod fuse;
mod par;
mod seq;
use fuse::Fuse;
use par::Par;
use seq::Seq;

mod from_fn;
use from_fn::{Completing, Using};

pub mod xf;

/// The result of [Fold.step].
type ControlFlow<T> = std::ops::ControlFlow<T, T>;

/// A fold step function.
///
/// When you chain Folds, they are evaluated in reverse order.
///
/// ```
/// use ano::Fold;
/// let sum = |acc, item| acc + item;
/// assert_eq!(4, sum.filter(|x: &i32| x % 2 != 0).take(3).fold(0, 1..));
/// ```
///
/// You can use `xf` module to write pipelines in forward order.
///
/// ```
/// use ano::Fold;
/// assert_eq!(
///     4,
///     ano::xf::take(3)
///         .filter(|x: &i32| x % 2 != 0)
///         .apply(|acc, item| acc + item)
///         .fold(0, 1..)
/// );
/// ```
pub trait Fold<A, B> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    // TODO: consider to use Try instead of ControlFlow.
    // https://doc.rust-lang.org/std/ops/trait.Try.html
    // https://github.com/rust-lang/rust/issues/84277
    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc>;

    /// Invoked when folding is complete.
    ///
    /// You must call `done` exactly once.
    fn done(self, acc: Self::Acc) -> B;

    fn fold<It>(mut self, init: Self::Acc, iterable: It) -> B
    where
        Self: Sized,
        It: IntoIterator<Item = A>,
    {
        use std::ops::ControlFlow::*;
        match iterable.into_iter().try_fold(init, |acc, v| self.step(acc, v)) {
            Continue(c) => self.done(c),
            Break(b) => self.done(b),
        }
    }

    #[inline]
    fn fold_init<It>(self, iterable: It) -> B
    where
        Self: Sized + Init<A, B>,
        It: IntoIterator<Item = A>,
    {
        let iter = iterable.into_iter();
        let init = self.init(iter.size_hint());
        self.fold(init, iter)
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

    fn completing<C, F>(self, f: F) -> Completing<Self, B, F>
    where
        Self: Sized,
        F: FnMut(B) -> C,
    {
        Completing::new(self, f)
    }

    fn using<F>(self, f: F) -> Using<Self, F>
    where
        Self: Sized,
        F: Fn((usize, Option<usize>)) -> Self::Acc,
    {
        Using::new(self, f)
    }

    fn seq<That>(self, that: That) -> Seq<Self, That>
    where
        Self: Sized,
    {
        Seq::new(self, that)
    }

    fn par<'a, That>(self, that: That) -> Par<'a, Self, That>
    where
        Self: Sized,
    {
        Par::new(self, that)
    }
}

pub trait Init<A, B>: Fold<A, B> {
    fn init(&self, size_hint: (usize, Option<usize>)) -> Self::Acc;
}
