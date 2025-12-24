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
fn thread_spawn_fold() {
    use std::thread;
    let f = conj();
    let handle = {
        let g = f.clone();
        thread::spawn(move || g.fold([1, 2, 3]))
    };
    assert_eq!(handle.join().unwrap(), vec![1, 2, 3]);
    let handle = {
        let g = f.clone();
        thread::spawn(move || g.fold([1, 2, 3]))
    };
    assert_eq!(handle.join().unwrap(), vec![1, 2, 3]);
}
