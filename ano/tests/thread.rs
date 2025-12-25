#![allow(missing_docs)]

use ano::{Fold, InitialState};
use std::thread;

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

fn par_fold<'a, 'b, A, B, C, F, G>(f: F, g: G, data: &'a [A]) -> C
where
    'a: 'b,
    &'b [A]: Send,
    B: Send,
    F: Fold<B, C> + InitialState<<F as Fold<B, C>>::State>,
    G: Fold<&'b A, B> + InitialState<<G as Fold<&'b A, B>>::State> + Clone + Send,
{
    let bs = thread::scope(|scope| {
        let mut handles = Vec::with_capacity(data.len() / 3 + 1);
        data.chunks(3).for_each(|chunk| {
            let g = g.clone();
            handles.push(scope.spawn(move || g.fold(chunk)));
        });
        handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>()
    });
    f.fold(bs)
}

#[test]
fn thread_scope_fold() {
    let data = vec![1, 2, 3, 4, 5, 6];
    // let map_sum = sum::<_, i32>().map(mul3);
    let r = par_fold(sum::<_, i32>(), sum::<_, i32>().map(mul3), &data);
    assert_eq!(r, 63);
}
