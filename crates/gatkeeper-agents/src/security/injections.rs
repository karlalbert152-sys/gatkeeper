use gatkeeper_core::{Finding, Severity};
use std::path::Path;

pub fn check_sql_injections(file: &Path, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let file_str = file.to_string_lossy();

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;

        // Check for string formatting in SQL queries
        if (line.contains("format!") || line.contains("concat!") || line.contains("+"))
            && (line.to_lowercase().contains("select")
                || line.to_lowercase().contains("insert")
                || line.to_lowercase().contains("update")
                || line.to_lowercase().contains("delete"))
        {
            findings.push(Finding {
                id: format!("GAT-SEC-{:04}", findings.len() + 1),
                agent: "SecurityAgent".to_string(),
                severity: Severity::Critical,
                file: file_str.to_string(),
                lines: vec![line_num],
                finding_type: "sql_injection".to_string(),
                cvss_score: Some(9.8),
                description: "Potential SQL injection via string formatting in query".to_string(),
                correction: Some("Use parameterized queries instead of string concatenation".to_string()),
                effort_correction: Some("30 minutes".to_string()),
            });
        }

        // Check for unsanitized user input in queries
        if line.contains("req.") && line.to_lowercase().contains("query") {
            findings.push(Finding {
                id: format!("GAT-SEC-{:04}", findings.len() + 1),
                agent: "SecurityAgent".to_string(),
                severity: Severity::High,
                file: file_str.to_string(),
                lines: vec![line_num],
                finding_type: "sql_injection_unsanitized".to_string(),
                cvss_score: Some(8.5),
                description: "Direct use of request data in SQL query without sanitization".to_string(),
                correction: Some("Sanitize input using a validation library".to_string()),
                effort_correction: Some("45 minutes".to_string()),
            });
        }
    }

    findings
}
