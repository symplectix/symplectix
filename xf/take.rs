use std::borrow::Borrow;

use crate::{Comp, Fold, Folding, Step, Xform};

#[derive(Debug)]
pub struct TakeXf {
    count: usize,
}
impl TakeXf {
    pub(crate) fn new(count: usize) -> Self {
        TakeXf { count }
    }
}

#[derive(Debug)]
pub struct Take<F> {
    f: F,
    count: usize,
}
impl<F> Take<F> {
    pub(crate) fn new(f: F, count: usize) -> Self {
        Take { f, count }
    }
}

impl<Sf> Xform<Sf> for TakeXf {
    type Fold = Take<Sf>;
    fn xform(self, sf: Sf) -> Self::Fold {
        Take::new(sf, self.count)
    }
}

impl<Xf> Folding<Xf> {
    pub fn take(self, count: usize) -> Folding<Comp<Xf, TakeXf>> {
        self.comp(TakeXf::new(count))
    }
}

impl<A, B, F> Fold<A, B> for Take<F>
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
