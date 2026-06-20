use gatkeeper_core::{Finding, Severity};
use std::path::Path;

const WEAK_ALGORITHMS: &[&str] = &["md5", "sha1", "des", "rc4", "ecb"];

pub fn check_weak_crypto(file: &Path, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let file_str = file.to_string_lossy();

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;
        let lower = line.to_lowercase();

        for algo in WEAK_ALGORITHMS {
            if lower.contains(algo) && (lower.contains("hash") || lower.contains("digest") || lower.contains("encrypt")) {
                findings.push(Finding {
                    id: format!("GAT-SEC-{:04}", findings.len() + 1),
                    agent: "SecurityAgent".to_string(),
                    severity: if *algo == "md5" || *algo == "des" {
                        Severity::Critical
                    } else {
                        Severity::High
                    },
                    file: file_str.to_string(),
                    lines: vec![line_num],
                    finding_type: "weak_cryptography".to_string(),
                    cvss_score: Some(if *algo == "md5" { 7.5 } else { 6.5 }),
                    description: format!("Weak cryptographic algorithm detected: {}", algo.to_uppercase()),
                    correction: Some(format!("Replace {} with a modern algorithm (SHA-256, AES-256, ChaCha20)", algo)),
                    effort_correction: Some("1 hour".to_string()),
                });
            }
        }
    }

    findings
}
