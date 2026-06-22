pub mod compliance_agent;
pub mod logic_agent;
pub mod performance_agent;
pub mod secret_agent;
pub mod security_agent;
pub mod supply_chain_agent;

use gatkeeper_core::finding::Finding;
use gatkeeper_parser::ParsedFile;

pub trait Agent {
    fn name(&self) -> &str;
    fn analyze(&self, files: &[ParsedFile]) -> Vec<Finding>;
}

pub fn all_agents() -> Vec<Box<dyn Agent>> {
    vec![
        Box::new(security_agent::SecurityAgent),
        Box::new(logic_agent::LogicAgent),
        Box::new(performance_agent::PerformanceAgent),
        Box::new(compliance_agent::ComplianceAgent),
        Box::new(secret_agent::SecretAgent),
        Box::new(supply_chain_agent::SupplyChainAgent),
    ]
}
