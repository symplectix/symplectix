use std::borrow::Borrow;

use crate::{Fold, Step};

#[derive(Debug)]
pub struct Either<F, G> {
    f: F,
    g: G,
}

impl<F, G> Either<F, G> {
    pub(crate) fn new(f: F, g: G) -> Self {
        Either { f, g }
    }
}

impl<A, B, C, F, G> Fold<A, (B, C)> for Either<F, G>
where
    F: Fold<A, B>,
    G: Fold<A, C>,
{
    type Acc = (<F as Fold<A, B>>::Acc, <G as Fold<A, C>>::Acc);

    fn step<T>(&mut self, acc: Self::Acc, item: &T) -> Step<Self::Acc>
    where
        T: Borrow<A>,
    {
        match (self.f.step(acc.0, item), self.g.step(acc.1, item)) {
            (Step::More(a), Step::More(b)) => Step::More((a, b)),
            (Step::Halt(a), Step::More(b)) => Step::Halt((a, b)),
            (Step::More(a), Step::Halt(b)) => Step::Halt((a, b)),
            (Step::Halt(a), Step::Halt(b)) => Step::Halt((a, b)),
        }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.f.done(acc.0), self.g.done(acc.1))
    }
}
