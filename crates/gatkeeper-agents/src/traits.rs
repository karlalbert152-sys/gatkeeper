use async_trait::async_trait;
use gatkeeper_core::{Finding, Severity};
use std::path::Path;

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn analyze(&self, file: &Path, source: &str) -> Vec<Finding>;
}

pub fn severity_from_cvss(score: f64) -> Severity {
    if score >= 9.0 {
        Severity::Critical
    } else if score >= 7.0 {
        Severity::High
    } else if score >= 4.0 {
        Severity::Medium
    } else if score >= 0.1 {
        Severity::Low
    } else {
        Severity::Info
    }
}
