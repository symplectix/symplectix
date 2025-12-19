use std::marker::PhantomData;

use crate::{ControlFlow, Fold};

impl<A, B, F> Fold<A, B> for F
where
    F: FnMut(B, A) -> B,
{
    type Acc = B;

    #[inline]
    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        use std::ops::ControlFlow::Continue;
        Continue((self)(acc, item))
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        acc
    }
}

#[derive(Debug)]
pub struct Completing<Rf, B, F> {
    rf: Rf,
    _b: PhantomData<B>,
    completing: F,
}

impl<Rf, B, F> Completing<Rf, B, F> {
    pub(crate) fn new(rf: Rf, completing: F) -> Self {
        Completing { _b: PhantomData, rf, completing }
    }
}

impl<A, B, C, Rf, F> Fold<A, C> for Completing<Rf, B, F>
where
    Rf: Fold<A, B>,
    F: FnMut(B) -> C,
{
    type Acc = Rf::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        self.rf.step(acc, item)
    }

    #[inline]
    fn done(mut self, acc: Self::Acc) -> C {
        (self.completing)(self.rf.done(acc))
    }
}
