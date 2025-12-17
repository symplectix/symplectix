use std::borrow::Borrow;

use crate::{Fold, Step};

#[derive(Debug)]
pub struct Map<Rf, F> {
    rf: Rf,
    mapf: F,
}

impl<Rf, F> Map<Rf, F> {
    pub(crate) fn new(rf: Rf, mapf: F) -> Self {
        Map { rf, mapf }
    }
}

impl<A, B, C, Rf, F> Fold<A, C> for Map<Rf, F>
where
    Rf: Fold<B, C>,
    F: FnMut(&A) -> B,
{
    type Acc = Rf::Acc;

    #[inline]
    fn step<T>(&mut self, acc: Self::Acc, item: &T) -> Step<Self::Acc>
    where
        T: Borrow<A>,
    {
        let mapped = (self.mapf)(item.borrow());
        self.rf.step(acc, &mapped)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> C {
        self.rf.done(acc)
    }
}
