use criterion::{Criterion, criterion_group, criterion_main};
use hpc_rs::simd::*;
use std::hint::black_box;
use rand::distr::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("SIMD add");
    let mut rng = StdRng::seed_from_u64(0);
    let uniform = Uniform::new(0., 1.).unwrap();
    // a,b,c are all aligned
    let a_vec: Vec<f64> = (0..10_000_000).map(|_| uniform.sample(&mut rng)).collect();
    let b_vec: Vec<f64> = (0..10_000_000).map(|_| uniform.sample(&mut rng)).collect();
    let mut c_vec: Vec<f64> = vec![0.; 10_000_000];


    group.bench_function("regular add", |b| {
        b.iter(|| vector_types::add(black_box(&a_vec), black_box(&b_vec), &mut c_vec))
    });

    group.bench_function("simd add", |b| {
        b.iter(|| vector_types::simd_add(black_box(&a_vec), black_box(&b_vec), &mut c_vec))
    });

    group.finish();
}

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("SIMD sum");
    let mut rng = StdRng::seed_from_u64(0);
    let uniform = Uniform::new_inclusive(-1000, 1000).unwrap();
    // a is aligned
    let a_vec: Vec<i32> = (0..1_000_000).map(|_| uniform.sample(&mut rng)).collect();

    group.bench_function("regular sum", |b| {
        b.iter(|| reductions::sum(black_box(&a_vec)))
    });

    group.bench_function("simd sum", |b| {
        b.iter(|| reductions::simd_sum(black_box(&a_vec)))
    });

    group.bench_function("block simd sum (B=2)", |b| {
        b.iter(|| reductions::block_simd_sum::<2>(black_box(&a_vec)))
    });

    group.finish();
}

// Criterion group for all benchmarks
criterion_group!(benches, bench_add, bench_sum);
// Criterion main function
criterion_main!(benches);
