use std::ops::ControlFlow::*;

use crate::{Fold, InitialState, Step};

#[derive(Debug, Clone)]
pub struct Take<Rf> {
    rf: Rf,
    count: usize,
}

impl<Rf> Take<Rf> {
    pub(crate) fn new(rf: Rf, count: usize) -> Self {
        Take { rf, count }
    }
}

impl<A, B, Rf> Fold<A, B> for Take<Rf>
where
    Rf: Fold<A, B>,
{
    type State = Rf::State;

    #[inline]
    fn step(&mut self, acc: Self::State, item: A) -> Step<Self::State> {
        match self.count {
            0 => Break(acc),
            1 => {
                self.count = 0;
                match self.rf.step(acc, item) {
                    Continue(a) => Break(a),
                    Break(a) => Break(a),
                }
            }
            _ => {
                self.count -= 1;
                self.rf.step(acc, item)
            }
        }
    }

    #[inline]
    fn complete(self, acc: Self::State) -> B {
        self.rf.complete(acc)
    }
}

impl<T, Rf> InitialState<T> for Take<Rf>
where
    Rf: InitialState<T>,
{
    #[inline]
    fn initial_state(&self, _size_hint: (usize, Option<usize>)) -> T {
        self.rf.initial_state((0, Some(self.count)))
    }
}
