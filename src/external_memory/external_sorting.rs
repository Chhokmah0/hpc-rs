pub fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut iter_left = left.iter().peekable();
    let mut iter_right = right.iter().peekable();
    
    while iter_left.peek().is_some() && iter_right.peek().is_some() {
        if iter_left.peek() <= iter_right.peek() {
            merged.push(iter_left.next().unwrap().clone());
        } else {
            merged.push(iter_right.next().unwrap().clone());
        }
    }
    while iter_left.peek().is_some() {
        merged.push(iter_left.next().unwrap().clone());
    }
    while iter_right.peek().is_some() {
        merged.push(iter_right.next().unwrap().clone());
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let left = vec![1, 3, 5];
        let right = vec![2, 4, 6];
        let result = merge(&left, &right);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);

        let left = vec![1, 2, 3];
        let right = vec![4, 5, 6];
        let result = merge(&left, &right);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);

        let left: Vec<i32> = vec![];
        let right = vec![1, 2];
        let result = merge(&left, &right);
        assert_eq!(result, vec![1, 2]);
    }
}
