pub fn sum(a: &[i32]) -> i32 {
    let mut s = 0;
    for &val in a {
        s += val;
    }
    s
}

pub fn simd_sum(a: &[i32]) -> i32 {
    use std::arch;

    unsafe {
        let (prefix, simd_chunks, suffix) = a.align_to::<arch::x86_64::__m256i>();
        let mut sum_vec = arch::x86_64::_mm256_setzero_si256();
        for &chunk in simd_chunks {
            sum_vec = arch::x86_64::_mm256_add_epi32(sum_vec, chunk);
        }
        let mut sum = 0;
        let mut temp = [0i32; 8];
        arch::x86_64::_mm256_storeu_si256(temp.as_mut_ptr() as *mut arch::x86_64::__m256i, sum_vec);
        for &val in &temp {
            sum += val;
        }
        for &val in prefix {
            sum += val;
        }
        for &val in suffix {
            sum += val;
        }
        sum
    }
}

pub fn block_simd_sum<const B: usize>(a: &[i32]) -> i32 {
    use std::arch;

    unsafe {
        let mut block_sums = [arch::x86_64::_mm256_setzero_si256(); B];
        let (prefix, simd_chunks, suffix) = a.align_to::<[arch::x86_64::__m256i; B]>();
        for chunk_pair in simd_chunks {
            for i in 0..B {
                block_sums[i] = arch::x86_64::_mm256_add_epi32(block_sums[i], chunk_pair[i]);
            }
        }
        let mut total_sum = 0;
        for &block_sum in &block_sums {
            let mut temp = [0i32; 8];
            arch::x86_64::_mm256_storeu_si256(
                temp.as_mut_ptr() as *mut arch::x86_64::__m256i,
                block_sum,
            );
            for &val in &temp {
                total_sum += val;
            }
        }
        for &val in prefix {
            total_sum += val;
        }
        for &val in suffix {
            total_sum += val;
        }
        total_sum
    }
}

pub fn hsum(a: std::arch::x86_64::__m256i) -> i32 {
    use std::arch::x86_64::*;
    unsafe {
        let lo = _mm256_extracti128_si256(a, 0);
        let hi = _mm256_extracti128_si256(a, 1);
        let sum128 = _mm_add_epi32(lo, hi);
        let hsum128 = _mm_hadd_epi32(sum128, sum128);
        _mm_extract_epi32(hsum128, 0) + _mm_extract_epi32(hsum128, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&data), 15);
        assert_eq!(simd_sum(&data), 15);
        assert_eq!(block_simd_sum::<2>(&data), 15);
    }

    #[test]
    fn test_hsum() {
        use std::arch::x86_64::*;
        unsafe {
            let vec = _mm256_setr_epi32(1, 10, 100, 1000, 10000, 100000, 1000000, 10000000);
            assert_eq!(hsum(vec), 11111111);
        }
    }
}
