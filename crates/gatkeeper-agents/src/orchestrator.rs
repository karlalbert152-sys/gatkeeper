use crate::traits::Agent;
use crate::context::AnalysisContext;
use gatkeeper_core::Finding;
use std::path::Path;
use tracing::{info, warn};

pub struct AgentOrchestrator {
    agents: Vec<Box<dyn Agent>>,
}

impl AgentOrchestrator {
    pub fn new() -> Self {
        let agents: Vec<Box<dyn Agent>> = vec![
            Box::new(crate::security::SecurityAgent::new()),
            Box::new(crate::logic::LogicAgent::new()),
            Box::new(crate::performance::PerformanceAgent::new()),
            Box::new(crate::compliance::ComplianceAgent::new()),
            Box::new(crate::secrets::SecretAgent::new()),
            Box::new(crate::supply_chain::SupplyChainAgent::new()),
        ];

        Self { agents }
    }

    pub fn agent_names(&self) -> Vec<&str> {
        self.agents.iter().map(|a| a.name()).collect()
    }

    pub async fn run_agent(&self, agent_name: &str, file: &Path, source: &str) -> Vec<Finding> {
        if let Some(agent) = self.agents.iter().find(|a| a.name() == agent_name) {
            info!("Running {} on {}", agent_name, file.display());
            agent.analyze(file, source).await
        } else {
            warn!("Unknown agent: {}", agent_name);
            Vec::new()
        }
    }

    pub async fn run_all(&self, file: &Path, source: &str) -> Vec<Finding> {
        let mut all_findings = Vec::new();
        for agent in &self.agents {
            let findings = agent.analyze(file, source).await;
            all_findings.extend(findings);
        }
        all_findings
    }
}
