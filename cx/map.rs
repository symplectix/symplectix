use crate::{Chain, Adapter, Fold, Step};

pub fn map<F>(f: F) -> Map<F> {
    Map { mapper: f }
}

pub struct Map<F> {
    mapper: F,
}

pub struct MapFold<Rf, F> {
    rf: Rf,
    mapper: F,
}

impl<Rf, F> Adapter<Rf> for Map<F> {
    type Fold = MapFold<Rf, F>;

    fn apply(self, rf: Rf) -> Self::Fold {
        MapFold { rf, mapper: self.mapper }
    }
}
impl<F> Chain for Map<F> {}

impl<Rf, F, A, B> Fold<A> for MapFold<Rf, F>
where
    Rf: Fold<B>,
    F: FnMut(A) -> B,
{
    type Acc = Rf::Acc;

    fn step(&mut self, acc: Self::Acc, input: A) -> Step<Self::Acc> {
        self.rf.step(acc, (self.mapper)(input))
    }

    fn done(&mut self, acc: Self::Acc) -> Self::Acc {
        self.rf.done(acc)
    }
}
