pub struct PerformanceAgent;

impl PerformanceAgent {
    pub fn new() -> Self { Self }
}

#[async_trait::async_trait]
impl crate::traits::Agent for PerformanceAgent {
    fn name(&self) -> &str { "PerformanceAgent" }
    fn description(&self) -> &str {
        "Detects performance issues: memory leaks, N+1 queries, unnecessary allocations, hot paths"
    }
    async fn analyze(&self, _file: &std::path::Path, _source: &str) -> Vec<gatkeeper_core::Finding> {
        // Phase 2: Full implementation
        Vec::new()
    }
}
