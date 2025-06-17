pub fn matrix_transpose<T>(matrix: &mut [T], n: usize) {
    for i in 0..n {
        for j in 0..i {
            matrix.swap(i * n + j, j * n + i);
        }
    }
}

fn matrix_transpose_cache_oblivious_rec<T>(matrix: *mut T, n: usize, tot_n: usize) {
    if n <= 32 {
        for i in 0..n {
            for j in 0..i {
                unsafe {
                    std::ptr::swap(matrix.add(i * tot_n + j), matrix.add(j * tot_n + i));
                }
            }
        }
    } else {
        let mid = n / 2;
        unsafe {
            matrix_transpose_cache_oblivious_rec(matrix, mid, tot_n);
            matrix_transpose_cache_oblivious_rec(matrix.add(mid), mid, tot_n);
            matrix_transpose_cache_oblivious_rec(matrix.add(mid * tot_n), mid, tot_n);
            matrix_transpose_cache_oblivious_rec(matrix.add(mid * tot_n + mid), mid, tot_n);
        }
        for i in 0..mid {
            for j in 0..mid {
                unsafe {
                    std::ptr::swap(
                        matrix.add(i * tot_n + j + mid),
                        matrix.add((i + mid) * tot_n + j),
                    );
                }
            }
        }
        if n % 2 == 1 {
            let last_row = n - 1;
            for i in 0..n - 1 {
                unsafe {
                    std::ptr::swap(
                        matrix.add(last_row * tot_n + i),
                        matrix.add(i * tot_n + last_row),
                    );
                }
            }
        }
    }
}

pub fn matrix_transpose_cache_oblivious<T>(matrix: &mut [T], n: usize) {
    matrix_transpose_cache_oblivious_rec(matrix.as_mut_ptr(), n, n);
}

fn matrix_transpose_cache_oblivious_fast_rec<T>(
    matrix: *mut T,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    tot_n: usize,
) {
    if x1 - x0 <= 16 && y1 - y0 <= 16 {
        for i in x0..x1 {
            for j in y0..y1.min(i) {
                unsafe {
                    std::ptr::swap(matrix.add(i * tot_n + j), matrix.add(j * tot_n + i));
                }
            }
        }
        return;
    }
    if (x1 - x0) > (y1 - y0) {
        let mid_x = x0.midpoint(x1);
        matrix_transpose_cache_oblivious_fast_rec(matrix, x0, y0, mid_x, y1, tot_n);
        matrix_transpose_cache_oblivious_fast_rec(matrix, mid_x, y0, x1, y1, tot_n);
        return;
    }

    let mid_y = y0.midpoint(y1);
    matrix_transpose_cache_oblivious_fast_rec(matrix, x0, y0, x1, mid_y, tot_n);
    matrix_transpose_cache_oblivious_fast_rec(matrix, x0, mid_y, x1, y1, tot_n);
}

pub fn matrix_transpose_cache_oblivious_fast<T>(matrix: &mut [T], n: usize) {
    matrix_transpose_cache_oblivious_fast_rec(matrix.as_mut_ptr(), 0, 0, n, n, n);
}

pub struct Matrix<T> {
    data: Vec<T>,
    n: usize,
    m: usize,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(n: usize, m: usize) -> Self {
        let size = n * m;
        let data = vec![T::default(); size];
        Matrix { data, n, m }
    }

    pub fn from_vec(data: Vec<T>, n: usize, m: usize) -> Self {
        assert_eq!(data.len(), n * m);
        Matrix { data, n, m }
    }

