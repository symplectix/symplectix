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
    let f = sum::<_, i32>();
    let r = thread::scope(|s| {
        let mut results = Vec::with_capacity(2);
        let mut handles = Vec::with_capacity(2);
        let g = f.clone();
        handles.push(s.spawn(|| g.fold(&data[..3])));
        let g = f.clone();
        handles.push(s.spawn(|| g.fold(&data[3..])));
        for h in handles {
            results.push(h.join().unwrap());
        }
        results
    });

    assert_eq!(r, vec![6, 15]);
}
