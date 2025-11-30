//! mung adds an another layer of adapters.

use std::iter;

/// Adapter has a common set of adapters
/// and creates the actual adapter when applied.
pub trait Adapter<T> {
    /// Type of result of adapt.
    type Adapted;

    /// Creates the actual adapter type.
    fn apply(self, that: T) -> Self::Adapted;
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

impl<T, F, X> Adapter<T> for Map<F>
where
    T: iter::Iterator,
    F: FnMut(T::Item) -> X,
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
