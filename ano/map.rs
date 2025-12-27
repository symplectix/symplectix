use crate::{ControlFlow, InitialState, StepFn};

#[derive(Debug, Clone)]
pub struct Map<Rf, F> {
    rf: Rf,
    mapf: F,
}

impl<Rf, F> Map<Rf, F> {
    pub(crate) fn new(rf: Rf, mapf: F) -> Self {
        Map { rf, mapf }
    }
}

impl<A, B, C, Rf, F> StepFn<A, C> for Map<Rf, F>
where
    Rf: StepFn<B, C>,
    F: FnMut(A) -> B,
{
    type State = Rf::State;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> ControlFlow<Self::State> {
        self.rf.step(acc, (self.mapf)(item))
    }

    #[inline]
    fn complete(self, acc: Self::State) -> C {
        self.rf.complete(acc)
    }
}

impl<St, Sf, F> InitialState<St> for Map<Sf, F>
where
    Sf: InitialState<St>,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> St {
        self.rf.initial_state(size_hint)
    }
}
