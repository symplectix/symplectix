use crate::{ControlFlow, Fold, InitialState};

#[derive(Debug)]
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
    type Acc = Rf::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        use std::ops::ControlFlow::Continue;
        if (self.pred)(&item) { self.rf.step(acc, item) } else { Continue(acc) }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.rf.done(acc)
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
