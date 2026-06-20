use gatkeeper_core::{Finding, Severity};
use std::path::Path;

pub fn check_owasp_patterns(file: &Path, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let file_str = file.to_string_lossy();

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;

        // Hardcoded credentials
        if line.contains("password") && (line.contains("=") || line.contains(":")) {
            if !line.contains("env") && !line.contains("config") && !line.contains("getenv") {
                findings.push(Finding {
                    id: format!("GAT-OWASP-{:04}", findings.len() + 1),
                    agent: "SecurityAgent".to_string(),
                    severity: Severity::Critical,
                    file: file_str.to_string(),
                    lines: vec![line_num],
                    finding_type: "hardcoded_credential".to_string(),
                    cvss_score: Some(9.1),
                    description: "Potential hardcoded credential detected".to_string(),
                    correction: Some("Use environment variables or a secrets manager".to_string()),
                    effort_correction: Some("15 minutes".to_string()),
                });
            }
        }

        // Unsafe deserialization
        if line.contains("eval(") || line.contains("exec(") || line.contains("unserialize(") {
            findings.push(Finding {
                id: format!("GAT-OWASP-{:04}", findings.len() + 1),
                agent: "SecurityAgent".to_string(),
                severity: Severity::High,
                file: file_str.to_string(),
                lines: vec![line_num],
                finding_type: "unsafe_deserialization".to_string(),
                cvss_score: Some(8.0),
                description: "Unsafe deserialization / code execution detected".to_string(),
                correction: Some("Avoid eval/exec with untrusted input; use safe alternatives".to_string()),
                effort_correction: Some("1 hour".to_string()),
            });
        }

        // Missing input validation (XSS pattern)
        if line.contains("innerHTML") || line.contains("dangerouslySetInnerHTML") {
            findings.push(Finding {
                id: format!("GAT-OWASP-{:04}", findings.len() + 1),
                agent: "SecurityAgent".to_string(),
                severity: Severity::High,
                file: file_str.to_string(),
                lines: vec![line_num],
                finding_type: "xss_potential".to_string(),
                cvss_score: Some(7.5),
                description: "Potential XSS via unescaped HTML insertion".to_string(),
                correction: Some("Use textContent instead of innerHTML, or sanitize input".to_string()),
                effort_correction: Some("30 minutes".to_string()),
            });
        }
    }

    findings
}
