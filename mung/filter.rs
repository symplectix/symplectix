use crate::{Step, Fold, XFn};

#[derive(Debug)]
pub struct Filter<P> {
    pred: P,
}
impl<P> Filter<P> {
    pub(crate) fn new(pred: P) -> Self {
        Filter { pred }
    }
}

pub struct FilterStep<Sf, P> {
    sf: Sf,
    pred: P,
}

impl<Sf, P> XFn<Sf> for Filter<P> {
    type StepFn = FilterStep<Sf, P>;

    fn apply(self, step_fn: Sf) -> Self::StepFn {
        FilterStep { sf: step_fn, pred: self.pred }
    }
}

impl<Sf, P, T> Fold<T> for FilterStep<Sf, P>
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
