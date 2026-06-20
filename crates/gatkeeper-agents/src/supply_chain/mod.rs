pub mod dependencies;

pub struct SupplyChainAgent;

impl SupplyChainAgent {
    pub fn new() -> Self { Self }
}

#[async_trait::async_trait]
impl crate::traits::Agent for SupplyChainAgent {
    fn name(&self) -> &str { "SupplyChainAgent" }
    fn description(&self) -> &str {
        "Analyzes supply chain security: typosquatting, abandoned dependencies, CVEs, license conflicts"
    }
    async fn analyze(&self, file: &std::path::Path, source: &str) -> Vec<gatkeeper_core::Finding> {
        dependencies::check_dependencies(file, source)
    }
}
