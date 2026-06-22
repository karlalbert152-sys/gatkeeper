use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub project_name: String,
    pub timestamp: String,
    pub duration_ms: u64,
    pub files_analyzed: u32,
    pub lines_analyzed: u64,
    pub agents_used: Vec<String>,
    pub findings: Vec<crate::finding::Finding>,
    pub risk_score: crate::risk::RiskScore,
}

impl ScanResult {
    pub fn summary(&self) -> String {
        let critical = self
            .findings
            .iter()
            .filter(|f| f.severity == crate::finding::Severity::Critical)
            .count();
        let high = self
            .findings
            .iter()
            .filter(|f| f.severity == crate::finding::Severity::High)
            .count();
        let medium = self
            .findings
            .iter()
            .filter(|f| f.severity == crate::finding::Severity::Medium)
            .count();
        let low = self
            .findings
            .iter()
            .filter(|f| f.severity == crate::finding::Severity::Low)
            .count();

        format!(
            "Scan complete: {} files, {} lines, {} findings ({} critical, {} high, {} medium, {} low) | Risk: {}/100",
            self.files_analyzed,
            self.lines_analyzed,
            self.findings.len(),
            critical,
            high,
            medium,
            low,
            self.risk_score.global,
        )
    }
}
