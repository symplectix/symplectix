use crate::{Fold, Step, XformFn};

#[derive(Debug)]
pub struct Map<F> {
    mapf: F,
}
impl<F> Map<F> {
    pub(crate) fn new(f: F) -> Self {
        Map { mapf: f }
    }
}

impl<Sf, F> XformFn<Sf> for Map<F> {
    type Fold = MapFold<Sf, F>;

    fn apply(self, sf: Sf) -> Self::Fold {
        MapFold { sf, mapf: self.mapf }
    }
}

#[derive(Debug)]
pub struct MapFold<Sf, MapF> {
    sf: Sf,
    mapf: MapF,
}

impl<Sf, F, A, B> Fold<A> for MapFold<Sf, F>
where
    Sf: Fold<B>,
    F: FnMut(A) -> B,
{
    type Acc = Sf::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, input: A) -> Step<Self::Acc> {
        self.sf.step(acc, (self.mapf)(input))
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.sf.done(acc)
    }
}
