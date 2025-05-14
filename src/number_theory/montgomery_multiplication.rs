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

#[derive(Debug, Clone, Copy)]
pub struct Montgomery {
    n: u32,
    inv_n: u32,
    sqr_r: u32,
}
#[derive(Debug, Clone, Copy)]
pub struct MontgomerySpace {
    x: u32,
}

impl Montgomery {
    #[inline]
    /// create a new Montgomery instance
    /// `n` must be coprime to 2^32
    /// i32 make x + x will not exceed 2^32
    pub const fn new(n: i32) -> Self {
        if n % 2 == 0 {
            panic!("n is not coprime to 2^32");
        }
        let n = n as u32;
        let mut inv_n = 1u32; // 2^1
        inv_n = inv_n.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(inv_n))); // 2^2
        inv_n = inv_n.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(inv_n))); // 2^4
        inv_n = inv_n.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(inv_n))); // 2^8
        inv_n = inv_n.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(inv_n))); // 2^16
        inv_n = inv_n.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(inv_n))); // 2^32

        // compute (1 << 64) % n, result is in [1, n]
        let sqr_r = (0xffff_ffff_ffff_ffffu64 % n as u64) as u32 + 1;
        Montgomery { n, inv_n, sqr_r }
    }

    #[inline]
    /// times r^(-1) (mod n)
    /// returns a number in the [0, 2 * n - 2] range
    fn reduce(&self, x: u64) -> MontgomerySpace {
        let q = (x as u32 as u64 * self.inv_n as u64) as u32; // x * n^(-1) mod R
        let m = ((q as u64 * self.n as u64) >> 32) as u32; // q * n / R

        // returns a number in the [0, 2 * n - 2] range
        MontgomerySpace {
            x: (x >> 32) as u32 + (self.n - m),
        }
    }

    /// convert montgomery space to normal space
    /// returns a number in the (0, n] range
    #[inline]
    pub fn from(&self, m: MontgomerySpace) -> u32 {
        self.reduce(m.x as u64).x
    }

    /// convert normal space to montgomery space
    #[inline]
    pub fn to(&self, x: u32) -> MontgomerySpace {
        // let mut x = ((x as i64) * self.r as i64) % self.n as i64;
        // if x < 0 {
        //     x += self.n as i64;
        // }
        // MontgomerySpace { x: x as u32 }
        // or
        self.reduce(x as u64 * self.sqr_r as u64)
    }

    /// multiply two numbers in montgomery space
    #[inline]
    pub fn mul(&self, lhs: MontgomerySpace, rhs: MontgomerySpace) -> MontgomerySpace {
        let x = lhs.x as u64 * rhs.x as u64;
        self.reduce(x)
    }
}

pub fn inverse_using_montgomery(base: i32, mod_val: i32) -> i32 {
    let montgomery = Montgomery::new(mod_val);
    let mut result = montgomery.to(1);
    let mut base = montgomery.to(base as u32);
    let mut exp = mod_val - 2;

    while exp > 0 {
        if exp & 1 == 1 {
            result = montgomery.mul(result, base);
        }
        base = montgomery.mul(base, base);
        exp >>= 1;
    }
    montgomery.from(result) as i32
}

pub fn inverse_with_montgomery(base: i32, montgomery: &Montgomery) -> i32 {
    let mut result = montgomery.to(1);
    let mut base = montgomery.to(base as u32);
    let mut exp = montgomery.n - 2;

    while exp > 0 {
        if exp & 1 == 1 {
            result = montgomery.mul(result, base);
        }
        base = montgomery.mul(base, base);
        exp >>= 1;
    }
    montgomery.from(result) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_montgomery() {
        let montgomery = Montgomery::new(1_000_000_007);
        let a = montgomery.to(2);
        let b = montgomery.to(3);
        assert_eq!(montgomery.from(montgomery.mul(a, b)), 6);
    }

    #[test]
    fn test_inverse_with_montgomery() {
        assert_eq!(inverse_using_montgomery(2, 1_000_000_007), 500000004);
        assert_eq!(inverse_using_montgomery(3, 1_000_000_007), 333333336);
        assert_eq!(inverse_using_montgomery(4, 1_000_000_007), 250000002);
    }
}
