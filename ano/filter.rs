use crate::{ControlFlow, InitialState, StepFn};

#[derive(Debug, Clone)]
pub struct Filter<Rf, P> {
    rf: Rf,
    pred: P,
}

impl<Rf, P> Filter<Rf, P> {
    pub(crate) fn new(rf: Rf, pred: P) -> Self {
        Filter { rf, pred }
    }
}

impl<A, B, Rf, P> StepFn<A, B> for Filter<Rf, P>
where
    Rf: StepFn<A, B>,
    P: FnMut(&A) -> bool,
{
    type State = Rf::State;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> ControlFlow<Self::State> {
        use std::ops::ControlFlow::Continue;
        if (self.pred)(&item) { self.rf.step(acc, item) } else { Continue(acc) }
    }

    #[inline]
    fn complete(self, acc: Self::State) -> B {
        self.rf.complete(acc)
    }
}

impl<St, Sf, P> InitialState<St> for Filter<Sf, P>
where
    Sf: InitialState<St>,
{
    #[inline]
    fn initial_state(&self, (_lo, hi): (usize, Option<usize>)) -> St {
        self.rf.initial_state((0, hi))
    }
}
