#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

use std::ops::ControlFlow;

mod filter;
mod map;
mod take;
use filter::Filter;
use map::Map;
use take::Take;

mod from_fn;
mod fuse;
mod par;
mod seq;
use from_fn::FromFn;
use fuse::Fuse;
use par::Par;
use seq::Seq;

pub mod xf;

/// A fold step function.
///
/// When you chain Folds, they are evaluated in reverse order.
///
/// ```
/// use ano::Fold;
/// let sum = ano::from_fn(|acc, item: i32| acc + item);
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
///         .into_fn(|acc, item: i32| acc + item)
///         .fold(0, 1..)
/// );
/// ```
pub trait Fold<A, B> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    // TODO: use Try instead of ControlFlow
    // https://doc.rust-lang.org/std/ops/trait.Try.html
    // https://github.com/rust-lang/rust/issues/84277
    fn step(&mut self, acc: Self::Acc, item: A) -> Step<Self::Acc>;

    /// Invoked when folding is complete.
    ///
    /// You must call `done` exactly once.
    fn done(self, acc: Self::Acc) -> B;

    fn fold<It>(mut self, mut acc: Self::Acc, iterable: It) -> B
    where
        Self: Sized,
        It: IntoIterator<Item = A>,
    {
        use ControlFlow::*;

        for item in iterable.into_iter() {
            match self.step(acc, item) {
                Continue(ret) => {
                    acc = ret;
                }
                Break(ret) => {
                    acc = ret;
                    break;
                }
            }
        }
        self.done(acc)
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

    fn par<That>(self, that: That) -> Par<Self, That>
    where
        Self: Sized,
    {
        Par::new(self, that)
    }
}

/// The result of [Fold.step].
type Step<T> = ControlFlow<T, T>;

pub fn from_fn<A, B, F>(f: F) -> FromFn<F>
where
    F: FnMut(B, A) -> B,
{
    FromFn::new(f)
}
