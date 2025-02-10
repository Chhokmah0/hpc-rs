use likely_stable::likely;
use rand::distr::{Distribution, Uniform};

pub fn branch_cost() {
    const N: usize = 1000000;
    let mut rng = rand::rng();
    let uniform = Uniform::new(0, 100).unwrap();

    let a: Vec<i32> = uniform.sample_iter(&mut rng).take(N).collect();

    let mut s = 0;
    for &val in a.iter() {
        if val < 50 {
            unsafe {
                std::ptr::write_volatile(&mut s, s + val);
            }
        }
    }
}

pub fn branch_cost_sort() {
    const N: usize = 1000000;
    let mut rng = rand::rng();
    let uniform = Uniform::new(0, 100).unwrap();

    let mut a: Vec<i32> = uniform.sample_iter(&mut rng).take(N).collect();

    a.sort_unstable();

    let mut s = 0;
    for &val in a.iter() {
        if val < 50 {
            unsafe {
                std::ptr::write_volatile(&mut s, s + val);
            }
        }
    }
}

pub fn branch_cost_likely() {
    const N: usize = 1000000;
    let mut rng = rand::rng();
    let uniform = Uniform::new(0, 100).unwrap();

    let a: Vec<i32> = uniform.sample_iter(&mut rng).take(N).collect();

    let mut s = 0;
    for &val in a.iter() {
        if likely(val < 50) {
            unsafe {
                std::ptr::write_volatile(&mut s, s + val);
            }
        }
    }
}
