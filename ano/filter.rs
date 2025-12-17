use std::borrow::Borrow;

use crate::{Fold, Step};

#[derive(Debug)]
pub struct Filter<Rf, P> {
    rf: Rf,
    pred: P,
}

impl<F, P> Filter<F, P> {
    pub(crate) fn new(f: F, pred: P) -> Self {
        Filter { rf: f, pred }
    }
}

impl<A, B, Sf, P> Fold<A, B> for Filter<Sf, P>
where
    Sf: Fold<A, B>,
    P: FnMut(&A) -> bool,
{
    type Acc = Sf::Acc;

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
