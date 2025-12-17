#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

use std::borrow::Borrow;

mod filter;
mod map;
mod take;
use filter::Filter;
use map::Map;
use take::Take;

mod either;
mod from_fn;
mod fuse;
mod par;
use either::Either;
use from_fn::FromFn;
use fuse::Fuse;
use par::Par;

pub mod xf;

/// A fold step function.
///
/// When you chain Folds, they are evaluated in reverse order.
///
/// ```
/// use ano::Fold;
/// let sum = ano::from_fn(|acc, item: &i32| acc + item);
/// assert_eq!(4, sum.filter(|x: &i32| x % 2 != 0).take(3).fold(0, 1..));
/// ```
///
/// You can use `xf` module to write pipelines in forward order.
///
/// ```
/// use ano::Fold;
/// let sum = ano::from_fn(|acc, item: &i32| acc + item);
/// assert_eq!(4, ano::xf::take(3).filter(|x: &i32| x % 2 != 0).apply(sum).fold(0, 1..));
/// ```
pub trait Fold<A, B> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step<In>(&mut self, acc: Self::Acc, item: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>;

    /// Invoked when folding is complete.
    ///
    /// You must call `done` exactly once.
    fn done(self, acc: Self::Acc) -> B;

    fn fold<It, T>(mut self, mut acc: Self::Acc, iterable: It) -> B
    where
        Self: Sized,
        It: IntoIterator<Item = T>,
        T: Borrow<A>,
    {
        for item in iterable.into_iter() {
            match self.step(acc, &item) {
                Step::More(ret) => {
                    acc = ret;
                }
                Step::Halt(ret) => {
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

    fn par<That>(self, that: That) -> Par<Self, That>
    where
        Self: Sized,
    {
        Par::new(self, that)
    }

    fn either<That>(self, that: That) -> Either<Self, That>
    where
        Self: Sized,
    {
        Either::new(self, that)
    }
}

/// The result of [Fold.step].
#[derive(Debug, Copy, Clone)]
pub enum Step<T> {
    /// Keep folding.
    More(T),
    /// Stop folding.
    Halt(T),
}

pub fn from_fn<A, B, F>(f: F) -> FromFn<F>
where
    F: FnMut(B, &A) -> B,
{
    FromFn::new(f)
}

impl<A, B, F> Fold<A, B> for F
where
    F: FnMut(B, &A) -> B,
{
    type Acc = B;

    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        Step::More(self(acc, input.borrow()))
    }

    #[inline]
    fn done(self, acc: B) -> B {
        acc
    }
}

#[inline]
pub fn count<T>(acc: usize, _input: &T) -> usize {
    acc + 1
}

#[inline]
pub fn sum<A, B>(mut acc: B, input: &A) -> B
where
    B: for<'a> std::ops::AddAssign<&'a A>,
{
    acc += input;
    acc
}
