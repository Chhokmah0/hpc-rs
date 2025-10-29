use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use hpc_rs::simd::*;

fn bench_reduce_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reduce Sum");
    group.bench_function("sum", |b| {
        let a: Vec<i32> = (0..1_000_000).collect();
        b.iter(|| {
            sum(black_box(&a))
        })
    });
    group.finish();
}

// Criterion group for all benchmarks
criterion_group!(benches, bench_reduce_sum);
// Criterion main function
criterion_main!(benches);
