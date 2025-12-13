use std::marker::PhantomData;

use crate::fold;

// Xform exists only to compose xf and an another XformFn.
#[derive(Debug)]
pub struct Prep<Xf> {
    xf: Xf,
}
impl<Xf> Prep<Xf> {
    fn new(xf: Xf) -> Self {
        Prep { xf }
    }
}

/// An adapter that creates a new [Fold] from the given one.
pub trait Xform<Sf> {
    /// A new step function created by apply.
    type Fold;

    /// Creates a new [Fold] from the given one.
    fn apply(self, sf: Sf) -> Self::Fold;
}

impl<Xf> Prep<Xf> {
    pub fn apply<Sf>(self, step_fn: Sf) -> Xf::Fold
    where
        Xf: Xform<Sf>,
    {
        self.xf.apply(step_fn)
    }

    fn comp<That>(self, that: That) -> Prep<Comp<Xf, That>> {
        Prep { xf: comp(self.xf, that) }
    }

    pub fn map<F>(self, f: F) -> Prep<Comp<Xf, Map<F>>> {
        self.comp(Map::new(f))
    }

    pub fn filter<P>(self, pred: P) -> Prep<Comp<Xf, Filter<P>>> {
        self.comp(Filter::new(pred))
    }
}

#[derive(Debug)]
pub struct Id<T>(PhantomData<T>);
impl<Sf: fold::Fold<T>, T> Xform<Sf> for Id<T> {
    type Fold = Sf;
    #[inline]
    fn apply(self, step_fn: Sf) -> Self::Fold {
        step_fn
    }
}
pub fn id<T>() -> Prep<Id<T>> {
    Prep::new(Id(PhantomData))
}

/// Comp is an adapter of [Adapter]s.
#[derive(Debug)]
pub struct Comp<A, B> {
    a: A,
    b: B,
}
fn comp<A, B>(a: A, b: B) -> Comp<A, B> {
    Comp { a, b }
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

#[derive(Debug)]
pub struct Map<F> {
    mapf: F,
}
impl<Sf, F> Xform<Sf> for Map<F> {
    type Fold = fold::Map<Sf, F>;
    fn apply(self, sf: Sf) -> Self::Fold {
        fold::Map::new(sf, self.mapf)
    }
}
pub fn map<F>(f: F) -> Prep<Map<F>> {
    Prep::new(Map::new(f))
}
impl<F> Map<F> {
    fn new(mapf: F) -> Map<F> {
        Map { mapf }
    }
}

#[derive(Debug)]
pub struct Filter<P> {
    pred: P,
}
impl<Sf, P> Xform<Sf> for Filter<P> {
    type Fold = fold::Filter<Sf, P>;
    fn apply(self, sf: Sf) -> Self::Fold {
        fold::Filter::new(sf, self.pred)
    }
}
pub fn filter<P>(pred: P) -> Prep<Filter<P>> {
    Prep::new(Filter::new(pred))
}
impl<P> Filter<P> {
    fn new(pred: P) -> Self {
        Filter { pred }
    }
}
