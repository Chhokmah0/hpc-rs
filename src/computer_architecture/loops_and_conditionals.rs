//! It looks like the rust compiler optimizes all of this under opt-level=2/3.
//! when opt-level is 0. For sum is faster than iterators.
//! Unrolling loop does work when opt-level is 0/1.

#[allow(dead_code)]
fn for_sum(n: usize) -> usize {
    let mut sum = 0usize;
    for i in 0..n {
        sum = sum.wrapping_add(i);
    }
    sum
}

#[allow(dead_code)]
fn unroll_for_sum(n: usize) -> usize {
    let mut sum = 0usize;
    for i in 0..n / 4 {
        sum = sum.wrapping_add(4 * i);
        sum = sum.wrapping_add(4 * i + 1);
        sum = sum.wrapping_add(4 * i + 2);
        sum = sum.wrapping_add(4 * i + 3);
    }
    for i in (n / 4 * 4)..n {
        sum = sum.wrapping_add(i);
    }
    sum
}

#[allow(dead_code)]
fn iterator_sum(n: usize) -> usize {
    let mut sum = 0usize;
    (0..n).for_each(|i| sum = sum.wrapping_add(i));
    sum
}

#[allow(dead_code)]
fn iterator_sum2(n: usize) -> usize {
    (0..n).fold(0usize, |sum, i| sum.wrapping_add(i))
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{black_box, Bencher};

    const N: usize = 1000000;

    #[bench]
    fn for_test(b: &mut Bencher) {
        b.iter(|| black_box(for_sum(black_box(N))))
    }

    #[bench]
    fn unroll_for_test(b: &mut Bencher) {
        b.iter(|| black_box(unroll_for_sum(black_box(N))))
    }

    #[bench]
    fn iterator_test(b: &mut Bencher) {
        b.iter(|| black_box(iterator_sum(black_box(N))))
    }

    #[bench]
    fn iterator_test2(b: &mut Bencher) {
        b.iter(|| black_box(iterator_sum2(black_box(N))))
    }
}
