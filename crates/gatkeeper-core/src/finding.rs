use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn score(&self) -> u32 {
        match self {
            Severity::Critical => 100,
            Severity::High => 75,
            Severity::Medium => 50,
            Severity::Low => 25,
            Severity::Info => 10,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub agent: String,
    pub severity: Severity,
    pub file: String,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub finding_type: String,
    pub cvss_score: Option<f64>,
    pub description: String,
    pub correction: Option<String>,
    pub effort_correction: Option<String>,
}

impl Finding {
    pub fn new(
        agent: &str,
        severity: Severity,
        file: &str,
        finding_type: &str,
        description: &str,
    ) -> Self {
        let id = format!(
            "GAT-{}-{:04}",
            severity.label(),
            rand_id()
        );
        Self {
            id,
            agent: agent.to_string(),
            severity,
            file: file.to_string(),
            line_start: None,
            line_end: None,
            finding_type: finding_type.to_string(),
            cvss_score: None,
            description: description.to_string(),
            correction: None,
            effort_correction: None,
        }
    }

    pub fn with_lines(mut self, start: u32, end: u32) -> Self {
        self.line_start = Some(start);
        self.line_end = Some(end);
        self
    }

    pub fn with_cvss(mut self, score: f64) -> Self {
        self.cvss_score = Some(score);
        self
    }

    pub fn with_correction(mut self, correction: &str, effort: &str) -> Self {
        self.correction = Some(correction.to_string());
        self.effort_correction = Some(effort.to_string());
        self
    }
}

fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    (t % 9999) as u32
}
