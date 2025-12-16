use std::borrow::Borrow;

use crate::{Comp, Fold, Folding, Step, Xform};

#[derive(Debug)]
pub struct Filter<P> {
    pred: P,
}

impl<P> Filter<P> {
    pub(crate) fn new(pred: P) -> Self {
        Filter { pred }
    }
}

#[derive(Debug)]
pub struct FilterSf<Sf, P> {
    sf: Sf,
    pred: P,
}

impl<F, P> FilterSf<F, P> {
    fn new(f: F, pred: P) -> Self {
        FilterSf { sf: f, pred }
    }
}

impl<Sf, P> Xform<Sf> for Filter<P> {
    type Fold = FilterSf<Sf, P>;
    fn xform(self, sf: Sf) -> Self::Fold {
        FilterSf::new(sf, self.pred)
    }
}

impl<Xf> Folding<Xf> {
    pub fn filter<P>(self, pred: P) -> Folding<Comp<Xf, Filter<P>>> {
        self.comp(Filter::new(pred))
    }
}

impl<A, B, Sf, P> Fold<A, B> for FilterSf<Sf, P>
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
        if (self.pred)(input.borrow()) { self.sf.step(acc, input) } else { Step::More(acc) }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.sf.done(acc)
    }
}
