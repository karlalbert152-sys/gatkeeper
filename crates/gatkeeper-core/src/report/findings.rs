use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Severity::Critical => "\x1b[31m",
            Severity::High => "\x1b[33m",
            Severity::Medium => "\x1b[36m",
            Severity::Low => "\x1b[32m",
            Severity::Info => "\x1b[37m",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub agent: String,
    pub severity: Severity,
    pub file: String,
    pub lines: Vec<usize>,
    pub finding_type: String,
    pub cvss_score: Option<f64>,
    pub description: String,
    pub correction: Option<String>,
    pub effort_correction: Option<String>,
}
