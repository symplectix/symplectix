use std::borrow::Borrow;

use crate::{Comp, Fold, Folding, Step, Xform};

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
        if (self.pred)(input.borrow()) { self.sf.step(acc, input) } else { Step::More(acc) }
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
            0 => Step::Halt(acc),
            1 => {
                self.count = 0;
                match self.f.step(acc, input) {
                    Step::More(a) => Step::Halt(a),
                    Step::Halt(a) => Step::Halt(a),
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
