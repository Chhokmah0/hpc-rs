pub const fn exgcd_rec(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = exgcd_rec(b % a, a);
        (gcd, y1 - (b / a) * x1, x1)
    }
}

pub const fn exgcd_iter(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        old_r -= q * r;
        old_s -= q * s;
        old_t -= q * t;
        std::mem::swap(&mut old_r, &mut r);
        std::mem::swap(&mut old_s, &mut s);
        std::mem::swap(&mut old_t, &mut t);
        // let (new_r, new_s, new_t) = (old_r - q * r, old_s - q * s, old_t - q * t);
        // (old_r, r) = (r, new_r);
        // (old_s, s) = (s, new_s);
        // (old_t, t) = (t, new_t);
    }
    (old_r, old_s, old_t)
}

pub const fn exgcd_rec_i64(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = exgcd_rec_i64(b % a, a);
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

pub const fn inverse_exgcd_iter<const M: i32>(a: i32) -> Option<i32> {
    let (mut old_r, mut r) = (M, a);
    let (mut old_t, mut t) = (0, 1);

    while r > 1 {
        let q = old_r / r;
        old_r -= q * r;
        old_t -= q * t;
        std::mem::swap(&mut r, &mut old_r);
        std::mem::swap(&mut t, &mut old_t);
        // let (new_r, new_t) = (old_r - q * r, old_t - q * t);
        // (old_r, r) = (r, new_r);
        // (old_t, t) = (t, new_t);
    }
    if r == 1 {
        if t < 0 {
            t += M;
        }
        Some(t)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_exgcd() {
        let inv = inverse_exgcd_rec::<7>(3).unwrap();
        assert_eq!(inv, 5);

        let inv = inverse_exgcd_iter::<7>(3).unwrap();
        assert_eq!(inv, 5);

        let inv = inverse_exgcd_rec::<7>(4);
        assert_eq!(inv, Some(2));

        let inv = inverse_exgcd_iter::<7>(4);
        assert_eq!(inv, Some(2));
    }

    #[test]
    fn test_exgcd() {
        let (gcd, x, y) = exgcd_rec(30, 12);
        assert_eq!(gcd, x * 30 + y * 12);

        let (gcd, x, y) = exgcd_iter(30, 12);
        assert_eq!(gcd, x * 30 + y * 12);

        let (gcd, x, y) = exgcd_rec_i64(30, 12);
        assert_eq!(gcd, x * 30 + y * 12);
    }
}
