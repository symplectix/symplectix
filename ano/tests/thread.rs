#![allow(missing_docs)]

use std::ops::ControlFlow::*;
use std::slice::Chunks;
use std::thread::{self, Result, Scope, ScopedJoinHandle};

use ano::{Fold, InitialState, StepFn};

mod helper;
use helper::*;

#[test]
fn check_clone() {
    fn check<T: Clone>(_: T) {}
    check(conj::<i32>());
    check(conj::<i32>().map(pow2));
    check(conj::<i32>().map(pow2).filter(even::<i32>));
    check(conj::<i32>().map(pow2).filter(even::<i32>).take(10));
}

#[test]
fn check_send() {
    fn check<T: Send>(_: T) {}
    check(conj::<i32>());
    check(conj::<i32>().map(pow2));
    check(conj::<i32>().map(pow2).filter(even::<i32>));
    check(conj::<i32>().map(pow2).filter(even::<i32>).take(10));
}

#[derive(Debug, Clone)]
struct Par<F, G> {
    f: F,
    g: G,
}

impl<F, G> Par<F, G> {
    fn new(f: F, g: G) -> Self {
        Par { f, g }
    }

    fn fold<'a, A, B, C>(self, chunks: Chunks<'a, A>) -> Result<C>
    where
        A: Sync,
        B: Send + 'a,
        F: StepFn<B, C> + InitialState<<F as StepFn<B, C>>::State>,
        G: StepFn<&'a A, B> + InitialState<<G as StepFn<&'a A, B>>::State> + Clone + Send + 'a,
    {
        let gs = thread::scope(|scope| FoldInScope::new(scope, self.g).fold(chunks))?;
        Ok(self.f.fold(gs))
    }
}

#[derive(Debug, Clone)]
struct FoldInScope<'scope, 'env, F> {
    scope: &'scope Scope<'scope, 'env>,
    f: F,
}

impl<'s, 'e, F> FoldInScope<'s, 'e, F> {
    fn new(scope: &'s Scope<'s, 'e>, f: F) -> Self {
        FoldInScope { scope, f }
    }
}

impl<'s, 'e, A, T, B, F> StepFn<T, Result<Vec<B>>> for FoldInScope<'s, 'e, F>
where
    T: IntoIterator<Item = &'e A> + Send + 's,
    A: 'e,
    B: Send + 's,
    F: StepFn<&'e A, B> + InitialState<<F as StepFn<&'e A, B>>::State> + Clone + Send + 's,
{
    type State = Vec<ScopedJoinHandle<'s, B>>;

    fn step(&mut self, mut acc: Self::State, item: T) -> ano::Step<Self::State> {
        let f = self.f.clone();
        acc.push(self.scope.spawn(move || f.fold(item)));
        Continue(acc)
    }

    fn complete(self, acc: Self::State) -> Result<Vec<B>> {
        acc.into_iter().map(|h| h.join()).collect()
    }
}

impl<'s, 'e, B, F> InitialState<Vec<ScopedJoinHandle<'s, B>>> for FoldInScope<'s, 'e, F> {
    fn initial_state(&self, (lo, _hi): (usize, Option<usize>)) -> Vec<ScopedJoinHandle<'s, B>> {
        Vec::with_capacity(lo.saturating_add(1))
    }
}

#[test]
fn thread_scope_fold() {
    let data = vec![1, 2, 3, 4, 5, 6];
    let r = Par::new(sum::<_, i32>(), sum::<_, i32>().map(mul3)).fold(data.chunks(3));
    assert_eq!(r.unwrap(), 63);
    let r = Par::new(sum::<_, usize>(), count().map(mul3)).fold(data.chunks(4));
    assert_eq!(r.unwrap(), 6);
}
