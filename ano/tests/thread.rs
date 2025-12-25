#![allow(missing_docs)]

use ano::Fold;

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

#[test]
fn thread_scope_fold() {
    use std::thread;
    let data = vec![1, 2, 3, 4, 5, 6];
    let map_sum = sum::<_, i32>().map(mul3);
    let sums = thread::scope(|scope| {
        let mut handles = Vec::with_capacity(data.len() / 3 + 1);
        data.chunks(3).for_each(|chunk| {
            let f = map_sum.clone();
            handles.push(scope.spawn(move || f.fold(chunk)));
        });
        handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>()
    });
    assert_eq!(sum::<_, i32>().fold(&sums), 63);
}
