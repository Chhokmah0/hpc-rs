use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hpc_rs::instruction_level_parallelism::{branchless_programming::*, the_cost_of_branching::*};

fn bench_branch_cost(c: &mut Criterion) {
    c.bench_function("branch cost", |b| b.iter(|| branch_cost()));
}

fn bench_branch_cost_sort(c: &mut Criterion) {
    c.bench_function("branch cost when sort", |b| b.iter(|| branch_cost_sort()));
}

fn bench_branch_cost_likely(c: &mut Criterion) {
    c.bench_function("branch cost when likely", |b| b.iter(|| branch_cost_likely()));
}

fn bench_branchless_cost(c: &mut Criterion) {
    c.bench_function("branchless cost", |b| b.iter(|| branchless_cost()));
}

fn bench_branchless_no_volatile(c: &mut Criterion) {
    c.bench_function("branchless no volatile", |b| b.iter(|| branchless_no_volatile()));
}

criterion_group!(benches, bench_branch_cost, bench_branchless_cost, bench_branchless_no_volatile);
criterion_main!(benches);