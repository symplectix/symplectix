use crate::{Adapter, Compose, Reducer, Step};

pub fn map<F>(f: F) -> Map<F> {
    Map { mapper: f }
}

pub struct Map<F> {
    mapper: F,
}

pub struct MapReducer<Rf, F> {
    rf: Rf,
    mapper: F,
}

impl<Rf, F> Adapter<Rf> for Map<F> {
    type Reducer = MapReducer<Rf, F>;

    fn apply(self, rf: Rf) -> Self::Reducer {
        MapReducer { rf, mapper: self.mapper }
    }
}
impl<F> Compose for Map<F> {}

impl<Rf, F, A, B> Reducer<A> for MapReducer<Rf, F>
where
    Rf: Reducer<B>,
    F: FnMut(A) -> B,
{
    type Acc = Rf::Acc;

    fn step(&mut self, acc: Self::Acc, v: A) -> Step<Self::Acc> {
        self.rf.step(acc, (self.mapper)(v))
    }

    fn done(&mut self, acc: Self::Acc) -> Self::Acc {
        self.rf.done(acc)
    }
}
