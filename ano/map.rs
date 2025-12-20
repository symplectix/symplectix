use crate::{ControlFlow, Fold, Init};

#[derive(Debug)]
pub struct Map<Rf, F> {
    rf: Rf,
    mapf: F,
}

impl<Rf, F> Map<Rf, F> {
    pub(crate) fn new(rf: Rf, mapf: F) -> Self {
        Map { rf, mapf }
    }
}

impl<A, B, C, Rf, F> Fold<A, C> for Map<Rf, F>
where
    Rf: Fold<B, C>,
    F: FnMut(A) -> B,
{
    type Acc = Rf::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, item: A) -> ControlFlow<Self::Acc> {
        self.rf.step(acc, (self.mapf)(item))
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> C {
        self.rf.done(acc)
    }
}

impl<T, Rf, F> Init<T> for Map<Rf, F>
where
    Rf: Init<T>,
{
    #[inline]
    fn init(&self, size_hint: (usize, Option<usize>)) -> T {
        self.rf.init(size_hint)
    }
}
