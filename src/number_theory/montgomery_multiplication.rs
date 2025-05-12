use super::exgcd;
use exgcd::inverse_exgcd_iter;

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

// TODO: Implement Montgomery multiplication
