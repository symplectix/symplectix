use std::borrow::Borrow;

pub trait Fold<T>: Sized {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>;

    /// Invoked when folding is complete.
    ///
    /// You must call `done` exactly once.
    fn done(self, acc: Self::Acc) -> Self::Acc;

    fn fold<I, Q>(mut self, mut acc: Self::Acc, iterable: I) -> Self::Acc
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<T>,
    {
        for i in iterable.into_iter() {
            match self.step(acc, &i) {
                Step::Yield(ret) => {
                    acc = ret;
                }
                Step::Break(ret) => {
                    acc = self.done(ret);
                    break;
                }
            }
        }
        acc
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

impl<T, A, B> Fold<T> for (A, B)
where
    A: Fold<T>,
    B: Fold<T>,
{
    type Acc = (<A as Fold<T>>::Acc, <B as Fold<T>>::Acc);

    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::Yield(a), Step::Yield(b)) => Step::Yield((a, b)),
            (Step::Break(a), Step::Yield(b)) => Step::Break((a, b)),
            (Step::Yield(a), Step::Break(b)) => Step::Break((a, b)),
            (Step::Break(a), Step::Break(b)) => Step::Break((a, b)),
        }
    }

    fn done(self, acc: Self::Acc) -> Self::Acc {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
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
impl<Sf, F, A, B> Fold<A> for Map<Sf, F>
where
    Sf: Fold<B>,
    F: FnMut(&A) -> B,
{
    type Acc = Sf::Acc;

    #[inline]
    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<A>,
    {
        let mapped = (self.mapf)(input.borrow());
        self.sf.step(acc, &mapped)
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
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
impl<Sf, P, T> Fold<T> for Filter<Sf, P>
where
    Sf: Fold<T>,
    P: FnMut(&T) -> bool,
{
    type Acc = Sf::Acc;

    #[inline]
    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        if (self.pred)(input.borrow()) { self.sf.step(acc, input) } else { Step::Yield(acc) }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
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
impl<Sf, T> Fold<T> for Take<Sf>
where
    Sf: Fold<T>,
{
    type Acc = Sf::Acc;

    #[inline]
    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        match self.count {
            #[allow(unreachable_code)]
            0 => {
                unreachable!("this should not happen");
                Step::Break(acc)
            }
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
    fn done(self, acc: Self::Acc) -> Self::Acc {
        self.sf.done(acc)
    }
}

#[derive(Debug)]
pub struct Either<A, B>(pub(crate) A, pub(crate) B);
impl<T, A, B> Fold<T> for Either<A, B>
where
    A: Fold<T>,
    B: Fold<T>,
{
    type Acc = (<A as Fold<T>>::Acc, <B as Fold<T>>::Acc);

    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>,
    {
        match (self.0.step(acc.0, input), self.1.step(acc.1, input)) {
            (Step::Yield(a), Step::Yield(b)) => Step::Yield((a, b)),
            (Step::Break(a), Step::Yield(b)) => Step::Break((a, b)),
            (Step::Yield(a), Step::Break(b)) => Step::Break((a, b)),
            (Step::Break(a), Step::Break(b)) => Step::Break((a, b)),
        }
    }

    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc {
        (self.0.done(acc.0), self.1.done(acc.1))
    }
}
