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
    fn xform(self, sf: Rf) -> Self::Fold {
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
