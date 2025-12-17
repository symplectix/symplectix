use super::{Comp, Folding, Xform};

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
    type Fold = crate::Filter<Sf, P>;
    fn xform(self, sf: Sf) -> Self::Fold {
        crate::Filter::new(sf, self.pred)
    }
}

impl<Xf> Folding<Xf> {
    pub fn filter<P>(self, pred: P) -> Folding<Comp<Xf, FilterXf<P>>> {
        self.comp(FilterXf::new(pred))
    }
}
