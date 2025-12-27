use crate::{ControlFlow, InitialState, StepFn};

#[derive(Debug, Clone)]
pub struct Beginning<Rf, F> {
    rf: Rf,
    begin: F,
}

impl<Rf, F> Beginning<Rf, F> {
    pub(crate) fn new(rf: Rf, begin: F) -> Self {
        Beginning { rf, begin }
    }
}

impl<A, B, Rf, F> StepFn<A, B> for Beginning<Rf, F>
where
    Rf: StepFn<A, B>,
{
    type State = Rf::State;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> ControlFlow<Self::State> {
        self.rf.step(acc, item)
    }

    #[inline]
    fn complete(self, acc: Self::State) -> B {
        self.rf.complete(acc)
    }
}

impl<T, Rf, F> InitialState<T> for Beginning<Rf, F>
where
    F: Fn((usize, Option<usize>)) -> T,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T {
        (self.begin)(size_hint)
    }
}
