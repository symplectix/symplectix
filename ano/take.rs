use std::ops::ControlFlow::*;

use crate::{Fold, Step};

#[derive(Debug)]
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
    type Acc = Rf::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, item: A) -> Step<Self::Acc> {
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
    fn done(self, acc: Self::Acc) -> B {
        self.rf.done(acc)
    }
}
