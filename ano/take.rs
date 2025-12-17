use std::borrow::Borrow;

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
    fn step<T>(&mut self, acc: Self::Acc, item: &T) -> Step<Self::Acc>
    where
        T: Borrow<A>,
    {
        match self.count {
            0 => Step::Halt(acc),
            1 => {
                self.count = 0;
                match self.rf.step(acc, item) {
                    Step::More(a) => Step::Halt(a),
                    Step::Halt(a) => Step::Halt(a),
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
