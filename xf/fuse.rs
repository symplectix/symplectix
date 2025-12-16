use std::borrow::Borrow;

use crate::{Fold, Step};

#[derive(Debug)]
pub(crate) struct Fuse<F> {
    f: F,
    halt: bool,
}

impl<F> Fuse<F> {
    pub(crate) fn new(f: F) -> Self {
        Fuse { f, halt: false }
    }
}

impl<A, B, F> Fold<A, B> for Fuse<F>
where
    F: Fold<A, B>,
{
    type Acc = F::Acc;

    fn step<In>(&mut self, acc: <F as Fold<A, B>>::Acc, input: &In) -> Step<<F as Fold<A, B>>::Acc>
    where
        In: Borrow<A>,
    {
        if self.halt {
            Step::Halt(acc)
        } else {
            match self.f.step(acc, input) {
                Step::Halt(ret) => {
                    self.halt = true;
                    Step::Halt(ret)
                }
                step => step,
            }
        }
    }

    fn done(self, acc: Self::Acc) -> B {
        self.f.done(acc)
    }
}
