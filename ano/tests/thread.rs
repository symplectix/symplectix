#![allow(missing_docs)]

use ano::{Fold, InitialState};
use std::ops::ControlFlow::*;
use std::slice::Chunks;
use std::thread::{self, Scope, ScopedJoinHandle};

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
struct FoldChunks<'scope, 'env, G> {
    scope: &'scope Scope<'scope, 'env>,
    g: G,
}

impl<'s, 'e, G> FoldChunks<'s, 'e, G> {
    fn new(scope: &'s Scope<'s, 'e>, g: G) -> Self {
        FoldChunks { scope, g }
    }
}

impl<'s, 'e, A, T, B, G> Fold<T, Vec<B>> for FoldChunks<'s, 'e, G>
where
    T: IntoIterator<Item = &'e A> + Send + 's,
    A: 'e,
    B: Send + 'e,
    G: Fold<&'e A, B> + InitialState<<G as Fold<&'e A, B>>::State> + Clone + Send + 's,
{
    type State = Vec<ScopedJoinHandle<'s, B>>;

    fn step(&mut self, mut acc: Self::State, item: T) -> ano::Step<Self::State> {
        let sf = self.g.clone();
        acc.push(self.scope.spawn(move || sf.fold(item)));
        Continue(acc)
    }

    fn complete(self, acc: Self::State) -> Vec<B> {
        acc.into_iter().map(|h| h.join().unwrap()).collect()
    }
}

impl<'s, 'e, B, G> InitialState<Vec<ScopedJoinHandle<'s, B>>> for FoldChunks<'s, 'e, G> {
    fn initial_state(&self, (lo, _hi): (usize, Option<usize>)) -> Vec<ScopedJoinHandle<'s, B>> {
        Vec::with_capacity(lo.saturating_add(1))
    }
}

fn fold_slice<'a, 'b, A, B, C, F, G>(f: F, g: G, data: &'a [A]) -> C
where
    'a: 'b,
    A: Sync,
    B: Send + 'b,
    F: Fold<B, C> + InitialState<<F as Fold<B, C>>::State>,
    G: Fold<&'b A, B> + InitialState<<G as Fold<&'b A, B>>::State> + Clone + Send + 'b,
{
    fold_chunks(f, g, data.chunks(3))
}

fn fold_chunks<'a, A, B, C, F, G>(f: F, g: G, chunks: Chunks<'a, A>) -> C
where
    A: Sync,
    B: Send + 'a,
    F: Fold<B, C> + InitialState<<F as Fold<B, C>>::State>,
    G: Fold<&'a A, B> + InitialState<<G as Fold<&'a A, B>>::State> + Clone + Send + 'a,
{
    let bs = thread::scope(|scope| FoldChunks::new(scope, g).fold(chunks));
    f.fold(bs)
}

#[test]
fn thread_scope_fold() {
    let data = vec![1, 2, 3, 4, 5, 6];
    let r = fold_slice(sum::<_, i32>(), sum::<_, i32>().map(mul3), &data);
    assert_eq!(r, 63);
    let r = fold_slice(sum::<_, usize>(), count().map(mul3), &data);
    assert_eq!(r, 6);
}
