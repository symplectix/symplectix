//! mung adds an another layer of adapters.

use std::iter;

/// Adapter has a common set of adapters
/// and creates the actual adapter when applied.
pub trait Adapter<T> {
    /// Type of result of adapt.
    type Adapted;

    /// Creates the actual adapter type.
    fn apply(self, that: T) -> Self::Adapted;

    /// Composes two adapters into a single adapter.
    fn compose<F>(self, f: F) -> Compose<Self, F>
    where
        Self: Sized,
    {
        Compose::new(self, f)
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
pub struct Map<F> {
    mapper: F,
}

impl<F> Map<F> {
    /// Constructs a mapping adapter.
    pub fn new(mapper: F) -> Self {
        Map { mapper }
    }
}

impl<T, F, A> Adapter<T> for Map<F>
where
    T: iter::Iterator,
    F: FnMut(T::Item) -> A,
{
    type Adapted = iter::Map<T, F>;

    fn apply(self, that: T) -> Self::Adapted {
        that.map(self.mapper)
    }
}

/// Filtering function adapter.
#[derive(Clone, Debug)]
pub struct Filter<P> {
    predicate: P,
}

impl<P> Filter<P> {
    /// Constructs a filtering adapter.
    pub fn new(predicate: P) -> Self {
        Filter { predicate }
    }
}

impl<T, P> Adapter<T> for Filter<P>
where
    T: iter::Iterator,
    P: FnMut(&T::Item) -> bool,
{
    type Adapted = iter::Filter<T, P>;

    fn apply(self, that: T) -> Self::Adapted {
        that.filter(self.predicate)
    }
}
