//! Benchmark roaring partitioning.

use std::hint::black_box;
use std::ops::Range;

use bitcomp_roaring::bit_set::BitSet;
use bits::{
    Bits,
    BitsMut,
    Block,
};
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use rand::prelude::*;

const NBITS: u64 = 150_000;
const BOUND: u64 = 10_000_000;

fn gen_bits(r: Range<u64>) -> (Vec<u64>, BitSet<u64>) {
    let mut rng = rand::rng();
    let mut roaring_bv = BitSet::new();
    let mut bv = vec![0u64; bits::blocks(BOUND, <u64 as Block>::BITS)];
    for _ in 0..NBITS {
        let bit = rng.random_range(r.clone());
        roaring_bv.insert(bit);
        bv.set1(bit);
    }
    (bv, roaring_bv)
}

fn benchmark(c: &mut Criterion) {
    let (bv, roaring_bv) = gen_bits(0..BOUND);
    {
        let mut group = c.benchmark_group("bitcomp_rank");
        let i = 1 << 20;
        group.bench_function("vec_u64_rank1", |b| {
            b.iter(|| {
                let _ = black_box(bv.rank1(..i));
            })
        });
        group.bench_function("vec_u64_rank0", |b| {
            b.iter(|| {
                let _ = black_box(bv.rank0(..i));
            })
        });
        group.bench_function("roaring_rank1", |b| {
            b.iter(|| {
                let _ = black_box(roaring_bv.rank1(i));
            })
        });
        group.bench_function("roaring_rank0", |b| {
            b.iter(|| {
                let _ = black_box(roaring_bv.rank0(i));
            })
        });
    }
    {
        let n = 10000;
        let mut group = c.benchmark_group("bitcomp_select");
        group.bench_function("vec_u64_select1", |b| {
            b.iter(|| {
                let _ = black_box(bv.select1(n));
            })
        });
        group.bench_function("vec_u64_select0", |b| {
            b.iter(|| {
                let _ = black_box(bv.select0(n));
            })
        });
        group.bench_function("roaring_select1", |b| {
            b.iter(|| {
                let _ = black_box(roaring_bv.select1(n));
            })
        });
        group.bench_function("roaring_select0", |b| {
            b.iter(|| {
                let _ = black_box(roaring_bv.select0(n));
            })
        });
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
