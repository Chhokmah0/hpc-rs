use rand::distributions::{Distribution, Uniform};

pub fn matmul_test() {
    const N: usize = 1024;
    let mut rng = rand::thread_rng();
    let uniform = Uniform::new(0.0, 1.0);

    let mut a = vec![vec![0.0; N]; N];
    let mut b = vec![vec![0.0; N]; N];
    for i in 0..N {
        for j in 0..N {
            a[i][j] = uniform.sample(&mut rng);
            b[i][j] = uniform.sample(&mut rng);
        }
    }

    let mut c = vec![vec![0.0; N]; N];
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

#[cfg(test)]
mod test {
    /// This took 2.33s on my computer.\
    /// The same c++ code(using vector<double>) took 0.833s.
    /// 
    /// And I don't know what caused it.
    #[test]
    #[ignore = "This took 2.33s on my computer."]
    fn matmul_test() {
        super::matmul_test();
    }
}
