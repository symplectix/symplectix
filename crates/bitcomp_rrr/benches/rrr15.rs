//! Benchmark rrr where b = 15.

use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use rand::prelude::*;

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rrr15");
    group.bench_function("encode", |b| {
        let mut rng = rand::rng();
        b.iter(|| {
            let _ = rrr::encode(rng.random());
        })
    });
    group.bench_function("decode", |b| {
        let mut rng = rand::rng();
        let (c, o) = rrr::encode(rng.random());
        b.iter(|| {
            let _ = rrr::decode(c, o);
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
