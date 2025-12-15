use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::{Fold, Step};

// Exists only to compose xf and construct a Fold.
#[derive(Debug)]
pub struct Folding<Xf> {
    xf: Xf,
}

/// An adapter that creates a new [Fold] from the given one.
pub trait Xform<F> {
    /// A new step function created by apply.
    type Fold;

    /// Creates a new [Fold] from the given one.
    fn apply(self, fold: F) -> Self::Fold;
}

impl<Xf> Folding<Xf> {
    pub fn apply<F>(self, fold: F) -> Xf::Fold
    where
        Xf: Xform<F>,
    {
        self.xf.apply(fold)
    }

    fn new(xf: Xf) -> Self {
        Folding { xf }
    }

    fn comp<That>(self, that: That) -> Folding<Comp<Xf, That>> {
        Folding::new(comp(self.xf, that))
    }

    pub fn map<F>(self, mapf: F) -> Folding<Comp<Xf, Map<F>>> {
        self.comp(Map::new(mapf))
    }

    pub fn filter<P>(self, pred: P) -> Folding<Comp<Xf, Filter<P>>> {
        self.comp(Filter::new(pred))
    }

    pub fn take(self, count: usize) -> Folding<Comp<Xf, Take>> {
        self.comp(Take::new(count))
    }
}

#[derive(Debug)]
pub struct Id<In, Out>(PhantomData<(In, Out)>);
impl<Sf: Fold<In, Out>, In, Out> Xform<Sf> for Id<In, Out> {
    type Fold = Sf;
    #[inline]
    fn apply(self, step_fn: Sf) -> Self::Fold {
        step_fn
    }
}
pub fn folding<In, Out>() -> Folding<Id<In, Out>> {
    Folding::new(Id(PhantomData))
}

/// Comp is an adapter of [Adapter]s.
#[derive(Debug)]
pub struct Comp<A, B> {
    a: A,
    b: B,
}
fn comp<A, B>(a: A, b: B) -> Comp<A, B> {
    Comp { a, b }
}
impl<Sf, A, B> Xform<Sf> for Comp<A, B>
where
    A: Xform<B::Fold>,
    B: Xform<Sf>,
{
    type Fold = A::Fold;

    fn apply(self, rf: Sf) -> Self::Fold {
        self.a.apply(self.b.apply(rf))
    }
}

#[derive(Debug)]
pub struct Map<F> {
    mapf: F,
}

#[derive(Debug)]
pub struct MapFold<F, MapF> {
    f: F,
    mapf: MapF,
}

impl<Sf, F> Xform<Sf> for Map<F> {
    type Fold = MapFold<Sf, F>;
    fn apply(self, sf: Sf) -> Self::Fold {
        MapFold::new(sf, self.mapf)
    }
}

pub fn map<F>(f: F) -> Folding<Map<F>> {
    Folding::new(Map::new(f))
}

impl<F> Map<F> {
    fn new(mapf: F) -> Map<F> {
        Map { mapf }
    }
}

impl<F, MapF> MapFold<F, MapF> {
    pub(crate) fn new(f: F, mapf: MapF) -> Self {
        MapFold { f, mapf }
    }
}

impl<A, B, C, F, MapF> Fold<A, C> for MapFold<F, MapF>
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
pub struct Filter<P> {
    pred: P,
}

#[derive(Debug)]
pub struct FilterFold<F, P> {
    f: F,
    pred: P,
}

impl<Sf, P> Xform<Sf> for Filter<P> {
    type Fold = FilterFold<Sf, P>;
    fn apply(self, sf: Sf) -> Self::Fold {
        FilterFold::new(sf, self.pred)
    }
}

pub fn filter<P>(pred: P) -> Folding<Filter<P>> {
    Folding::new(Filter::new(pred))
}

impl<P> Filter<P> {
    fn new(pred: P) -> Self {
        Filter { pred }
    }
}

impl<F, P> FilterFold<F, P> {
    pub(crate) fn new(f: F, pred: P) -> Self {
        FilterFold { f, pred }
    }
}

impl<A, B, F, P> Fold<A, B> for FilterFold<F, P>
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
pub struct Take {
    count: usize,
}

#[derive(Debug)]
pub struct TakeFold<F> {
    f: F,
    count: usize,
}

impl<Sf> Xform<Sf> for Take {
    type Fold = TakeFold<Sf>;
    fn apply(self, sf: Sf) -> Self::Fold {
        TakeFold::new(sf, self.count)
    }
}

pub fn take(count: usize) -> Folding<Take> {
    Folding::new(Take::new(count))
}

impl Take {
    fn new(count: usize) -> Self {
        Take { count }
    }
}
impl<F> TakeFold<F> {
    pub(crate) fn new(f: F, count: usize) -> Self {
        TakeFold { f, count }
    }
}
impl<A, B, F> Fold<A, B> for TakeFold<F>
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
