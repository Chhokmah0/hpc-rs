use crate::arithmetic::*;

pub const fn binpow_rec<const M: u64>(base: u64, exp: u64) -> u64 {
    match exp {
        0 => 1,
        _ if exp % 2 == 0 => {
            let half = binpow_rec::<M>(base, exp / 2);
            (half * half) % M
        }
        _ => (base % M * binpow_rec::<M>(base, exp - 1)) % M,
    }
}

pub const fn binpow_iter<const M: u64>(mut base: u64, mut exp: u64) -> u64 {
    base %= M;
    let mut result = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % M;
        }
        base = (base * base) % M;
        exp /= 2;
    }
    result
}

pub const fn binpow_iter_with_mod(mut base: u64, mut exp: u64, mod_val: u32) -> u32 {
    let magic_number = cal_lemire_reduction_magic_number(mod_val);
    let mut result = 1;
    base %= mod_val as u64;

    while exp > 0 {
        if exp % 2 == 1 {
            result = lemire_reduction_mod(result as u64 * base, mod_val, magic_number);
        }
        base = lemire_reduction_mod(base * base, mod_val, magic_number) as u64;
        exp /= 2;
    }
    result
}

// 根据 show-asm 的结果，这里并没有被优化
pub fn inverse<const M: u64>(mut a: u64) -> u64 {
    let mut result = 1;
    let exp = M - 2;
    for i in 0..=exp.ilog2() {
        if (M >> i) & 1 == 1 {
            result = (result * a) % M;
        }
        a = (a * a) % M;
    }
    result
}

pub fn inverse_monotonic(a: u64) -> u64 {
    const M: u64 = 1_000_000_007;
    inverse::<M>(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binpow_rec() {
        assert_eq!(binpow_rec::<1000000007>(2, 10), 1024);
        assert_eq!(binpow_rec::<1000000007>(3, 5), 243);
        assert_eq!(binpow_rec::<1000000007>(5, 0), 1);
    }

    #[test]
    fn test_binpow_iter() {
        assert_eq!(binpow_iter::<1000000007>(2, 10), 1024);
        assert_eq!(binpow_iter::<1000000007>(3, 5), 243);
        assert_eq!(binpow_iter::<1000000007>(5, 0), 1);
    }

    #[test]
    fn test_binpow_iter_with_mod() {
        assert_eq!(binpow_iter_with_mod(2, 10, 1000), 24);
        assert_eq!(binpow_iter_with_mod(3, 5, 1000), 243);
        assert_eq!(binpow_iter_with_mod(5, 0, 1000), 1);
    }
}
