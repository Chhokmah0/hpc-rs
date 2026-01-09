use std::arch;

pub fn add(a: &[f64], b: &[f64], c: &mut [f64]) {
    assert!(a.len() == b.len() && a.len() == c.len());
    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
}

pub fn simd_add(a: &[f64], b: &[f64], c: &mut [f64]) {
    assert!(a.len() == b.len() && a.len() == c.len());
    let chunk_size = 4; // Number of f64 values processed in one SIMD operation
    let len = a.len();
    let simd_chunks = len / chunk_size * chunk_size;

    unsafe {
        for i in (0..simd_chunks).step_by(chunk_size) {
            let a_vec = arch::x86_64::_mm256_loadu_pd(a.as_ptr().add(i));
            let b_vec = arch::x86_64::_mm256_loadu_pd(b.as_ptr().add(i));
            let c_vec = arch::x86_64::_mm256_add_pd(a_vec, b_vec);
            arch::x86_64::_mm256_storeu_pd(c.as_mut_ptr().add(i), c_vec);
        }
    }

    // Handle remaining elements
    for i in simd_chunks..len {
        c[i] = a[i] + b[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let expected = vec![11.0, 22.0, 33.0, 44.0, 55.0];
        let mut result = vec![0.0; a.len()];
        add(&a, &b, &mut result);
        assert_eq!(result, expected);
        simd_add(&a, &b, &mut result);
        assert_eq!(result, expected);
    }
}
