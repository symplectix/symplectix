use std::borrow::Borrow;

pub trait Fold<In, Out>: Sized {
    /// Runs just a one step of folding.
    fn step<T>(&mut self, input: &T) -> Step
    where
        T: Borrow<In>;

    /// Invoked when folding is complete.
    ///
    /// You must call `done` exactly once.
    fn done(self) -> Out;

    fn fold<It, T>(mut self, iterable: It) -> Out
    where
        It: IntoIterator<Item = T>,
        T: Borrow<In>,
    {
        for item in iterable.into_iter() {
            match self.step(&item) {
                Step::Yield => {}
                Step::Break => {
                    break;
                }
            }
        }
        self.done()
    }

    fn either<That>(self, that: That) -> Either<Self, That>
    where
        Self: Sized,
    {
        Either(self, that)
    }
}

/// The result of [Fold.step].
#[derive(Debug, Copy, Clone)]
pub enum Step {
    /// Keep folding.
    Yield,
    /// Stop folding.
    Break,
}

#[derive(Debug)]
pub struct Map<Sf, F> {
    sf: Sf,
    mapf: F,
}
impl<Sf, F> Map<Sf, F> {
    pub(crate) fn new(sf: Sf, mapf: F) -> Self {
        Map { sf, mapf }
    }
}
impl<Sf, F, A, In, Out> Fold<In, Out> for Map<Sf, F>
where
    Sf: Fold<A, Out>,
    F: FnMut(&In) -> A,
{
    #[inline]
    fn step<T>(&mut self, input: &T) -> Step
    where
        T: Borrow<In>,
    {
        let mapped = (self.mapf)(input.borrow());
        self.sf.step(&mapped)
    }

    #[inline]
    fn done(self) -> Out {
        self.sf.done()
    }
}

#[derive(Debug)]
pub struct Filter<Sf, P> {
    sf: Sf,
    pred: P,
}
impl<Sf, P> Filter<Sf, P> {
    pub(crate) fn new(sf: Sf, pred: P) -> Self {
        Filter { sf, pred }
    }
}
impl<Sf, P, In, Out> Fold<In, Out> for Filter<Sf, P>
where
    Sf: Fold<In, Out>,
    P: FnMut(&In) -> bool,
{
    #[inline]
    fn step<T>(&mut self, input: &T) -> Step
    where
        T: Borrow<In>,
    {
        if (self.pred)(input.borrow()) { self.sf.step(input) } else { Step::Yield }
    }

    #[inline]
    fn done(self) -> Out {
        self.sf.done()
    }
}

#[derive(Debug)]
pub struct Take<Sf> {
    sf: Sf,
    count: usize,
}
impl<Sf> Take<Sf> {
    pub(crate) fn new(sf: Sf, count: usize) -> Self {
        Take { sf, count }
    }
}
impl<Sf, In, Out> Fold<In, Out> for Take<Sf>
where
    Sf: Fold<In, Out>,
{
    #[inline]
    fn step<T>(&mut self, input: &T) -> Step
    where
        T: Borrow<In>,
    {
        match self.count {
            0 => Step::Break,
            1 => {
                self.count = 0;
                let _step = self.sf.step(input);
                Step::Break
            }
            _ => {
                self.count -= 1;
                self.sf.step(input)
            }
        }
    }

    #[inline]
    fn done(self) -> Out {
        self.sf.done()
    }
}

#[derive(Debug)]
pub struct Either<A, B>(pub(crate) A, pub(crate) B);
impl<In, O1, O2, A, B> Fold<In, (O1, O2)> for Either<A, B>
where
    A: Fold<In, O1>,
    B: Fold<In, O2>,
{
    fn step<T>(&mut self, input: &T) -> Step
    where
        T: Borrow<In>,
    {
        match (self.0.step(input), self.1.step(input)) {
            (Step::Yield, Step::Yield) => Step::Yield,
            _ => Step::Break,
        }
    }

    #[inline]
    fn done(self) -> (O1, O2) {
        (self.0.done(), self.1.done())
    }
}
