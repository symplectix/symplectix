use crate::{Chain, Step, StepFn, Xform};

pub fn filter<P>(predicate: P) -> Filter<P> {
    Filter { pred: predicate }
}

pub struct Filter<P> {
    pred: P,
}

pub struct FilterStep<Sf, P> {
    sf: Sf,
    pred: P,
}

impl<Sf, P> Xform<Sf> for Filter<P> {
    type StepFn = FilterStep<Sf, P>;

    fn apply(self, step_fn: Sf) -> Self::StepFn {
        FilterStep { sf: step_fn, pred: self.pred }
    }
}
impl<P> Chain for Filter<P> {}

impl<Sf, P, T> StepFn<T> for FilterStep<Sf, P>
where
    Sf: StepFn<T>,
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
