use std::borrow::Borrow;

use crate::{Fold, Step};

#[derive(Debug)]
pub struct Either<F, G>(F, G);

impl<F, G> Either<F, G> {
    pub(crate) fn new(f: F, g: G) -> Self {
        Either(f, g)
    }
}

impl<A, B, C, F, G> Fold<A, (B, C)> for Either<F, G>
where
    F: Fold<A, B>,
    G: Fold<A, C>,
{
    type Acc = (<F as Fold<A, B>>::Acc, <G as Fold<A, C>>::Acc);

    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::More(a), Step::More(b)) => Step::More((a, b)),
            (Step::Halt(a), Step::More(b)) => Step::Halt((a, b)),
            (Step::More(a), Step::Halt(b)) => Step::Halt((a, b)),
            (Step::Halt(a), Step::Halt(b)) => Step::Halt((a, b)),
        }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}
