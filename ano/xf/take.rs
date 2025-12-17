use super::{Comp, Folding, Xform};

#[derive(Debug)]
pub struct TakeXf {
    count: usize,
}

impl TakeXf {
    pub(crate) fn new(count: usize) -> Self {
        TakeXf { count }
    }
}

impl<Sf> Xform<Sf> for TakeXf {
    type Fold = crate::Take<Sf>;
    fn xform(self, sf: Sf) -> Self::Fold {
        crate::Take::new(sf, self.count)
    }
}

impl<Xf> Folding<Xf> {
    pub fn take(self, count: usize) -> Folding<Comp<Xf, TakeXf>> {
        self.comp(TakeXf::new(count))
    }
}
