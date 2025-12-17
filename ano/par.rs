use std::borrow::Borrow;

use crate::{Fold, Fuse, Step};

#[derive(Debug)]
pub struct Par<F, G>(Fuse<F>, Fuse<G>);

impl<F, G> Par<F, G> {
    pub(crate) fn new(f: F, g: G) -> Self {
        Par(Fuse::new(f), Fuse::new(g))
    }
}

impl<A, B, C, F, G> Fold<A, (B, C)> for Par<F, G>
where
    F: Fold<A, B>,
    G: Fold<A, C>,
{
    type Acc = (<F as Fold<A, B>>::Acc, <G as Fold<A, C>>::Acc);
    fn step<T>(&mut self, acc: Self::Acc, input: &T) -> Step<Self::Acc>
    where
        T: Borrow<A>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::More(a), Step::More(b)) => Step::More((a, b)),
            (Step::Halt(a), Step::More(b)) => Step::More((a, b)),
            (Step::More(a), Step::Halt(b)) => Step::More((a, b)),
            (Step::Halt(a), Step::Halt(b)) => Step::Halt((a, b)),
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> (B, C) {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}
