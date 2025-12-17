use std::borrow::Borrow;

use crate::{Fold, Step};

#[derive(Debug)]
pub struct FromFn<F> {
    f: F,
}

impl<F> FromFn<F> {
    pub(crate) fn new(f: F) -> Self {
        FromFn { f }
    }
}

impl<A, B, F> Fold<A, B> for FromFn<F>
where
    F: FnMut(B, &A) -> B,
{
    type Acc = B;

    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        Step::More((self.f)(acc, input.borrow()))
    }

    #[inline]
    fn done(self, acc: B) -> B {
        acc
    }
}
