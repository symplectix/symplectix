use crate::{Chain, Step, StepFn, Xform};

#[derive(Debug)]
pub struct Map<F> {
    mapf: F,
}
impl<F> Map<F> {
    pub(crate) fn new(f: F) -> Self {
        Map { mapf: f }
    }
}

pub struct MapStep<Sf, MapF> {
    sf: Sf,
    mapf: MapF,
}

impl<Sf, F> Xform<Sf> for Map<F> {
    type StepFn = MapStep<Sf, F>;

    fn apply(self, step_fn: Sf) -> Self::StepFn {
        MapStep { sf: step_fn, mapf: self.mapf }
    }
}
impl<F> Chain for Map<F> {}

impl<Sf, F, A, B> StepFn<A> for MapStep<Sf, F>
where
    Sf: StepFn<B>,
    F: FnMut(A) -> B,
{
    type Acc = Sf::Acc;

    #[inline]
    fn step(&mut self, acc: Self::Acc, input: A) -> Step<Self::Acc> {
        self.sf.step(acc, (self.mapf)(input))
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.sf.done(acc)
    }
}
