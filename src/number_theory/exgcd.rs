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
