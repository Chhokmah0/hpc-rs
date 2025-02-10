use std::usize;

use rand::{distr::{Distribution, Uniform}, Rng, SeedableRng};
const N: usize = 1_000_000;

pub fn statistical_profiling_init() -> Vec<usize> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let uniform = Uniform::new(0, usize::MAX).unwrap();
    let mut vec: Vec<usize> = uniform.sample_iter(&mut rng).take(N).collect();
    vec.sort_unstable();
    vec
}

pub fn statistical_profiling_query() -> usize {
    let mut rng = rand::rngs::StdRng::seed_from_u64(10);
    let uniform = Uniform::new(0, usize::MAX).unwrap();
    let vec = statistical_profiling_init();
    let mut check_sum = 0;
    for _ in 0..N {
        let r = uniform.sample(&mut rng);
        let p = vec.partition_point(|x| *x < r);
        check_sum += p;
    }
    check_sum
}
