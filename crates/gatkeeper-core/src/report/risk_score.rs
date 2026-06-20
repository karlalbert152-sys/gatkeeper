use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub global: usize,
    pub security: usize,
    pub performance: usize,
    pub compliance: usize,
    pub trend: String,
}

impl Default for RiskScore {
    fn default() -> Self {
        Self {
            global: 100,
            security: 100,
            performance: 100,
            compliance: 100,
            trend: "stable".to_string(),
        }
    }
}

impl RiskScore {
    pub fn calculate(findings: &[super::Finding]) -> Self {
        let mut security = 100;
        let mut performance = 100;
        let mut compliance = 100;

        for finding in findings {
            let penalty = match finding.severity {
                super::Severity::Critical => 25,
                super::Severity::High => 15,
                super::Severity::Medium => 8,
                super::Severity::Low => 3,
                super::Severity::Info => 0,
            };

            match finding.agent.as_str() {
                "SecurityAgent" | "SecretAgent" | "SupplyChainAgent" => {
                    security = security.saturating_sub(penalty);
                }
                "PerformanceAgent" => {
                    performance = performance.saturating_sub(penalty);
                }
                "ComplianceAgent" => {
                    compliance = compliance.saturating_sub(penalty);
                }
                _ => {}
            }
        }

        let global = (security + performance + compliance) / 3;

        let trend = if global >= 80 {
            "stable"
        } else if global >= 60 {
            "down"
        } else {
            "critical"
        };

        Self {
            global,
            security,
            performance,
            compliance,
            trend: trend.to_string(),
        }
    }
}
