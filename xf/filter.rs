use std::borrow::Borrow;

use crate::{Comp, Fold, Folding, Step, Xform};

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
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        if (self.pred)(input.borrow()) { self.rf.step(acc, input) } else { Step::More(acc) }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.rf.done(acc)
    }
}

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
    fn xform(self, sf: Sf) -> Self::Fold {
        Filter::new(sf, self.pred)
    }
}

impl<Xf> Folding<Xf> {
    pub fn filter<P>(self, pred: P) -> Folding<Comp<Xf, FilterXf<P>>> {
        self.comp(FilterXf::new(pred))
    }
}
