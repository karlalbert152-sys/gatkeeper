use gatkeeper_core::{Finding, Severity};
use std::path::Path;

const SECRET_PATTERNS: &[(&str, &str, f64)] = &[
    ("api_key", "API key", 8.5),
    ("apikey", "API key", 8.5),
    ("secret_key", "Secret key", 8.5),
    ("password", "Password", 9.0),
    ("passwd", "Password", 9.0),
    ("token", "Token", 7.5),
    ("jwt", "JWT token", 8.0),
    ("private_key", "Private key", 9.5),
    ("access_key", "Access key", 8.0),
    ("aws_secret", "AWS secret", 9.0),
];

pub fn detect_secrets(file: &Path, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let file_str = file.to_string_lossy();

    // Skip test files and example files
    if file_str.contains("test") || file_str.contains("example") || file_str.contains("fixture") {
        return findings;
    }

    for (i, line) in source.lines().enumerate() {
        let line_num = i + 1;
        let lower = line.to_lowercase();

        for (pattern, name, cvss) in SECRET_PATTERNS {
            if lower.contains(pattern) {
                // Check if it's assigned a literal value (not env/config)
                if (line.contains("= \"") || line.contains("= '") || line.contains(": \""))
                    && !line.contains("env")
                    && !line.contains("getenv")
                    && !line.contains("config")
                    && !line.contains("example")
                {
                    findings.push(Finding {
                        id: format!("GAT-SECRET-{:04}", findings.len() + 1),
                        agent: "SecretAgent".to_string(),
                        severity: Severity::Critical,
                        file: file_str.to_string(),
                        lines: vec![line_num],
                        finding_type: "hardcoded_secret".to_string(),
                        cvss_score: Some(*cvss),
                        description: format!("Hardcoded {} detected", name),
                        correction: Some("Move to environment variable or secrets manager".to_string()),
                        effort_correction: Some("15 minutes".to_string()),
                    });
                }
            }
        }
    }

    findings
}
