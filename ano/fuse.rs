use std::ops::ControlFlow::Break;

use crate::{Fold, Step};

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

impl<A, B, Rf> Fold<A, B> for Fuse<Rf>
where
    Rf: Fold<A, B>,
{
    type Acc = Rf::Acc;

    fn step(&mut self, acc: <Rf as Fold<A, B>>::Acc, item: A) -> Step<<Rf as Fold<A, B>>::Acc> {
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
