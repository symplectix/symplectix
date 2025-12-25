use crate::{Fold, InitialState, Step};

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

impl<A, B, Rf, P> Fold<A, B> for Filter<Rf, P>
where
    Rf: Fold<A, B>,
    P: FnMut(&A) -> bool,
{
    type State = Rf::State;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> Step<Self::State> {
        use std::ops::ControlFlow::Continue;
        if (self.pred)(&item) { self.rf.step(acc, item) } else { Continue(acc) }
    }

    #[inline]
    fn complete(self, acc: Self::State) -> B {
        self.rf.complete(acc)
    }
}

impl<T, Rf, P> InitialState<T> for Filter<Rf, P>
where
    Rf: InitialState<T>,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T {
        self.rf.initial_state(size_hint)
    }
}
