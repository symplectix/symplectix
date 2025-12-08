#![allow(missing_docs)]
//! experimental implementations of transduer in rust.

/// A reducer adapter, a.k.a "Transducer".
pub trait Adapter<Rf> {
    type Adapted;

    fn apply(self, rf: Rf) -> Self::Adapted;

    fn compose<T>(self, that: T) -> Compose<Self, T>
    where
        Self: Sized,
    {
        compose(self, that)
    }

    fn map<F>(self, f: F) -> Compose<Self, Map<F>>
    where
        Self: Sized,
    {
        compose(self, map(f))
    }

    // fn filter<P>(self, p: P) -> Compose<Self, Filter<P>>
    // where
    //     Self: Sized,
    // {
    //     compose(self, filter(f))
    // }
}

pub trait Reducer<T> {
    type Acc;

    /// Invoked when reducing.
    fn step(&mut self, acc: Self::Acc, v: T) -> Step<Self::Acc>;

    /// Invoked when reducing has completed.
    fn done(&mut self, acc: Self::Acc) -> Self::Acc;
}

#[derive(Debug, Copy, Clone)]
pub enum Step<T> {
    Done(T),
    Next(T),
}

pub struct Compose<A, B> {
    a: A,
    b: B,
}
pub fn compose<A, B>(a: A, b: B) -> Compose<A, B> {
    Compose { a, b }
}

impl<Rf, A, B> Adapter<Rf> for Compose<A, B>
where
    A: Adapter<Rf>,
    B: Adapter<A::Adapted>,
{
    type Adapted = B::Adapted;

    fn apply(self, rf: Rf) -> Self::Adapted {
        self.b.apply(self.a.apply(rf))
    }
}

pub struct Map<F> {
    mapper: F,
}
pub fn map<F>(f: F) -> Map<F> {
    Map { mapper: f }
}

pub struct MapReducer<Rf, F> {
    rf: Rf,
    mapper: F,
}

impl<Rf, F> Adapter<Rf> for Map<F> {
    type Adapted = MapReducer<Rf, F>;

    fn apply(self, rf: Rf) -> Self::Adapted {
        MapReducer { rf, mapper: self.mapper }
    }
}

impl<Rf, F, A, B> Reducer<A> for MapReducer<Rf, F>
where
    Rf: Reducer<B>,
    F: FnMut(A) -> B,
{
    type Acc = Rf::Acc;

    fn step(&mut self, acc: Self::Acc, v: A) -> Step<Self::Acc> {
        self.rf.step(acc, (self.mapper)(v))
    }

    fn done(&mut self, acc: Self::Acc) -> Self::Acc {
        self.rf.done(acc)
    }
}

// pub struct Filter<P> {
//     predicate: P,
// }
// pub fn filter<P>(predicate: P) -> Filter<P> {
//     Filter { predicate }
// }

// pub struct FilterReducer<Rf, P> {
//     rf: Rf,
//     predicate: P,
// }

// impl<Rf, P> Adapter<Rf> for Filter<P> {
//     type Adapted = FilterReducer<Rf, P>;

//     fn apply(self, rf: Rf) -> Self::Adapted {
//         FilterReducer { rf, predicate: self.predicate }
//     }
// }

// impl<Rf, F, T> Reducer<T> for FilterReducer<Rf, F> {
//     type Acc = ();

//     fn step(self, acc: Self::Acc, v: T) -> Step<Self::Acc> {
//         unimplemented!()
//     }
//     fn done(self, acc: Self::Acc) -> Step<Self::Acc> {
//         unimplemented!()
//     }
// }
