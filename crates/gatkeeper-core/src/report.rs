use crate::finding::Finding;
use crate::risk::RiskScore;
use crate::scan::ScanResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatReport {
    pub session: SessionInfo,
    pub risk_score: RiskScore,
    pub findings: Vec<Finding>,
    pub subconscious: Option<SubconsciousInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub organisation: String,
    pub project: String,
    pub version: String,
    pub timestamp: String,
    pub agents_used: Vec<String>,
    pub lines_analyzed: u64,
    pub scenarios_sub: u32,
    pub duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubconsciousInfo {
    pub dreams: Vec<Dream>,
    pub intuitions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dream {
    pub id: String,
    pub couche: String,
    pub decouverte_a: String,
    pub horizon: String,
    pub probabilite: f64,
    pub impact: String,
    pub condition: String,
}

impl GatReport {
    pub fn from_scan(scan: &ScanResult, subconscious: Option<SubconsciousInfo>) -> Self {
        Self {
            session: SessionInfo {
                organisation: "Unknown".to_string(),
                project: scan.project_name.clone(),
                version: "0.1.0".to_string(),
                timestamp: scan.timestamp.clone(),
                agents_used: scan.agents_used.clone(),
                lines_analyzed: scan.lines_analyzed,
                scenarios_sub: 0,
                duration: format!("{}ms", scan.duration_ms),
            },
            risk_score: scan.risk_score.clone(),
            findings: scan.findings.clone(),
            subconscious,
        }
    }

    pub fn to_toml(&self) -> String {
        toml::to_string_pretty(self).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn finding_counts(&self) -> (usize, usize, usize, usize, usize) {
        use crate::finding::Severity;
        let c = self.findings.iter().filter(|f| f.severity == Severity::Critical).count();
        let h = self.findings.iter().filter(|f| f.severity == Severity::High).count();
        let m = self.findings.iter().filter(|f| f.severity == Severity::Medium).count();
        let l = self.findings.iter().filter(|f| f.severity == Severity::Low).count();
        let i = self.findings.iter().filter(|f| f.severity == Severity::Info).count();
        (c, h, m, l, i)
    }
}
