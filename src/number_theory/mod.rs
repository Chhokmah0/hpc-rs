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

pub const fn inverse<const M: u64>(mut _a: u64) -> u64 {
    let mut result = 1;
    seq_macro::seq!(N in 0..=30 {
        if (M - 2) & (1 << N) != 0 {
            result = (result * _a) % M;
        }
        _a = (_a * _a) % M;
    });
    result
}

pub const fn exgcd_rec(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = exgcd_rec(b % a, a);
        (gcd, y1 - (b / a) * x1, x1)
    }
}

pub const fn inverse_exgcd_rec<const M: i32>(a: i32) -> Option<i32> {
    let (gcd, x, _) = exgcd_rec(a, M);
    if gcd == 1 {
        Some((x % M + M) % M)
    } else {
        None
    }
}

pub const fn inverse_exgcd_iter<const M: i32>(mut a: i32) -> Option<i32> {
    let mut b = M;
    let mut x = 1;
    let mut y = 0;
    while a > 1 {
        y -= (b / a) * x;
        b %= a;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut x, &mut y);
    }
    if b == 1 {
        Some((x % M + M) % M)
    } else {
        None
    }
}

pub fn slow_sum<const M: i32>(a: Vec<i32>) -> i32 {
    let mut sum = 0;
    for val in a {
        sum += val;
        sum %= M;
    }
    sum
}

pub fn fast_sum<const M: i32>(a: Vec<i32>) -> i32 {
    let mut sum = 0;
    for val in a {
        sum += val;
        if sum >= M {
            sum -= M;
        }
    }
    sum
}

pub fn faster_sum<const M: i32>(a: Vec<i32>) -> i32 {
    let mut sum = 0i64;
    for val in a {
        sum += val as i64;
    }
    (sum % M as i64) as i32
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

    #[test]
    fn test_inverse() {
        assert_eq!(inverse::<1000000007>(2), 500000004);
        assert_eq!(inverse::<1000000007>(3), 333333336);
        assert_eq!(inverse::<1000000007>(5), 400000003);
    }

    #[test]
    fn test_inverse_exgcd_rec() {
        assert_eq!(inverse_exgcd_rec::<1000000007>(2), Some(500000004));
        assert_eq!(inverse_exgcd_rec::<1000000007>(3), Some(333333336));
        assert_eq!(inverse_exgcd_rec::<1000000007>(5), Some(400000003));
        assert_eq!(inverse_exgcd_rec::<1000000006>(6), None);
    }
}
