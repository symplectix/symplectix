use std::borrow::Borrow;

use crate::{Fold, Step};

#[derive(Debug)]
pub struct Filter<Rf, P> {
    rf: Rf,
    pred: P,
}

impl<Rf, P> Filter<Rf, P> {
    pub(crate) fn new(rf: Rf, pred: P) -> Self {
        Filter { rf, pred }
    }
}

impl<A, B, Rf, P> Fold<A, B> for Filter<Rf, P>
where
    Rf: Fold<A, B>,
    P: FnMut(&A) -> bool,
{
    type Acc = Rf::Acc;

    #[inline]
    fn step<T>(&mut self, acc: Self::Acc, item: &T) -> Step<Self::Acc>
    where
        T: Borrow<A>,
    {
        if (self.pred)(item.borrow()) { self.rf.step(acc, item) } else { Step::More(acc) }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.rf.done(acc)
    }
}
