use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::{Fold, Step};

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
    fn apply(self, fold: Sf) -> Self::Fold;

    // We can't implement adapters (e.g., map, filter) in Xform,
    // because rustc won't be able to infer the Sf type.
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
}

#[derive(Debug)]
pub struct Id<A, B>(PhantomData<(A, B)>);
impl<A, B, Sf: Fold<A, B>> Xform<Sf> for Id<A, B> {
    type Fold = Sf;
    #[inline]
    fn apply(self, step_fn: Sf) -> Self::Fold {
        step_fn
    }
}
pub fn folding<A, B>() -> Folding<Id<A, B>> {
    Folding::new(Id(PhantomData))
}

/// Comp is an adapter of [Adapter]s.
#[derive(Debug)]
pub struct Comp<F, G> {
    f: F,
    g: G,
}
fn comp<F, G>(f: F, g: G) -> Comp<F, G> {
    Comp { f, g }
}
impl<Sf, F, G> Xform<Sf> for Comp<F, G>
where
    F: Xform<G::Fold>,
    G: Xform<Sf>,
{
    type Fold = F::Fold;

    fn apply(self, rf: Sf) -> Self::Fold {
        self.f.apply(self.g.apply(rf))
    }
}

#[derive(Debug)]
pub struct Map<F> {
    mapf: F,
}

impl<F> Map<F> {
    fn new(mapf: F) -> Map<F> {
        Map { mapf }
    }
}

#[derive(Debug)]
pub struct MapSf<Sf, F> {
    sf: Sf,
    mapf: F,
}

impl<F, MapF> MapSf<F, MapF> {
    fn new(f: F, mapf: MapF) -> Self {
        MapSf { sf: f, mapf }
    }
}

impl<Rf, F> Xform<Rf> for Map<F> {
    type Fold = MapSf<Rf, F>;
    fn apply(self, sf: Rf) -> Self::Fold {
        MapSf::new(sf, self.mapf)
    }
}

impl<Xf> Folding<Xf> {
    pub fn map<F>(self, mapf: F) -> Folding<Comp<Xf, Map<F>>> {
        self.comp(Map::new(mapf))
    }
}

pub fn map<F>(f: F) -> Folding<Map<F>> {
    Folding::new(Map::new(f))
}

impl<A, B, C, Sf, F> Fold<A, C> for MapSf<Sf, F>
where
    Sf: Fold<B, C>,
    F: FnMut(&A) -> B,
{
    type Acc = Sf::Acc;
    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        let mapped = (self.mapf)(input.borrow());
        self.sf.step(acc, &mapped)
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> C {
        self.sf.done(acc)
    }
}

#[derive(Debug)]
pub struct Filter<P> {
    pred: P,
}

impl<P> Filter<P> {
    fn new(pred: P) -> Self {
        Filter { pred }
    }
}

#[derive(Debug)]
pub struct FilterSf<Sf, P> {
    sf: Sf,
    pred: P,
}

impl<F, P> FilterSf<F, P> {
    fn new(f: F, pred: P) -> Self {
        FilterSf { sf: f, pred }
    }
}

impl<Sf, P> Xform<Sf> for Filter<P> {
    type Fold = FilterSf<Sf, P>;
    fn apply(self, sf: Sf) -> Self::Fold {
        FilterSf::new(sf, self.pred)
    }
}

impl<Xf> Folding<Xf> {
    pub fn filter<P>(self, pred: P) -> Folding<Comp<Xf, Filter<P>>> {
        self.comp(Filter::new(pred))
    }
}
pub fn filter<P>(pred: P) -> Folding<Filter<P>> {
    Folding::new(Filter::new(pred))
}

impl<A, B, Sf, P> Fold<A, B> for FilterSf<Sf, P>
where
    Sf: Fold<A, B>,
    P: FnMut(&A) -> bool,
{
    type Acc = Sf::Acc;
    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        if (self.pred)(input.borrow()) { self.sf.step(acc, input) } else { Step::Yield(acc) }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.sf.done(acc)
    }
}

#[derive(Debug)]
pub struct Take {
    count: usize,
}
impl Take {
    fn new(count: usize) -> Self {
        Take { count }
    }
}

#[derive(Debug)]
pub struct TakeSf<F> {
    f: F,
    count: usize,
}
impl<F> TakeSf<F> {
    fn new(f: F, count: usize) -> Self {
        TakeSf { f, count }
    }
}

impl<Sf> Xform<Sf> for Take {
    type Fold = TakeSf<Sf>;
    fn apply(self, sf: Sf) -> Self::Fold {
        TakeSf::new(sf, self.count)
    }
}

impl<Xf> Folding<Xf> {
    pub fn take(self, count: usize) -> Folding<Comp<Xf, Take>> {
        self.comp(Take::new(count))
    }
}
pub fn take(count: usize) -> Folding<Take> {
    Folding::new(Take::new(count))
}

impl<A, B, F> Fold<A, B> for TakeSf<F>
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
