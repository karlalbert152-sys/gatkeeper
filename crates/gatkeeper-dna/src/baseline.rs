use crate::fingerprint::DnaFingerprint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Baseline {
    pub project_name: String,
    pub root_hash: String,
    pub file_count: u32,
    pub total_lines: u64,
    pub established_at: String,
    pub findings_baseline: Vec<String>,
}

impl Baseline {
    pub fn establish(project_name: &str, fingerprint: &DnaFingerprint) -> Self {
        Self {
            project_name: project_name.to_string(),
            root_hash: fingerprint.root_hash.clone(),
            file_count: fingerprint.file_count,
            total_lines: fingerprint.total_lines,
            established_at: chrono::Utc::now().to_rfc3339(),
            findings_baseline: Vec::new(),
        }
    }

    pub fn compare(&self, current: &DnaFingerprint) -> BaselineComparison {
        let hash_changed = self.root_hash != current.root_hash;
        let files_delta = current.file_count as i64 - self.file_count as i64;
        let lines_delta = current.total_lines as i64 - self.total_lines as i64;

        BaselineComparison {
            hash_changed,
            files_delta,
            lines_delta,
            drift_detected: hash_changed && (files_delta.abs() > 10 || lines_delta.abs() > 100),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BaselineComparison {
    pub hash_changed: bool,
    pub files_delta: i64,
    pub lines_delta: i64,
    pub drift_detected: bool,
}
