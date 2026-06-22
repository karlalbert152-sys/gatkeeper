use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub global: u32,
    pub security: u32,
    pub performance: u32,
    pub compliance: u32,
    pub tendency: String,
}

impl Default for RiskScore {
    fn default() -> Self {
        Self {
            global: 100,
            security: 100,
            performance: 100,
            compliance: 100,
            tendency: "stable".to_string(),
        }
    }
}

impl RiskScore {
    pub fn compute(
        findings: &[crate::finding::Finding],
        prev: Option<&RiskScore>,
    ) -> Self {
        let mut security_penalty = 0u32;
        let mut performance_penalty = 0u32;
        let mut compliance_penalty = 0u32;

        for f in findings {
            let penalty = f.severity.score();
            match f.agent.as_str() {
                "SecurityAgent" | "SecretAgent" | "SupplyChainAgent" => {
                    security_penalty = security_penalty.saturating_add(penalty);
                }
                "PerformanceAgent" => {
                    performance_penalty = performance_penalty.saturating_add(penalty);
                }
                "ComplianceAgent" => {
                    compliance_penalty = compliance_penalty.saturating_add(penalty);
                }
                "LogicAgent" => {
                    security_penalty = security_penalty.saturating_add(penalty / 2);
                    performance_penalty =
                        performance_penalty.saturating_add(penalty / 2);
                }
                _ => {}
            }
        }

        let security = 100u32.saturating_sub(security_penalty.min(100));
        let performance = 100u32.saturating_sub(performance_penalty.min(100));
        let compliance = 100u32.saturating_sub(compliance_penalty.min(100));
        let global = (security + performance + compliance) / 3;

        let tendency = if let Some(p) = prev {
            if global > p.global + 5 {
                "up".to_string()
            } else if global + 5 < p.global {
                "down".to_string()
            } else {
                "stable".to_string()
            }
        } else {
            "stable".to_string()
        };

        Self {
            global,
            security,
            performance,
            compliance,
            tendency,
        }
    }

    pub fn is_blocking(&self, threshold: u32) -> bool {
        self.global < threshold
    }
}
