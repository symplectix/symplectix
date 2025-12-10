#![allow(missing_docs)]
//! Transducers in rust.
//! https://clojure.org/reference/transducers

mod filter;
mod map;

pub use filter::{Filter, filter};
pub use map::{Map, map};

pub trait Reducer<T> {
    /// The result of reducing.
    type Acc;

    /// Invoked when reducing.
    fn step(&mut self, acc: Self::Acc, v: T) -> Step<Self::Acc>;

    /// Invoked when reducing has completed.
    fn done(&mut self, acc: Self::Acc) -> Self::Acc;
}

/// A reducer adapter, a.k.a "Transducer".
pub trait Adapter<Rf> {
    type Reducer;

    fn apply(self, rf: Rf) -> Self::Reducer;
}

pub trait Compose {
    fn comp<T>(self, that: T) -> Comp<Self, T>
    where
        Self: Sized,
    {
        comp(self, that)
    }

    fn map<F>(self, f: F) -> Comp<Self, Map<F>>
    where
        Self: Sized,
    {
        comp(self, map(f))
    }

    fn filter<P>(self, p: P) -> Comp<Self, Filter<P>>
    where
        Self: Sized,
    {
        comp(self, filter(p))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Step<T> {
    Reduced(T),
    Continue(T),
}

pub struct Comp<A, B> {
    a: A,
    b: B,
}

fn comp<A, B>(a: A, b: B) -> Comp<A, B> {
    Comp { a, b }
}

impl<Rf, A, B> Adapter<Rf> for Comp<A, B>
where
    A: Adapter<B::Reducer>,
    B: Adapter<Rf>,
{
    type Reducer = A::Reducer;

    fn apply(self, rf: Rf) -> Self::Reducer {
        self.a.apply(self.b.apply(rf))
    }
}
impl<A, B> Compose for Comp<A, B> {}
