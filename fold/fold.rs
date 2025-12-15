#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

pub mod xf;
use std::borrow::Borrow;

/// Fold represents a left fold computation from a collection of A to B.
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
                Step::Yield(ret) => {
                    acc = ret;
                }
                Step::Break(ret) => {
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
    Yield(T),
    /// Stop folding.
    Break(T),
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
        Step::Yield(self(acc, input.borrow()))
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
pub struct Map<F, MapF> {
    f: F,
    mapf: MapF,
}
impl<F, MapF> Map<F, MapF> {
    pub(crate) fn new(f: F, mapf: MapF) -> Self {
        Map { f, mapf }
    }
}
impl<A, B, C, F, MapF> Fold<A, C> for Map<F, MapF>
where
    F: Fold<B, C>,
    MapF: FnMut(&A) -> B,
{
    type Acc = F::Acc;
    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        let mapped = (self.mapf)(input.borrow());
        self.f.step(acc, &mapped)
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> C {
        self.f.done(acc)
    }
}

#[derive(Debug)]
pub struct Filter<F, P> {
    f: F,
    pred: P,
}
impl<F, P> Filter<F, P> {
    pub(crate) fn new(f: F, pred: P) -> Self {
        Filter { f, pred }
    }
}
impl<A, B, F, P> Fold<A, B> for Filter<F, P>
where
    F: Fold<A, B>,
    P: FnMut(&A) -> bool,
{
    type Acc = F::Acc;
    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        if (self.pred)(input.borrow()) { self.f.step(acc, input) } else { Step::Yield(acc) }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.f.done(acc)
    }
}

#[derive(Debug)]
pub struct Take<F> {
    f: F,
    count: usize,
}
impl<F> Take<F> {
    pub(crate) fn new(f: F, count: usize) -> Self {
        Take { f, count }
    }
}
impl<A, B, F> Fold<A, B> for Take<F>
where
    F: Fold<A, B>,
{
    type Acc = F::Acc;
    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        match self.count {
            0 => Step::Break(acc),
            1 => {
                self.count = 0;
                match self.f.step(acc, input) {
                    Step::Yield(a) => Step::Break(a),
                    Step::Break(a) => Step::Break(a),
                }
            }
            _ => {
                self.count -= 1;
                self.f.step(acc, input)
            }
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.f.done(acc)
    }
}

#[derive(Debug)]
struct Fuse<F> {
    f: F,
    complete: bool,
}
impl<F> Fuse<F> {
    fn new(f: F) -> Self {
        Fuse { f, complete: false }
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
        if self.complete {
            Step::Break(acc)
        } else {
            match self.f.step(acc, input) {
                Step::Break(ret) => {
                    self.complete = true;
                    Step::Break(ret)
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
            (Step::Yield(a), Step::Yield(b)) => Step::Yield((a, b)),
            (Step::Break(a), Step::Yield(b)) => Step::Yield((a, b)),
            (Step::Yield(a), Step::Break(b)) => Step::Yield((a, b)),
            (Step::Break(a), Step::Break(b)) => Step::Break((a, b)),
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}

#[derive(Debug)]
pub struct Either<F, G>(pub(crate) F, pub(crate) G);
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
            (Step::Yield(a), Step::Yield(b)) => Step::Yield((a, b)),
            (Step::Break(a), Step::Yield(b)) => Step::Break((a, b)),
            (Step::Yield(a), Step::Break(b)) => Step::Break((a, b)),
            (Step::Break(a), Step::Break(b)) => Step::Break((a, b)),
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}
