//! mung adds an another layer of adapters.

use std::iter;
use std::marker::PhantomData;

/// Adapter has a common set of adapters
/// and creates the actual adapter when applied.
pub trait Adapter<T>: Sized {
    /// Type of result of adapt.
    type Adapted;

    /// Creates the actual adapter type.
    fn apply(self, that: T) -> Self::Adapted;

    /// Composes two adapters into a single adapter.
    fn compose<F>(self, f: F) -> Compose<Self, F> {
        Compose::new(self, f)
    }

    /// Creates a new adapter.
    fn map<F>(self, mapper: F) -> Map<Self, F> {
        Map::new(self, mapper)
    }

    /// Creates a new adapter.
    fn filter<P>(self, predicate: P) -> Filter<Self, P> {
        Filter::new(self, predicate)
    }
}

/// Mung.
#[derive(Clone, Debug)]
pub struct Mung<T> {
    _ty: PhantomData<T>,
}

/// Creates a no-op adapter.
pub fn adapter<T>() -> Mung<T> {
    Mung { _ty: PhantomData }
}

impl<T> Adapter<T> for Mung<T> {
    type Adapted = T;
    fn apply(self, that: T) -> Self::Adapted {
        that
    }
}

/// Composes two adapters into a single adapter.
#[derive(Clone, Debug)]
pub struct Compose<F, G> {
    f: F,
    g: G,
}

impl<F, G> Compose<F, G> {
    /// Constructs a composing adapter.
    pub fn new(f: F, g: G) -> Self {
        Compose { f, g }
    }
}

impl<F, G, T> Adapter<T> for Compose<F, G>
where
    F: Adapter<T>,
    G: Adapter<F::Adapted>,
{
    type Adapted = G::Adapted;

    fn apply(self, that: T) -> Self::Adapted {
        self.g.apply(self.f.apply(that))
    }
}

/// Mapping function adapter.
#[derive(Clone, Debug)]
pub struct Map<A, F> {
    inner: A,
    mapper: F,
}

impl<A, F> Map<A, F> {
    /// Constructs a mapping adapter.
    pub fn new(inner: A, mapper: F) -> Self {
        Map { inner, mapper }
    }
}

impl<A, F, T, U> Adapter<T> for Map<A, F>
where
    A: Adapter<T>,
    A::Adapted: iter::Iterator,
    F: FnMut(<A::Adapted as Iterator>::Item) -> U,
{
    type Adapted = iter::Map<A::Adapted, F>;

    fn apply(self, that: T) -> Self::Adapted {
        self.inner.apply(that).map(self.mapper)
    }
}

/// Filtering function adapter.
#[derive(Clone, Debug)]
pub struct Filter<A, P> {
    inner: A,
    predicate: P,
}

impl<A, P> Filter<A, P> {
    /// Constructs a filtering adapter.
    pub fn new(inner: A, predicate: P) -> Self {
        Filter { inner, predicate }
    }
}

impl<A, P, T> Adapter<T> for Filter<A, P>
where
    A: Adapter<T>,
    A::Adapted: iter::Iterator,
    P: FnMut(&<A::Adapted as Iterator>::Item) -> bool,
{
    type Adapted = iter::Filter<A::Adapted, P>;

    fn apply(self, that: T) -> Self::Adapted {
        self.inner.apply(that).filter(self.predicate)
    }
}
