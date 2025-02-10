use hpc_rs::profiling::statistical_profiling;

fn main() {
    let s = statistical_profiling::statistical_profiling_query();
    println!("{}", s);
}
