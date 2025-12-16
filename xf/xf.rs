#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

use std::borrow::Borrow;

mod comp;
mod filter;
mod identity;
mod map;
mod take;

use comp::Comp;
use filter::Filter;
use identity::Identity;
use map::Map;
use take::Take;

mod either;
mod fuse;
mod par;
use either::Either;
use fuse::Fuse;
use par::Par;

/// A fold step function.
pub trait Fold<A, B> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
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

// Exists only to compose xf and construct a Fold.
#[derive(Debug)]
pub struct Folding<Xf> {
    xf: Xf,
}

/// An adapter that creates a new [Fold] from the given one.
pub trait Xform<Sf> {
    /// A new step function created by apply.
    type Fold;

    /// Creates a new [Fold] from the given one.
    fn xform(self, fold: Sf) -> Self::Fold;

    // We can't implement adapters (e.g., map, filter) in Xform,
    // because rustc won't be able to infer the Sf type.
}

impl<Xf> Folding<Xf> {
    pub fn apply<F>(self, fold: F) -> Xf::Fold
    where
        Xf: Xform<F>,
    {
        self.xf.xform(fold)
    }

    fn new(xf: Xf) -> Self {
        Folding { xf }
    }

    fn comp<That>(self, that: That) -> Folding<Comp<Xf, That>> {
        Folding::new(Comp::new(self.xf, that))
    }
}

pub fn folding<A, B>() -> Folding<Identity<A, B>> {
    Folding::new(Identity::new())
}

pub fn map<F>(f: F) -> Folding<Map<F>> {
    Folding::new(Map::new(f))
}

pub fn filter<P>(pred: P) -> Folding<Filter<P>> {
    Folding::new(Filter::new(pred))
}

pub fn take(count: usize) -> Folding<Take> {
    Folding::new(Take::new(count))
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
