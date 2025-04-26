#[inline]
pub const fn cal_lemire_reduction_magic_number(x: u32) -> u64 {
    (u64::MAX / x as u64) + 1 // ceil(2^64 / n)
}

/// 取模运算 n % x
#[inline]
pub const fn lemire_reduction_mod(n: u64, x: u32, magic_number_x: u64) -> u32 {
    let lowbits = n.wrapping_mul(magic_number_x);
    ((lowbits as u128 * x as u128) >> 64) as u32
}

/// 除法运算 n / x
#[inline]
pub const fn lemire_division(n: u64, magic_number_x: u64) -> u64 {
    // hignbits
    ((n as u128 * magic_number_x as u128) >> 64) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lemire_reduction() {
        let n = 10;
        let x = 3;
        let magic_number_x = cal_lemire_reduction_magic_number(x);
        assert_eq!(lemire_reduction_mod(n, x, magic_number_x), 1);
        assert_eq!(lemire_division(n, magic_number_x), 3);

        let n = 10000;
        let x = 123;
        let magic_number_x = cal_lemire_reduction_magic_number(x);
        assert_eq!(magic_number_x, (u64::MAX / x as u64) + 1);
        assert_eq!(lemire_reduction_mod(n, x, magic_number_x), 37);
        assert_eq!(lemire_division(n, magic_number_x), 81);
    }
}