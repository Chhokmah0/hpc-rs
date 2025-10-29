pub fn sum(a: &[i32]) -> i32 {
    let mut result = 0;
    for &x in a {
        result += x;
    }
    result
}
