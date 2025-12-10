use crate::{Adapter, Compose, Reducer, Step};

pub fn filter<P>(predicate: P) -> Filter<P> {
    Filter { predicate }
}

pub struct Filter<P> {
    predicate: P,
}

pub struct FilterReducer<Rf, P> {
    rf: Rf,
    predicate: P,
}

impl<Rf, P> Adapter<Rf> for Filter<P> {
    type Reducer = FilterReducer<Rf, P>;

    fn apply(self, rf: Rf) -> Self::Reducer {
        FilterReducer { rf, predicate: self.predicate }
    }
}
impl<P> Compose for Filter<P> {}

impl<Rf, P, T> Reducer<T> for FilterReducer<Rf, P>
where
    Rf: Reducer<T>,
    P: FnMut(&T) -> bool,
{
    type Acc = Rf::Acc;

    fn step(&mut self, acc: Self::Acc, v: T) -> Step<Self::Acc> {
        if (self.predicate)(&v) { self.rf.step(acc, v) } else { Step::Continue(acc) }
    }
    fn done(&mut self, acc: Self::Acc) -> Self::Acc {
        self.rf.done(acc)
    }
}
