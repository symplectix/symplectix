use crate::{ControlFlow, Fold, InitialState};

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

impl<A, B, C, Rf, F> Fold<A, C> for Map<Rf, F>
where
    Rf: Fold<B, C>,
    F: FnMut(A) -> B,
{
    type State = Rf::State;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> ControlFlow<Self::State> {
        self.rf.step(acc, (self.mapf)(item))
    }

    #[inline]
    fn done(self, acc: Self::State) -> C {
        self.rf.done(acc)
    }
}

impl<T, Rf, F> InitialState<T> for Map<Rf, F>
where
    Rf: InitialState<T>,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T {
        self.rf.initial_state(size_hint)
    }
}
