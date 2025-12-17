use std::borrow::Borrow;

use crate::{Comp, Fold, Folding, Step, Xform};

#[derive(Debug)]
pub struct Map<Rf, F> {
    rf: Rf,
    mapf: F,
}

impl<Rf, F> Map<Rf, F> {
    pub(crate) fn new(rf: Rf, mapf: F) -> Self {
        Map { rf, mapf }
    }
}

impl<A, B, C, Rf, F> Fold<A, C> for Map<Rf, F>
where
    Rf: Fold<B, C>,
    F: FnMut(&A) -> B,
{
    type Acc = Rf::Acc;

    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        let mapped = (self.mapf)(input.borrow());
        self.rf.step(acc, &mapped)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> C {
        self.rf.done(acc)
    }
}

#[derive(Debug)]
pub struct MapXf<F> {
    mapf: F,
}

impl<F> MapXf<F> {
    pub(crate) fn new(mapf: F) -> MapXf<F> {
        MapXf { mapf }
    }
}

impl<Rf, F> Xform<Rf> for MapXf<F> {
    type Fold = Map<Rf, F>;
    fn xform(self, sf: Rf) -> Self::Fold {
        Map::new(sf, self.mapf)
    }
}

impl<Xf> Folding<Xf> {
    pub fn map<F>(self, mapf: F) -> Folding<Comp<Xf, MapXf<F>>> {
        self.comp(MapXf::new(mapf))
    }
}
