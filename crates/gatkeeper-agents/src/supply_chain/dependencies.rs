use gatkeeper_core::{Finding, Severity};
use std::path::Path;

pub fn check_dependencies(file: &Path, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let file_str = file.to_string_lossy();

    // Only check dependency manifest files
    let is_manifest = file_str.ends_with("Cargo.toml")
        || file_str.ends_with("package.json")
        || file_str.ends_with("requirements.txt")
        || file_str.ends_with("go.mod");

    if !is_manifest {
        return findings;
    }

    // Phase 2: Integrate with OSV/NVD API for real CVE checking
    // Phase 1: Basic pattern detection

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;

        // Check for git dependencies (potentially unstable)
        if line.contains("git = ") || line.contains("\"git\":") {
            findings.push(Finding {
                id: format!("GAT-Supply-{:04}", findings.len() + 1),
                agent: "SupplyChainAgent".to_string(),
                severity: Severity::Medium,
                file: file_str.to_string(),
                lines: vec![line_num],
                finding_type: "git_dependency".to_string(),
                cvss_score: Some(4.0),
                description: "Git-based dependency detected (not from package registry)".to_string(),
                correction: Some("Prefer published versions from the official registry".to_string()),
                effort_correction: Some("15 minutes".to_string()),
            });
        }

        // Check for wildcard versions
        if line.contains("*") || line.contains("latest") {
            findings.push(Finding {
                id: format!("GAT-Supply-{:04}", findings.len() + 1),
                agent: "SupplyChainAgent".to_string(),
                severity: Severity::High,
                file: file_str.to_string(),
                lines: vec![line_num],
                finding_type: "wildcard_version".to_string(),
                cvss_score: Some(6.5),
                description: "Wildcard version dependency — may pull untested versions".to_string(),
                correction: Some("Pin to a specific version range".to_string()),
                effort_correction: Some("5 minutes".to_string()),
            });
        }
    }

    findings
}
