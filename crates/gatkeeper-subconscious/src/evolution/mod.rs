pub mod generator;

pub struct CodeEvolution {
    pub original_function: String,
    pub variants: Vec<String>,
    pub benchmarks: Vec<BenchmarkResult>,
}

pub struct BenchmarkResult {
    pub variant_name: String,
    pub throughput_improvement: f64,
    pub memory_reduction: f64,
    pub safety_improvement: f64,
}
