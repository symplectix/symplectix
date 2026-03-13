//! Bench!!

#![feature(test)]
extern crate test;

use compacts::ops::*;
use compacts::{
    BitArray,
    WaveletMatrix,
};
use lazy_static::lazy_static;
use rand::prelude::*;
use test::Bencher;

type BitMap = compacts::BitMap<[u64; 1024]>;

macro_rules! generate {
    ($rng:expr, $len:expr, $tab:expr) => {{
        // let mut build = Vec::<u64>::with_capacity($len);
        let mut build = vec![0; $len];
        for i in 0..$len {
            build[i] = $tab[$rng.random_range(0..$tab.len())];
        }
        build
    }};
    ($rng:expr, $len:expr) => {{
        // let mut build = Vec::<u64>::with_capacity($len);
        let mut build = vec![0; $len];
        for i in 0..$len {
            build[i] = $rng.random_range(0..$len as u32);
        }
        build
    }};
}

const LENGTH: usize = 100_000_000;

lazy_static! {
    static ref T1: Vec<u32> = generate!(rand::rng(), 1000);
    static ref S0: Vec<u32> = generate!(rand::rng(), LENGTH);
    static ref S1: Vec<u32> = generate!(rand::rng(), 100_000_000, T1);
}

mod wm_vec {
    use super::*;

    lazy_static! {
        static ref W0: WaveletMatrix<u32, BitArray<u64>> = {
            let mut vec = S0.clone();
            WaveletMatrix::from(&mut vec[..])
        };
        static ref W1: WaveletMatrix<u32, BitArray<u64>> = {
            let mut vec = S1.clone();
            WaveletMatrix::from(&mut vec[..])
        };
    }

    #[bench]
    #[ignore]
    fn build(bench: &mut Bencher) {
        let mut vec = S0.clone();
        bench.iter(|| WaveletMatrix::<u32, BitArray<u64>>::from(vec.as_mut_slice()))
    }

    #[bench]
    fn rank5(bench: &mut Bencher) {
        bench.iter(|| W0.rank(&5, ..rand::rng().random_range(0..W0.size())));
    }

    #[bench]
    fn rank5_all(bench: &mut Bencher) {
        bench.iter(|| W0.view(0..rand::rng().random_range(0..W0.size())).counts(&5));
    }

    #[bench]
    fn rank7(bench: &mut Bencher) {
        bench.iter(|| W0.rank(&7, ..rand::rng().random_range(0..W0.size())));
    }

    #[bench]
    fn select5(bench: &mut Bencher) {
        let c = W0.rank(&5, ..W0.size());
        bench.iter(|| W0.select(&5, c / 2));
    }

    #[bench]
    fn select7(bench: &mut Bencher) {
        let c = W0.rank(&7, ..W0.size());
        bench.iter(|| W0.select(&7, c / 2));
    }

    #[bench]
    fn quantile(bench: &mut Bencher) {
        bench.iter(|| W0.view(2_000_000..14_000_000).quantile(rand::rng().random_range(0..1000)));
    }

    #[bench]
    fn topk(bench: &mut Bencher) {
        let m = rand::rng().random_range(0..2_000_000);
        let n = rand::rng().random_range(0..7_000_000);
        bench.iter(|| W1.view(m..m + n).topk(1000));
    }

    #[bench]
    fn mink(bench: &mut Bencher) {
        let m = rand::rng().random_range(0..2_000_000);
        let n = rand::rng().random_range(0..7_000_000);
        bench.iter(|| W1.view(m..m + n).mink(1000));
    }

    #[bench]
    fn maxk(bench: &mut Bencher) {
        let m = rand::rng().random_range(0..2_000_000);
        let n = rand::rng().random_range(0..7_000_000);
        bench.iter(|| W1.view(m..m + n).maxk(1000));
    }
}

mod wm_map {
    use super::*;

    lazy_static! {
        static ref W0: WaveletMatrix<u32, BitMap> = {
            let mut vec = S0.clone();
            WaveletMatrix::from(&mut vec[..])
        };
        static ref W1: WaveletMatrix<u32, BitMap> = {
            let mut vec = S1.clone();
            WaveletMatrix::from(&mut vec[..])
        };
    }

    #[bench]
    #[ignore]
    fn build(bench: &mut Bencher) {
        let mut vec = S0.clone();
        bench.iter(|| WaveletMatrix::<u32, BitMap>::from(vec.as_mut_slice()))
    }

    #[bench]
    fn rank5(bench: &mut Bencher) {
        bench.iter(|| W0.rank(&5, ..rand::rng().random_range(0..W0.size())));
    }

    #[bench]
    fn rank5_all(bench: &mut Bencher) {
        bench.iter(|| W0.view(0..rand::rng().random_range(0..W0.size())).counts(&5));
    }

    #[bench]
    fn rank7(bench: &mut Bencher) {
        bench.iter(|| W0.rank(&7, ..rand::rng().random_range(0..W0.size())));
    }

    #[bench]
    fn select5(bench: &mut Bencher) {
        let c = W0.rank(&5, ..W0.size());
        bench.iter(|| W0.select(&5, c / 2));
    }

    #[bench]
    fn select7(bench: &mut Bencher) {
        let c = W0.rank(&7, ..W0.size());
        bench.iter(|| W0.select(&7, c / 2));
    }

    #[bench]
    fn quantile(bench: &mut Bencher) {
        bench.iter(|| W0.view(2_000_000..14_000_000).quantile(rand::rng().random_range(0..1000)));
    }

    #[bench]
    fn topk(bench: &mut Bencher) {
        let m = rand::rng().random_range(0..2_000_000);
        let n = rand::rng().random_range(0..7_000_000);
        bench.iter(|| W1.view(m..m + n).topk(1000));
    }

    #[bench]
    fn mink(bench: &mut Bencher) {
        let m = rand::rng().random_range(0..2_000_000);
        let n = rand::rng().random_range(0..7_000_000);
        bench.iter(|| W1.view(m..m + n).mink(1000));
    }

    #[bench]
    fn maxk(bench: &mut Bencher) {
        let m = rand::rng().random_range(0..2_000_000);
        let n = rand::rng().random_range(0..7_000_000);
        bench.iter(|| W1.view(m..m + n).maxk(1000));
    }
}
