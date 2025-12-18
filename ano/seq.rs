use std::ops::ControlFlow::*;

use crate::{ControlFlow, Fold, Fuse};

#[derive(Debug)]
pub struct Seq<F, G> {
    f: Fuse<F>,
    g: Fuse<G>,
}

impl<F, G> Seq<F, G> {
    pub(crate) fn new(f: F, g: G) -> Self {
        Seq { f: Fuse::new(f), g: Fuse::new(g) }
    }
}

impl<A, B, C, F, G> Fold<A, (B, C)> for Seq<F, G>
where
    F: Fold<A, B>,
    G: Fold<A, C>,
{
    type Acc = (<F as Fold<A, B>>::Acc, <G as Fold<A, C>>::Acc);

    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        if !self.f.halted() {
            return match self.f.step(acc.0, item) {
                Continue(a) => Continue((a, acc.1)),
                Break(a) => Continue((a, acc.1)),
            };
        }
        if !self.g.halted() {
            return match self.g.step(acc.1, item) {
                Continue(b) => Continue((acc.0, b)),
                Break(b) => Break((acc.0, b)),
            };
        }
        Break(acc)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.f.done(acc.0), self.g.done(acc.1))
    }
}
