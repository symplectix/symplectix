use std::rc::Rc;

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
    F: Fold<Rc<A>, B>,
    G: Fold<Rc<A>, C>,
{
    type Acc = (<F as Fold<Rc<A>, B>>::Acc, <G as Fold<Rc<A>, C>>::Acc);

    fn step(&mut self, acc: Self::Acc, item: A) -> Step<Self::Acc> {
        let a = Rc::new(item);
        let b = a.clone();
        match (self.f.step(acc.0, a), self.g.step(acc.1, b)) {
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
