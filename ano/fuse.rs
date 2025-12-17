use std::borrow::Borrow;

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
}

impl<A, B, Rf> Fold<A, B> for Fuse<Rf>
where
    Rf: Fold<A, B>,
{
    type Acc = Rf::Acc;

    fn step<T>(&mut self, acc: <Rf as Fold<A, B>>::Acc, item: &T) -> Step<<Rf as Fold<A, B>>::Acc>
    where
        T: Borrow<A>,
    {
        if self.halt {
            Step::Halt(acc)
        } else {
            match self.rf.step(acc, item) {
                Step::Halt(ret) => {
                    self.halt = true;
                    Step::Halt(ret)
                }
                step => step,
            }
        }
    }

    fn done(self, acc: Self::Acc) -> B {
        self.rf.done(acc)
    }
}
