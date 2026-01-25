#![allow(missing_docs)]
use compacts::bits::{
    Fold,
    Mask,
    and,
    or,
    xor,
};
use compacts::ops::*;
use compacts::{
    BitArray,
    BitMap,
    WaveletMatrix,
};
use lazy_static::lazy_static;
use quickcheck::quickcheck;
use rand::prelude::*;

quickcheck! {
    fn index_all(vec: Vec<u64>) -> bool {
        let mut xs = vec.clone();
        let wm = WaveletMatrix::<u64, BitArray<u64>>::from(&mut xs[..]);
        vec.iter().enumerate().all(|(i, v)| wm.get(i).unwrap() == *v)
    }
}
