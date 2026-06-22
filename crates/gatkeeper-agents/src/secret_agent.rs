use crate::Agent;
use gatkeeper_core::finding::{Finding, Severity};
use gatkeeper_parser::ParsedFile;
use regex::Regex;

pub struct SecretAgent;

impl Agent for SecretAgent {
    fn name(&self) -> &str {
        "SecretAgent"
    }

    fn analyze(&self, files: &[ParsedFile]) -> Vec<Finding> {
        let mut findings = Vec::new();

        let patterns = vec![
            (r#"(?i)(api[_-]?key|apikey)\s*[:=]\s*["'][^"']{8,}"#, "api_key", Severity::Critical, 9.0),
            (r#"(?i)(secret|password|passwd|pwd)\s*[:=]\s*["'][^"']{4,}"#, "hardcoded_secret", Severity::Critical, 9.5),
            (r#"(?i)(token|auth)\s*[:=]\s*["'][A-Za-z0-9_\-\.]{20,}"#, "hardcoded_token", Severity::Critical, 9.0),
            (r#"(?i)bearer\s+[A-Za-z0-9_\-\.]{20,}"#, "bearer_token", Severity::High, 8.0),
            (r#"(?i)-----BEGIN\s+(RSA\s+)?PRIVATE\s+KEY-----"#, "private_key", Severity::Critical, 9.5),
            (r#"(?i)(AWS|AZURE|GCP)[_]?SECRET[_]?ACCESS[_]?KEY\s*[:=]"#, "cloud_credential", Severity::Critical, 9.5),
            (r#"(?i)jdbc:[a-z]+://[^;]*password=[^;]+"#, "database_url", Severity::High, 8.0),
            (r#"(?i)mysql://[^:]+:[^@]+@"#, "database_url", Severity::High, 8.0),
            (r#"(?i)postgres(ql)?://[^:]+:[^@]+@"#, "database_url", Severity::High, 8.0),
        ];

        for file in files {
            let source = match std::fs::read_to_string(&file.path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            if file.path.contains(".env") || file.path.contains("secret") || file.path.contains("credential") {
                findings.push(
                    Finding::new(
                        "SecretAgent",
                        Severity::High,
                        &file.path,
                        "sensitive_file",
                        "Sensitive file detected in source tree",
                    )
                    .with_cvss(7.0)
                    .with_correction("Add to .gitignore and rotate credentials", "30 minutes"),
                );
            }

            for (pattern, finding_type, severity, cvss) in &patterns {
                let re = Regex::new(pattern).unwrap();
                for (i, line) in source.lines().enumerate() {
                    let line_num = i as u32 + 1;
                    if re.is_match(line) {
                        findings.push(
                            Finding::new(
                                "SecretAgent",
                                *severity,
                                &file.path,
                                finding_type,
                                &format!("Hardcoded {} detected", finding_type.replace('_', " ")),
                            )
                            .with_lines(line_num, line_num)
                            .with_cvss(*cvss)
                            .with_correction("Move to environment variables or secret manager", "30 minutes"),
                        );
                    }
                }
            }
        }

        findings
    }
}
