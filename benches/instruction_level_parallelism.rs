use criterion::{criterion_group, criterion_main, Criterion};
use hpc_rs::instruction_level_parallelism::{branchless_programming::*, the_cost_of_branching::*};

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Instruction Level Parallelism");
    group.bench_function("branch cost", |b| b.iter(branch_cost));
    group.bench_function("branch cost when sort", |b| b.iter(branch_cost_sort));
    group.bench_function("branch cost when likely", |b| b.iter(branch_cost_likely));
    group.bench_function("branchless cost", |b| b.iter(branchless_cost));
    group.bench_function("branchless no volatile", |b| b.iter(branchless_no_volatile));
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
