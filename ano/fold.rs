use std::borrow::Borrow;

pub trait Fold<In, Out>: Sized {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step<T>(&mut self, acc: Self::Acc, input: &T) -> Step<Self::Acc>
    where
        T: Borrow<In>;

    /// Invoked when folding is complete.
    ///
    /// You must call `done` exactly once.
    fn done(self, acc: Self::Acc) -> Out;

    fn fold<It, T>(mut self, mut acc: Self::Acc, iterable: It) -> Out
    where
        It: IntoIterator<Item = T>,
        T: Borrow<In>,
    {
        for item in iterable.into_iter() {
            match self.step(acc, &item) {
                Step::Yield(ret) => {
                    acc = ret;
                }
                Step::Break(ret) => {
                    acc = ret;
                    break;
                }
            }
        }
        self.done(acc)
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
pub enum Step<T> {
    /// Keep folding.
    Yield(T),
    /// Stop folding.
    Break(T),
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
    type Acc = Sf::Acc;
    #[inline]
    fn step<T>(&mut self, acc: Self::Acc, input: &T) -> Step<Self::Acc>
    where
        T: Borrow<In>,
    {
        let mapped = (self.mapf)(input.borrow());
        self.sf.step(acc, &mapped)
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> Out {
        self.sf.done(acc)
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
    type Acc = Sf::Acc;
    #[inline]
    fn step<T>(&mut self, acc: Self::Acc, input: &T) -> Step<Self::Acc>
    where
        T: Borrow<In>,
    {
        if (self.pred)(input.borrow()) { self.sf.step(acc, input) } else { Step::Yield(acc) }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> Out {
        self.sf.done(acc)
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
    type Acc = Sf::Acc;
    #[inline]
    fn step<T>(&mut self, acc: Self::Acc, input: &T) -> Step<Self::Acc>
    where
        T: Borrow<In>,
    {
        match self.count {
            0 => Step::Break(acc),
            1 => {
                self.count = 0;
                match self.sf.step(acc, input) {
                    Step::Yield(a) => Step::Break(a),
                    Step::Break(a) => Step::Break(a),
                }
            }
            _ => {
                self.count -= 1;
                self.sf.step(acc, input)
            }
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> Out {
        self.sf.done(acc)
    }
}

#[derive(Debug)]
pub struct Either<A, B>(pub(crate) A, pub(crate) B);
impl<In, O1, O2, A, B> Fold<In, (O1, O2)> for Either<A, B>
where
    A: Fold<In, O1>,
    B: Fold<In, O2>,
{
    type Acc = (<A as Fold<In, O1>>::Acc, <B as Fold<In, O2>>::Acc);
    fn step<T>(&mut self, acc: Self::Acc, input: &T) -> Step<Self::Acc>
    where
        T: Borrow<In>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::Yield(a), Step::Yield(b)) => Step::Yield((a, b)),
            (Step::Break(a), Step::Yield(b)) => Step::Break((a, b)),
            (Step::Yield(a), Step::Break(b)) => Step::Break((a, b)),
            (Step::Break(a), Step::Break(b)) => Step::Break((a, b)),
        }
    }
    #[inline]
    fn done(self, acc: Self::Acc) -> (O1, O2) {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}
