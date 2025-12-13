#![allow(missing_docs)]
//! Composable transformations.

// Refs:
// - [foldl](https://github.com/Gabriella439/foldl)
// - [prefolds](https://github.com/effectfully/prefolds)
// - [transducers](https://clojure.org/reference/transducers)
// - [xforms](https://github.com/cgrand/xforms)

use std::borrow::Borrow;
use std::marker::PhantomData;

// xforms
mod filter;
mod map;
use filter::FilterXf;
use map::MapXf;

// foldings
mod either;
use either::Either;

pub trait Fold<T> {
    /// The accumulator, used to store the intermediate result while folding.
    type Acc;

    /// Runs just a one step of folding.
    fn step<Q>(&mut self, acc: Self::Acc, input: &Q) -> Step<Self::Acc>
    where
        Q: Borrow<T>;

    /// Invoked when folding is complete.
    ///
    /// - By default, done just returns acc.
    /// - You must call `done` exactly once.
    #[inline]
    fn done(self, acc: Self::Acc) -> Self::Acc
    where
        Self: Sized,
    {
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

// Xform exists only to compose xf and an another XformFn.
#[derive(Debug)]
pub struct Prep<Xf> {
    xf: Xf,
}

/// An adapter that creates a new [Fold] from the given one.
pub trait Xform<Sf> {
    /// A new step function created by apply.
    type Fold;

    /// Creates a new [Fold] from the given one.
    fn apply(self, sf: Sf) -> Self::Fold;
}

impl<T> Prep<Id<T>> {
    fn id() -> Self {
        Prep { xf: id() }
    }
}

impl<Xf> Prep<Xf> {
    fn apply<Sf>(self, step_fn: Sf) -> Xf::Fold
    where
        Xf: Xform<Sf>,
    {
        self.xf.apply(step_fn)
    }

    fn comp<That>(self, that: That) -> Prep<Comp<Xf, That>> {
        Prep { xf: comp(self.xf, that) }
    }

    pub fn map<F>(self, f: F) -> Prep<Comp<Xf, MapXf<F>>> {
        self.comp(MapXf::new(f))
    }

    pub fn filter<P>(self, pred: P) -> Prep<Comp<Xf, FilterXf<P>>> {
        self.comp(FilterXf::new(pred))
    }
}

pub struct Id<T>(PhantomData<T>);

fn id<T>() -> Id<T> {
    Id(PhantomData)
}

// impl<T> StepFn<T> for Id<T> {
//     type Acc = T;
//     fn step(&mut self, _acc: Self::Acc, input: T) -> Step<Self::Acc> {
//         input
//     }
//     fn done(self, acc: Self::Acc) -> Self::Acc {
//         acc
//     }
// }

impl<Sf, T> Xform<Sf> for Id<T>
where
    Sf: Fold<T>,
{
    type Fold = Sf;

    #[inline]
    fn apply(self, step_fn: Sf) -> Self::Fold {
        step_fn
    }
}

fn comp<A, B>(a: A, b: B) -> Comp<A, B> {
    Comp { a, b }
}

/// Comp is an adapter of [Adapter]s.
pub struct Comp<A, B> {
    a: A,
    b: B,
}

impl<Sf, A, B> Xform<Sf> for Comp<A, B>
where
    A: Xform<B::Fold>,
    B: Xform<Sf>,
{
    type Fold = A::Fold;

    fn apply(self, rf: Sf) -> Self::Fold {
        self.a.apply(self.b.apply(rf))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, ToOwned};
    use std::collections::VecDeque;

    use super::{Fold, Prep, Step, Xform};

    struct Conj;

    impl<T> Fold<T> for Conj
    where
        T: ToOwned,
    {
        type Acc = Vec<T::Owned>;

        fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> Step<Self::Acc>
        where
            Q: Borrow<T>,
        {
            acc.push(input.borrow().to_owned());
            Step::Yield(acc)
        }
    }

    struct Cons;

    impl<T> Fold<T> for Cons
    where
        T: ToOwned,
    {
        type Acc = VecDeque<T::Owned>;

        fn step<Q>(&mut self, mut acc: Self::Acc, input: &Q) -> Step<Self::Acc>
        where
            Q: Borrow<T>,
        {
            acc.push_front(input.borrow().to_owned());
            Step::Yield(acc)
        }
    }

    #[test]
    fn test_map_filter_step() {
        let mut fold = Prep::id()
            .map(|x: &i32| x + 1)
            .filter(|x: &i32| *x % 2 == 0)
            .apply(Cons)
            .either(Prep::id().map(|x: &i32| x - 1).filter(|x: &i32| *x % 2 != 0).apply(Conj));
        let mut acc = (VecDeque::with_capacity(10), vec![]);
        for i in 0..10 {
            match fold.step(acc, &i) {
                Step::Yield(ret) => {
                    acc = ret;
                }
                Step::Break(ret) => {
                    acc = fold.done(ret);
                    break;
                }
            }
        }
        assert_eq!(acc, (VecDeque::from([10, 8, 6, 4, 2]), vec![-1, 1, 3, 5, 7]));
    }
}
