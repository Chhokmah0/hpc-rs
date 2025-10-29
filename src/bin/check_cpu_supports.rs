use std::arch;

fn main() {
    println!("Checking CPU support for SIMD instructions...");
    println!("SSE: {}", arch::is_x86_feature_detected!("sse"));
    println!("SSE2: {}", arch::is_x86_feature_detected!("sse2"));
    println!("AVX: {}", arch::is_x86_feature_detected!("avx"));
    println!("AVX2: {}", arch::is_x86_feature_detected!("avx2"));
    println!("AVX512: {}", arch::is_x86_feature_detected!("avx512f"));
}
