use std::borrow::Borrow;

use crate::{Comp, Fold, Folding, Step, Xform};

#[derive(Debug)]
pub struct Take {
    count: usize,
}
impl Take {
    fn new(count: usize) -> Self {
        Take { count }
    }
}

#[derive(Debug)]
pub struct TakeSf<F> {
    f: F,
    count: usize,
}
impl<F> TakeSf<F> {
    fn new(f: F, count: usize) -> Self {
        TakeSf { f, count }
    }
}

impl<Sf> Xform<Sf> for Take {
    type Fold = TakeSf<Sf>;
    fn apply(self, sf: Sf) -> Self::Fold {
        TakeSf::new(sf, self.count)
    }
}

impl<Xf> Folding<Xf> {
    pub fn take(self, count: usize) -> Folding<Comp<Xf, Take>> {
        self.comp(Take::new(count))
    }
}
pub fn take(count: usize) -> Folding<Take> {
    Folding::new(Take::new(count))
}

impl<A, B, F> Fold<A, B> for TakeSf<F>
where
    F: Fold<A, B>,
{
    type Acc = F::Acc;
    #[inline]
    fn step<In>(&mut self, acc: Self::Acc, input: &In) -> Step<Self::Acc>
    where
        In: Borrow<A>,
    {
        match self.count {
            0 => Step::Halt(acc),
            1 => {
                self.count = 0;
                match self.f.step(acc, input) {
                    Step::More(a) => Step::Halt(a),
                    Step::Halt(a) => Step::Halt(a),
                }
            }
            _ => {
                self.count -= 1;
                self.f.step(acc, input)
            }
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> B {
        self.f.done(acc)
    }
}
