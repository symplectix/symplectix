use std::marker::PhantomData;

use crate::{ControlFlow, InitialState, StepFn};

#[derive(Debug, Clone)]
pub struct Completing<Rf, B, F> {
    rf: Rf,
    _b: PhantomData<B>,
    completing: F,
}

impl<Rf, B, F> Completing<Rf, B, F> {
    pub(crate) fn new(rf: Rf, completing: F) -> Self {
        Completing { _b: PhantomData, rf, completing }
    }
}

impl<A, B, C, Rf, F> StepFn<A, C> for Completing<Rf, B, F>
where
    Rf: StepFn<A, B>,
    F: FnMut(B) -> C,
{
    type State = Rf::State;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> ControlFlow<Self::State> {
        self.rf.step(acc, item)
    }

    #[inline]
    fn complete(mut self, acc: Self::State) -> C {
        (self.completing)(self.rf.complete(acc))
    }
}

impl<T, B, Rf, F> InitialState<T> for Completing<Rf, B, F>
where
    Rf: InitialState<T>,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T {
        self.rf.initial_state(size_hint)
    }
}
