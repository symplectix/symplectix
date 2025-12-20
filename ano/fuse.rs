use std::ops::ControlFlow::Break;

use crate::{ControlFlow, Fold, InitialState};

#[derive(Debug)]
pub(crate) struct Fuse<Rf> {
    rf: Rf,
    halt: bool,
}

impl<Rf> Fuse<Rf> {
    pub(crate) fn new(rf: Rf) -> Self {
        Fuse { rf, halt: false }
    }

    pub(crate) fn halted(&self) -> bool {
        self.halt
    }
}

impl<T, Rf> InitialState<T> for Fuse<Rf>
where
    Rf: InitialState<T>,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T {
        self.rf.initial_state(size_hint)
    }
}

impl<A, B, Rf> Fold<A, B> for Fuse<Rf>
where
    Rf: Fold<A, B>,
{
    type Acc = Rf::Acc;

    fn step(&mut self, acc: <Rf as Fold<A, B>>::Acc, item: A) -> ControlFlow<<Rf as Fold<A, B>>::Acc> {
        if self.halt {
            Break(acc)
        } else {
            let step = self.rf.step(acc, item);
            self.halt = step.is_break();
            step
        }
    }

    fn done(self, acc: Self::Acc) -> B {
        self.rf.done(acc)
    }
}
