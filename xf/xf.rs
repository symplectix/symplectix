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
        Par(Fuse::new(self), Fuse::new(that))
    }

    fn either<That>(self, that: That) -> Either<Self, That>
    where
        Self: Sized,
    {
        Either(self, that)
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
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        Step::More(self(acc, input.borrow()))
    }
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

#[derive(Debug)]
struct Fuse<F> {
    f: F,
    halt: bool,
}
impl<F> Fuse<F> {
    fn new(f: F) -> Self {
        Fuse { f, halt: false }
    }
}

impl<A, B, F> Fold<A, B> for Fuse<F>
where
    F: Fold<A, B>,
{
    type Acc = F::Acc;

    fn step<In>(&mut self, acc: <F as Fold<A, B>>::Acc, input: &In) -> Step<<F as Fold<A, B>>::Acc>
    where
        In: Borrow<A>,
    {
        if self.halt {
            Step::Halt(acc)
        } else {
            match self.f.step(acc, input) {
                Step::Halt(ret) => {
                    self.halt = true;
                    Step::Halt(ret)
                }
                step => step,
            }
        }
    }

    fn done(self, acc: Self::Acc) -> B {
        self.f.done(acc)
    }
}

#[derive(Debug)]
pub struct Par<F, G>(Fuse<F>, Fuse<G>);
impl<A, B, C, F, G> Fold<A, (B, C)> for Par<F, G>
where
    F: Fold<A, B>,
    G: Fold<A, C>,
{
    type Acc = (<F as Fold<A, B>>::Acc, <G as Fold<A, C>>::Acc);
    fn step<T>(&mut self, acc: Self::Acc, input: &T) -> Step<Self::Acc>
    where
        T: Borrow<A>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::More(a), Step::More(b)) => Step::More((a, b)),
            (Step::Halt(a), Step::More(b)) => Step::More((a, b)),
            (Step::More(a), Step::Halt(b)) => Step::More((a, b)),
            (Step::Halt(a), Step::Halt(b)) => Step::Halt((a, b)),
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}

#[derive(Debug)]
pub struct Either<F, G>(F, G);
impl<A, B, C, F, G> Fold<A, (B, C)> for Either<F, G>
where
    F: Fold<A, B>,
    G: Fold<A, C>,
{
    type Acc = (<F as Fold<A, B>>::Acc, <G as Fold<A, C>>::Acc);
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::More(a), Step::More(b)) => Step::More((a, b)),
            (Step::Halt(a), Step::More(b)) => Step::Halt((a, b)),
            (Step::More(a), Step::Halt(b)) => Step::Halt((a, b)),
            (Step::Halt(a), Step::Halt(b)) => Step::Halt((a, b)),
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}
