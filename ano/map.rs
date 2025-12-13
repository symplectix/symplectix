use std::borrow::Borrow;

use crate::{Fold, Step, Xform};

#[derive(Debug)]
pub struct MapXf<F> {
    mapf: F,
}
impl<F> MapXf<F> {
    pub(crate) fn new(f: F) -> Self {
        MapXf { mapf: f }
    }
}

impl<Sf, F> Xform<Sf> for MapXf<F> {
    type Fold = Map<Sf, F>;

    fn apply(self, sf: Sf) -> Self::Fold {
        Map { sf, mapf: self.mapf }
    }
}

#[derive(Debug)]
pub struct Map<Sf, MapF> {
    sf: Sf,
    mapf: MapF,
}

impl<Sf, F, A, B> Fold<A> for Map<Sf, F>
where
    Sf: Fold<B>,
    F: FnMut(&A) -> B,
{
    type Acc = Sf::Acc;

    #[inline]
    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<A>,
    {
        let mapped = (self.mapf)(input.borrow());
        self.sf.step(acc, &mapped)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.sf.done(acc)
    }
}
