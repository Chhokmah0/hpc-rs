use criterion::black_box;
use rand::distr::{Distribution, Uniform};

pub fn branchless_cost() {
    const N: usize = 1000000;
    let mut rng = rand::rng();
    let uniform = Uniform::new(0, 100).unwrap();

    let a: Vec<i32> = uniform.sample_iter(&mut rng).take(N).collect();

    let mut s = 0;
    for &val in a.iter() {
        unsafe {
            std::ptr::write_volatile(&mut s, s + val * (if val < 50 { 1 } else { 0 }));
            // or std::ptr::write_volatile(&mut s, s + val * (val < 50) as i32);
        }
    }
}

pub fn branchless_le0() {
    const N: usize = 1000000;
    let mut rng = rand::rng();
    let uniform = Uniform::new(0, 100).unwrap();

    let a: Vec<i32> = uniform.sample_iter(&mut rng).take(N).collect();

    let mut s = 0;
    for &val in a.iter() {
        unsafe {
            std::ptr::write_volatile(&mut s, s + val * (val - 50 < 0) as i32);
        }
    }
}

pub fn branchless_no_volatile() {
    const N: usize = 1000000;
    let mut rng = rand::rng();
    let uniform = Uniform::new(0, 100).unwrap();

    let a: Vec<i32> = uniform.sample_iter(&mut rng).take(N).collect();

    let mut s = 0;
    for &val in a.iter() {
        s += val * (val - 50 < 0) as i32;
    }
    black_box(s);
}
