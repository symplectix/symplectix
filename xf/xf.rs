#![allow(missing_docs)]
//! experimental implementations of transduer in rust.

/// A reducer adapter, a.k.a "Transducer".
pub trait Adapter<Rf>: Compose {
    type Adapted;

    fn apply(self, rf: Rf) -> Self::Adapted;
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

    // fn filter<P>(self, p: P) -> Compose<Self, Filter<P>>
    // where
    //     Self: Sized,
    // {
    //     compose(self, filter(f))
    // }
}

pub trait Reducer<Acc, T> {
    /// Invoked when reducing.
    fn step(&mut self, acc: Acc, v: T) -> Step<Acc>;

    /// Invoked when reducing has completed.
    fn done(&mut self, acc: Acc) -> Acc;
}

#[derive(Debug, Copy, Clone)]
pub enum Step<T> {
    Done(T),
    Next(T),
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
    A: Adapter<B::Adapted>,
    B: Adapter<Rf>,
{
    type Adapted = A::Adapted;

    fn apply(self, rf: Rf) -> Self::Adapted {
        self.a.apply(self.b.apply(rf))
    }
}
impl<A, B> Compose for Comp<A, B> {}

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
impl<F> Compose for Map<F> {}

impl<Rf, F, Acc, A, B> Reducer<Acc, A> for MapReducer<Rf, F>
where
    Rf: Reducer<Acc, B>,
    F: FnMut(A) -> B,
{
    // type Acc = Rf::Acc;

    fn step(&mut self, acc: Acc, v: A) -> Step<Acc> {
        self.rf.step(acc, (self.mapper)(v))
    }

    fn done(&mut self, acc: Acc) -> Acc {
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
