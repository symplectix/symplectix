use crate::{ControlFlow, Fold};

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
    F: FnMut(B, A) -> B,
{
    type Acc = B;

    #[inline]
    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        use std::ops::ControlFlow::Continue;
        Continue((self.f)(acc, item))
    }

    #[inline]
    fn done(self, acc: B) -> B {
        acc
    }
}
