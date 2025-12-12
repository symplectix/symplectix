use crate::{Fold, Step, XformFn};

#[derive(Debug)]
pub struct Filter<P> {
    pred: P,
}
impl<P> Filter<P> {
    pub(crate) fn new(pred: P) -> Self {
        Filter { pred }
    }
}

impl<Sf, P> XformFn<Sf> for Filter<P> {
    type Fold = FilterFold<Sf, P>;

    fn apply(self, sf: Sf) -> Self::Fold {
        FilterFold { sf, pred: self.pred }
    }
}

#[derive(Debug)]
pub struct FilterFold<Sf, P> {
    sf: Sf,
    pred: P,
}

impl<Sf, P, T> Fold<T> for FilterFold<Sf, P>
where
    Sf: Fold<T>,
    P: FnMut(&T) -> bool,
{
    type Acc = Sf::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, input: T) -> Step<Self::Acc> {
        if (self.pred)(&input) { self.sf.step(acc, input) } else { Step::Yield(acc) }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.sf.done(acc)
    }
}
