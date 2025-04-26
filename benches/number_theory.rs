use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hpc_rs::number_theory::*;

fn bench_binary_exponentiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Binary Exponentiation");
    group.bench_function("binpow_rec", |b| {
        b.iter(|| binpow_rec::<1000000007>(black_box(2), black_box(1_000_000_000)))
    });

    group.bench_function("binpow_iter", |b| {
        b.iter(|| binpow_iter::<1000000007>(black_box(2), black_box(1_000_000_000)))
    });

    group.bench_function("binpow_iter_with_mod", |b| {
        b.iter(|| binpow_iter_with_mod(black_box(2), black_box(1_000_000_000), black_box(1000000007)))
    });

    group.bench_function("inverse", |b| {
        b.iter(|| inverse::<1000000007>(black_box(2)))
    });

    group.finish();
}

// Criterion group for all benchmarks
criterion_group!(benches, bench_binary_exponentiation);
// Criterion main function
criterion_main!(benches);
