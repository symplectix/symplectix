use std::rc::Rc;
use std::{marker::PhantomData, ops::ControlFlow::*};

use crate::{ControlFlow, Fold, Fuse};

#[derive(Debug)]
pub struct Par<'a, F, G> {
    _a: PhantomData<&'a ()>,
    f: Fuse<F>,
    g: Fuse<G>,
}

impl<'a, F, G> Par<'a, F, G> {
    pub(crate) fn new(f: F, g: G) -> Self {
        Par { _a: PhantomData, f: Fuse::new(f), g: Fuse::new(g) }
    }
}

impl<'a, A, B, C, F, G> Fold<&'a A, (B, C)> for Par<'a, F, G>
where
    F: Fold<&'a A, B>,
    G: Fold<&'a A, C>,
{
    type Acc = (<F as Fold<&'a A, B>>::Acc, <G as Fold<&'a A, C>>::Acc);

    fn step(&mut self, acc: Self::Acc, item: &'a A) -> ControlFlow<Self::Acc> {
        match (self.f.step(acc.0, item), self.g.step(acc.1, item)) {
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

impl<A, B, C, F, G> Fold<Rc<A>, (B, C)> for Par<'_, F, G>
where
    F: Fold<Rc<A>, B>,
    G: Fold<Rc<A>, C>,
{
    type Acc = (<F as Fold<Rc<A>, B>>::Acc, <G as Fold<Rc<A>, C>>::Acc);

    fn step(&mut self, acc: Self::Acc, item: Rc<A>) -> ControlFlow<Self::Acc> {
        match (self.f.step(acc.0, item.clone()), self.g.step(acc.1, item)) {
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
