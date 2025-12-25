use crate::{Fold, InitialState, Step};

#[derive(Debug, Clone)]
pub struct WithInitialState<Rf, F> {
    rf: Rf,
    using: F,
}

impl<Rf, F> WithInitialState<Rf, F> {
    pub(crate) fn new(rf: Rf, using: F) -> Self {
        WithInitialState { rf, using }
    }
}

impl<A, B, Rf, F> Fold<A, B> for WithInitialState<Rf, F>
where
    Rf: Fold<A, B>,
{
    type State = Rf::State;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> Step<Self::State> {
        self.rf.step(acc, item)
    }

    #[inline]
    fn complete(self, acc: Self::State) -> B {
        self.rf.complete(acc)
    }
}

impl<T, Rf, F> InitialState<T> for WithInitialState<Rf, F>
where
    F: Fn((usize, Option<usize>)) -> T,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T {
        (self.using)(size_hint)
    }
}
