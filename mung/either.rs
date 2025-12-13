use std::borrow::Borrow;

use crate::{Fold, Step};

#[derive(Debug)]
pub struct Either<A, B>(pub(crate) A, pub(crate) B);

impl<T, A, B> Fold<T> for Either<A, B>
where
    A: Fold<T>,
    B: Fold<T>,
{
    type Acc = (<A as Fold<T>>::Acc, <B as Fold<T>>::Acc);

    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::Yield(a), Step::Yield(b)) => Step::Yield((a, b)),
            (Step::Break(a), Step::Yield(b)) => Step::Break((a, b)),
            (Step::Yield(a), Step::Break(b)) => Step::Break((a, b)),
            (Step::Break(a), Step::Break(b)) => Step::Break((a, b)),
        }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}
