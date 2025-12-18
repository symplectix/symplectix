use std::ops::ControlFlow::*;
use std::rc::Rc;

use crate::{Fold, Fuse, Step};

#[derive(Debug)]
pub struct Par<F, G> {
    f: Fuse<F>,
    g: Fuse<G>,
}

impl<F, G> Par<F, G> {
    pub(crate) fn new(f: F, g: G) -> Self {
        Par { f: Fuse::new(f), g: Fuse::new(g) }
    }
}

impl<A, B, C, F, G> Fold<Rc<A>, (B, C)> for Par<F, G>
where
    F: Fold<Rc<A>, B>,
    G: Fold<Rc<A>, C>,
{
    type Acc = (<F as Fold<Rc<A>, B>>::Acc, <G as Fold<Rc<A>, C>>::Acc);

    fn step(&mut self, acc: Self::Acc, item: Rc<A>) -> Step<Self::Acc> {
        match (self.f.step(acc.0, item.clone()), self.g.step(acc.1, item.clone())) {
            (Continue(a), Continue(b)) => Continue((a, b)),
            (Break(a), Continue(b)) => Continue((a, b)),
            (Continue(a), Break(b)) => Continue((a, b)),
            (Break(a), Break(b)) => Break((a, b)),
        }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.f.done(acc.0), self.g.done(acc.1))
    }
}
