use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
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
        b.iter(|| {
            binpow_iter_with_mod(
                black_box(2),
                black_box(1_000_000_000),
                black_box(1000000007),
            )
        })
    });

    group.bench_function("binpow inverse", |b| {
        b.iter(|| inverse::<1_000_000_007>(black_box(2)))
    });

    group.bench_function("binpow inverse without const", |b| {
        b.iter(|| inverse_without_const(black_box(2), black_box(1_000_000_007)))
    });

    group.bench_function("binpow inverse using montgomery", |b| {
        b.iter(|| inverse_using_montgomery(black_box(2), black_box(1_000_000_007)));
    });

    let montgomery = Montgomery::new(1_000_000_007);
    group.bench_function("binpow inverse with montgomery ref", |b| {
        b.iter(|| inverse_with_montgomery(black_box(2), black_box(&montgomery)));
    });

    group.finish();
}

fn bench_exgcd(c: &mut Criterion) {
    let mut group = c.benchmark_group("Extended GCD");
    group.bench_function("inverse exgcd rec 2^{-1} mod (1e9 + 7)", |b| {
        b.iter(|| inverse_exgcd_rec::<1_000_000_007>(black_box(2)));
    });

    group.bench_function("inverse exgcd rec 564400443^{-1} mod (1e9 + 7)", |b| {
        b.iter(|| inverse_exgcd_rec::<1_000_000_007>(black_box(564400443)));
    });

    group.bench_function("inverse exgcd iter 564400443^{-1} mod (1e9 + 7)", |b| {
        b.iter(|| inverse_exgcd_iter::<1_000_000_007>(black_box(564400443)));
    });
    group.finish();
}

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum");
    group.bench_function("sum vec slow", |b| {
        b.iter(|| slow_sum::<1_000_000_007>(black_box(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])))
    });
    group.bench_function("sum vec fast", |b| {
        b.iter(|| fast_sum::<1_000_000_007>(black_box(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])))
    });
    group.bench_function("sum vec faster", |b| {
        b.iter(|| faster_sum::<1_000_000_007>(black_box(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])))
    });
    group.finish();
}

// Criterion group for all benchmarks
criterion_group!(benches, bench_binary_exponentiation, bench_exgcd, bench_sum);
// Criterion main function
criterion_main!(benches);
