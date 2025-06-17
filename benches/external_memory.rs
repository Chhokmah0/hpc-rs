use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use hpc_rs::external_memory::cache_oblivious::{
    matrix_transpose, matrix_transpose_cache_oblivious, matrix_transpose_cache_oblivious_fast,
    Matrix,
};
use rand::distr::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn matrix_transpose_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix Transpose");
    const N: usize = 2000;

    group.bench_function("matrix_transpose", |b| {
        let mut rng = StdRng::seed_from_u64(0);
        let uniform = Uniform::new(0., 1.).unwrap();
        let mut matrix: Vec<f64> = uniform.sample_iter(&mut rng).take(N * N).collect();
        b.iter(|| {
            matrix_transpose(black_box(&mut matrix), black_box(N));
        })
    });
    group.bench_function("matrix_transpose_cache_oblivious", |b| {
        let mut rng = StdRng::seed_from_u64(0);
        let uniform = Uniform::new(0., 1.).unwrap();
        let mut matrix: Vec<f64> = uniform.sample_iter(&mut rng).take(N * N).collect();
        b.iter(|| {
            matrix_transpose_cache_oblivious(black_box(&mut matrix), black_box(N));
        })
    });
    group.bench_function("matrix_transpose_cache_oblivious_fast", |b| {
        let mut rng = StdRng::seed_from_u64(0);
        let uniform = Uniform::new(0., 1.).unwrap();
        let mut matrix: Vec<f64> = uniform.sample_iter(&mut rng).take(N * N).collect();
        b.iter(|| {
            matrix_transpose_cache_oblivious_fast(black_box(&mut matrix), black_box(N));
        })
    });
    group.bench_function("matrix_transpose_struct", |b| {
        let mut rng = StdRng::seed_from_u64(0);
        let uniform = Uniform::new(0., 1.).unwrap();
        let mut matrix = Matrix::new(N, N);
        for i in 0..N {
            for j in 0..N {
                matrix.set(i, j, uniform.sample(&mut rng));
            }
        }
        b.iter(|| {
            let _transposed_matrix = matrix.transpose();
        })
    });
    group.finish();
}

fn matrix_mul_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix Multiplication");
    const N: usize = 1024;
    group.sample_size(10);

    group.bench_function("matrix_simple_mul", |b| {
        let mut rng = StdRng::seed_from_u64(0);
        let uniform = Uniform::new(0., 1.).unwrap();
        let matrix_a = Matrix::from_vec(uniform.sample_iter(&mut rng).take(N * N).collect(), N, N);
        let matrix_b = Matrix::from_vec(uniform.sample_iter(&mut rng).take(N * N).collect(), N, N);
        b.iter(|| {
            let _result = matrix_a.simple_mul(black_box(&matrix_b));
        });
    });
    group.bench_function("matrix_transpose_mul", |b| {
        let mut rng = StdRng::seed_from_u64(0);
        let uniform = Uniform::new(0., 1.).unwrap();
        let matrix_a = Matrix::from_vec(uniform.sample_iter(&mut rng).take(N * N).collect(), N, N);
        let matrix_b = Matrix::from_vec(uniform.sample_iter(&mut rng).take(N * N).collect(), N, N);
        b.iter(|| {
            let _result = matrix_a.transpose_mul(black_box(&matrix_b));
        });
    });
    group.bench_function("matrix_mul_cache_oblivious", |b| {
        let mut rng = StdRng::seed_from_u64(0);
        let uniform = Uniform::new(0., 1.).unwrap();
        let matrix_a = Matrix::from_vec(uniform.sample_iter(&mut rng).take(N * N).collect(), N, N);
        let matrix_b = Matrix::from_vec(uniform.sample_iter(&mut rng).take(N * N).collect(), N, N);
        b.iter(|| {
            let _result = matrix_a.mul(black_box(&matrix_b));
        });
    });
    group.finish();
}

criterion_group!(benches, matrix_transpose_benchmarks, matrix_mul_benchmarks);
criterion_main!(benches);
