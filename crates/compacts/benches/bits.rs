//! Bench!!

#![allow(non_snake_case)]
#![feature(test)]
extern crate test;

use compacts::ops::*;
use compacts::{
    BitArray,
    BitMap,
    Pop,
};
use lazy_static::lazy_static;
use rand::prelude::*;
use test::Bencher;

// type BitMap = compacts::BitMap<[u64; 1024]>;

macro_rules! generate {
    (Vec; $rng:expr, $nbits:expr, $bound:expr) => {{
        // let mut build = vec![0; compacts::bits::blocks_by($bound, 64)];
        let mut build = compacts::bits::sized($bound);
        for _ in 0..$nbits {
            build.put1($rng.random_range(0..$bound));
        }
        build
    }};
    (Pop; $rng:expr, $nbits:expr, $bound:expr) => {{
        let mut build = Pop::new($bound);
        for _ in 0..$nbits {
            build.put1($rng.random_range(0..$bound));
        }
        build
    }};
    (BitMap; $rng:expr, $nbits:expr, $bound:expr) => {{
        let mut build = BitMap::none($bound);
        for _ in 0..$nbits {
            build.put1($rng.random_range(0..$bound));
        }
        build
    }};
}

const BOUND: usize = 10_000_000;

lazy_static! {
    static ref NBITS: usize = BOUND / rand::rng().random_range(1..100);
    static ref V0: Vec<u64> = generate!(Vec; rand::rng(), *NBITS, BOUND);
    static ref V1: Vec<u64> = generate!(Vec; rand::rng(), *NBITS, BOUND);
    static ref V2: Vec<u64> = generate!(Vec; rand::rng(), *NBITS, BOUND);
    static ref P0: Pop<u64> = generate!(Pop; rand::rng(), *NBITS, BOUND);
    static ref P1: Pop<u64> = generate!(Pop; rand::rng(), *NBITS, BOUND);
    static ref P2: Pop<u64> = generate!(Pop; rand::rng(), *NBITS, BOUND);
    static ref M0: BitMap<[u64; 1024]> = generate!(BitMap; rand::rng(), *NBITS, BOUND);
    static ref M1: BitMap<[u64; 1024]> = generate!(BitMap; rand::rng(), *NBITS, BOUND);
    static ref M2: BitMap<[u64; 1024]> = generate!(BitMap; rand::rng(), *NBITS, BOUND);
    static ref A0: BitArray<u64> = BitArray::from(V0.clone());
    static ref A1: BitArray<u64> = BitArray::from(V1.clone());
    static ref A2: BitArray<u64> = BitArray::from(V2.clone());
}

mod bit_vec {
    use super::*;

    #[bench]
    fn bit(bench: &mut Bencher) {
        let cap = V0.size() - 1;
        bench.iter(|| V0.bit(rand::rng().random_range(0..cap)));
    }

    #[bench]
    fn put1(bench: &mut Bencher) {
        let mut v0 = V0.clone();
        let cap = v0.size() - 1;
        bench.iter(|| {
            v0.put1(rand::rng().random_range(0..cap));
        });
    }
}

mod pop_vec {
    use super::*;

    #[bench]
    fn put1(bench: &mut Bencher) {
        let mut p0 = P0.clone();
        let cap = p0.len() - 1;
        bench.iter(|| {
            p0.put1(rand::rng().random_range(0..cap));
        });
    }
}

mod bit_map {
    use super::*;

    #[bench]
    fn bit(bench: &mut Bencher) {
        let cap = M0.size() - 1;
        bench.iter(|| M0.bit(rand::rng().random_range(0..cap)));
    }

    #[bench]
    fn put1(bench: &mut Bencher) {
        let mut m0 = M0.clone();
        let cap = m0.size() - 1;
        bench.iter(|| {
            m0.put1(rand::rng().random_range(0..cap));
        });
    }
}

mod rank {
    use super::*;

    #[bench]
    fn BitSlice(bench: &mut Bencher) {
        bench.iter(|| V0.rank1(..rand::rng().random_range(0..V0.size())));
    }

    #[bench]
    fn BitArray(bench: &mut Bencher) {
        bench.iter(|| A0.rank1(..rand::rng().random_range(0..A0.size())));
    }

    #[bench]
    fn BitMap(bench: &mut Bencher) {
        bench.iter(|| M0.rank1(..rand::rng().random_range(0..M0.size())));
    }

    #[bench]
    fn PopVec(bench: &mut Bencher) {
        bench.iter(|| P0.rank1(..rand::rng().random_range(0..P0.len())));
    }
}

mod select {
    use super::*;

    #[bench]
    fn BitSlice(bench: &mut Bencher) {
        let cap = V0.count1() - 1;
        bench.iter(|| V0.select1(rand::rng().random_range(0..cap)));
    }

    #[bench]
    fn BitArray(bench: &mut Bencher) {
        let cap = A0.count1() - 1;
        bench.iter(|| A0.select1(rand::rng().random_range(0..cap)));
    }

    #[bench]
    fn BitMap(bench: &mut Bencher) {
        let cap = M0.count1() - 1;
        bench.iter(|| M0.select1(rand::rng().random_range(0..cap)));
    }

    #[bench]
    fn PopVec(bench: &mut Bencher) {
        let cap = P0.count1() - 1;
        bench.iter(|| P0.select1(rand::rng().random_range(0..cap)));
    }
}
