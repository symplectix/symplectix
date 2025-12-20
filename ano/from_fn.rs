use std::marker::PhantomData;

use crate::{ControlFlow, Fold, InitialState};

impl<A, B, F> Fold<A, B> for F
where
    F: FnMut(B, A) -> ControlFlow<B>,
{
    type Acc = B;

    #[inline]
    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        (self)(acc, item)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        acc
    }
}

#[derive(Debug)]
pub struct Using<Rf, F> {
    rf: Rf,
    using: F,
}

impl<Rf, F> Using<Rf, F> {
    pub(crate) fn new(rf: Rf, using: F) -> Self {
        Using { rf, using }
    }
}

impl<A, B, Rf, F> Fold<A, B> for Using<Rf, F>
where
    Rf: Fold<A, B>,
{
    type Acc = Rf::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        self.rf.step(acc, item)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.rf.done(acc)
    }
}

impl<T, Rf, F> InitialState<T> for Using<Rf, F>
where
    F: Fn((usize, Option<usize>)) -> T,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T {
        (self.using)(size_hint)
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

impl<T, B, Rf, F> InitialState<T> for Completing<Rf, B, F>
where
    Rf: InitialState<T>,
{
    #[inline]
    fn initial_state(&self, size_hint: (usize, Option<usize>)) -> T {
        self.rf.initial_state(size_hint)
    }
}
