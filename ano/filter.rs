use std::borrow::Borrow;

use crate::{Fold, Step, Xform};

#[derive(Debug)]
pub struct FilterXf<P> {
    pred: P,
}
impl<P> FilterXf<P> {
    pub(crate) fn new(pred: P) -> Self {
        FilterXf { pred }
    }
}

impl<Sf, P> Xform<Sf> for FilterXf<P> {
    type Fold = Filter<Sf, P>;

    fn apply(self, sf: Sf) -> Self::Fold {
        Filter { sf, pred: self.pred }
    }
}

#[derive(Debug)]
pub struct Filter<Sf, P> {
    sf: Sf,
    pred: P,
}

impl<Sf, P, T> Fold<T> for Filter<Sf, P>
where
    Sf: Fold<T>,
    P: FnMut(&T) -> bool,
{
    type Acc = Sf::Acc;

    #[inline]
    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        if (self.pred)(input.borrow()) { self.sf.step(acc, input) } else { Step::Yield(acc) }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.sf.done(acc)
    }
}
