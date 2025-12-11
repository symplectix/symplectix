use crate::{Adapter, Chain, Fold, Step};

pub fn filter<P>(predicate: P) -> Filter<P> {
    Filter { pred: predicate }
}

pub struct Filter<P> {
    pred: P,
}

pub struct FilterFold<F, P> {
    fold: F,
    pred: P,
}

impl<F, P> Adapter<F> for Filter<P> {
    type Fold = FilterFold<F, P>;

    fn apply(self, fold: F) -> Self::Fold {
        FilterFold { fold, pred: self.pred }
    }
}
impl<P> Chain for Filter<P> {}

impl<F, P, T> Fold<T> for FilterFold<F, P>
where
    F: Fold<T>,
    P: FnMut(&T) -> bool,
{
    type Acc = F::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, input: T) -> Step<Self::Acc> {
        if (self.pred)(&input) { self.fold.step(acc, input) } else { Step::Yield(acc) }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.fold.done(acc)
    }
}