    fn transpose_to_rec(&self, target: &mut Matrix<T>, x0: usize, y0: usize, x1: usize, y1: usize) {
        if x1 - x0 <= 16 && y1 - y0 <= 16 {
            for i in x0..x1 {
                for j in y0..y1 {
                    target.set(j, i, self.get(i, j).clone());
                }
            }
            return;
        }
        if (x1 - x0) > (y1 - y0) {
            let mid_x = x0.midpoint(x1);
            self.transpose_to_rec(target, x0, y0, mid_x, y1);
            self.transpose_to_rec(target, mid_x, y0, x1, y1);
            return;
        }

        let mid_y = y0.midpoint(y1);
        self.transpose_to_rec(target, x0, y0, x1, mid_y);
        self.transpose_to_rec(target, x0, mid_y, x1, y1);
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut target = Matrix::new(self.m, self.n);
        self.transpose_to_rec(&mut target, 0, 0, self.n, self.m);
        target
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.data[i * self.m + j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.data[i * self.m + j] = value;
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.data[i * self.m + j]
    }
}

impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Default + Clone> Matrix<T> {
    pub fn simple_mul(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.m, other.n);
        let mut result = Matrix::new(self.n, other.m);
        for i in 0..self.n {
            for j in 0..other.m {
                let mut sum = T::default();
                for k in 0..self.m {
                    sum = sum + self.get(i, k).clone() * other.get(k, j).clone();
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    pub fn transpose_mul(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.m, other.n);
        let other_transposed = other.transpose();
        let mut result = Matrix::new(self.n, other.m);
        for i in 0..self.n {
            for j in 0..other.m {
                let mut sum = T::default();
                for k in 0..self.m {
                    sum = sum + self.get(i, k).clone() * other_transposed.get(j, k).clone();
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    #[rustfmt::skip]
    fn mul_rec(
        &self,
        ax0: usize,
        ax1: usize,
        ay0: usize,
        ay1: usize,
        other: &Matrix<T>,
        by0: usize,
        by1: usize,
        result: &mut Matrix<T>,
        cx0: usize,
        cx1: usize,
    ) {
        if ax1 - ax0 <= 16 && ay1 - ay0 <= 16 && by1 - by0 <= 16 {
            for i in ax0..ax1 {
                for j in by0..by1 {
                    let mut sum = result.get(i, j).clone();
                    for k in ay0..ay1 {
                        sum = sum + self.get(i, k).clone() * other.get(k, j).clone();
                    }
                    result.set(i, j, sum);
                }
            }
            return;
        }

        if ay1 - ay0 > 16 {
            let mid_y = ay0.midpoint(ay1);
            self.mul_rec(ax0, ax1, ay0, mid_y, other, by0, by1, result, cx0, cx1);
            self.mul_rec(ax0, ax1, mid_y, ay1, other, by0, by1, result, cx0, cx1);
            return;
        }

        if ax1 - ax0 > 16 {
            let mid_x = ax0.midpoint(ax1);
            self.mul_rec(ax0, mid_x, ay0, ay1, other, by0, by1, result, cx0, cx1);
            self.mul_rec(mid_x, ax1, ay0, ay1, other, by0, by1, result, cx0, cx1);
            return;
        }
        // if by1 - by0 > 16 {
        let mid_x = cx0.midpoint(cx1);
        self.mul_rec(ax0, ax1, ay0, ay1, other, by0, mid_x, result, cx0, mid_x);
        self.mul_rec(ax0, ax1, ay0, ay1, other, mid_x, by1, result, mid_x, cx1);
    }

    pub fn mul(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.m, other.n);
        let mut result = Matrix::new(self.n, other.m);
        self.mul_rec(
            0,
            self.n,
            0,
            self.m,
            other,
            0,
            other.m,
            &mut result,
            0,
            other.m,
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distr::{Distribution, Uniform};

    const N: usize = 100;
    #[test]
    fn test_matrix_transpose() {
        let mut rng = rand::rng();
        let uniform = Uniform::new(0, 100).unwrap();
        let mut matrix: Vec<i32> = uniform.sample_iter(&mut rng).take(N * N).collect();
        let original_matrix = matrix.clone();
        matrix_transpose(&mut matrix, N);
        for i in 0..N {
            for j in 0..N {
                assert_eq!(matrix[i * N + j], original_matrix[j * N + i]);
            }
        }
    }

    #[test]
    fn test_matrix_transpose_cache_oblivious() {
        let mut rng = rand::rng();
        let uniform = Uniform::new(0, 100).unwrap();
        let mut matrix: Vec<i32> = uniform.sample_iter(&mut rng).take(N * N).collect();
        let original_matrix = matrix.clone();
        matrix_transpose_cache_oblivious(&mut matrix, N);
        for i in 0..N {
            for j in 0..N {
                assert_eq!(matrix[i * N + j], original_matrix[j * N + i]);
            }
        }
    }

    #[test]
    fn test_matrix_transpose_cache_oblivious_fast() {
        let mut rng = rand::rng();
        let uniform = Uniform::new(0, 100).unwrap();
        let mut matrix: Vec<i32> = uniform.sample_iter(&mut rng).take(N * N).collect();
        let original_matrix = matrix.clone();
        matrix_transpose_cache_oblivious_fast(&mut matrix, N);
        for i in 0..N {
            for j in 0..N {
                assert_eq!(matrix[i * N + j], original_matrix[j * N + i]);
            }
        }
    }

    #[test]
    fn test_matrix_transpose_struct() {
        let mut rng = rand::rng();
        let uniform = Uniform::new(0, 100).unwrap();
        let mut matrix = Matrix::new(N, N);
        for i in 0..N {
            for j in 0..N {
                matrix.set(i, j, uniform.sample(&mut rng));
            }
        }
        let transposed_matrix = matrix.transpose();
        for i in 0..N {
            for j in 0..N {
                assert_eq!(transposed_matrix.get(j, i), matrix.get(i, j));
            }
        }
    }

    #[test]
    fn test_matrix_mul() {
        let mut rng = rand::rng();
        let uniform = Uniform::new(0, 100).unwrap();
        let matrix_a = Matrix::from_vec(uniform.sample_iter(&mut rng).take(N * N).collect(), N, N);
        let matrix_b = Matrix::from_vec(uniform.sample_iter(&mut rng).take(N * N).collect(), N, N);
        let simple_result = matrix_a.simple_mul(&matrix_b);
        let transposed_result = matrix_a.transpose_mul(&matrix_b);
        let result = matrix_a.mul(&matrix_b);
        for i in 0..N {
            for j in 0..N {
                assert_eq!(simple_result.get(i, j), transposed_result.get(i, j));
                assert_eq!(simple_result.get(i, j), result.get(i, j));
            }
        }
    }
}
