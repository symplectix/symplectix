use crate::{Adapter, Chain, Fold, Step};

pub fn map<F>(f: F) -> Map<F> {
    Map { mapf: f }
}

pub struct Map<F> {
    mapf: F,
}

pub struct MapFold<F, MapF> {
    fold: F,
    mapf: MapF,
}

impl<F, MapF> Adapter<F> for Map<MapF> {
    type Fold = MapFold<F, MapF>;

    fn apply(self, fold: F) -> Self::Fold {
        MapFold { fold, mapf: self.mapf }
    }
}
impl<F> Chain for Map<F> {}

impl<F, MapF, A, B> Fold<A> for MapFold<F, MapF>
where
    F: Fold<B>,
    MapF: FnMut(A) -> B,
{
    type Acc = F::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, input: A) -> Step<Self::Acc> {
        self.fold.step(acc, (self.mapf)(input))
    }

    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.fold.done(acc)
    }
}
