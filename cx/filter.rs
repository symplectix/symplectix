use crate::{Adapter, Chain, Fold, Step};

pub fn filter<P>(predicate: P) -> Filter<P> {
    Filter { predicate }
}

pub struct Filter<P> {
    predicate: P,
}

pub struct FilterFold<Rf, P> {
    rf: Rf,
    predicate: P,
}

impl<Rf, P> Adapter<Rf> for Filter<P> {
    type Fold = FilterFold<Rf, P>;

    fn apply(self, rf: Rf) -> Self::Fold {
        FilterFold { rf, predicate: self.predicate }
    }
}
impl<P> Chain for Filter<P> {}

impl<Rf, P, T> Fold<T> for FilterFold<Rf, P>
where
    Rf: Fold<T>,
    P: FnMut(&T) -> bool,
{
    type Acc = Rf::Acc;

    fn step(&mut self, acc: Self::Acc, v: T) -> Step<Self::Acc> {
        if (self.predicate)(&v) { self.rf.step(acc, v) } else { Step::Yield(acc) }
    }
    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.rf.done(acc)
    }
}
