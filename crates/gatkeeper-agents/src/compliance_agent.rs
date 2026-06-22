use crate::Agent;
use gatkeeper_core::finding::{Finding, Severity};
use gatkeeper_parser::ParsedFile;

pub struct ComplianceAgent;

impl Agent for ComplianceAgent {
    fn name(&self) -> &str {
        "ComplianceAgent"
    }

    fn analyze(&self, files: &[ParsedFile]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for file in files {
            let source = match std::fs::read_to_string(&file.path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            findings.extend(check_gdpr(&file.path, &source));
            findings.extend(check_logging(&file.path, &source));
            findings.extend(check_error_handling_compliance(&file.path, &source));
        }

        findings
    }
}

fn check_gdpr(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    let has_pii_fields = source.contains("email")
        || source.contains("phone")
        || source.contains("address")
        || source.contains("ssn")
        || source.contains("passport");

    let has_encryption = source.contains("encrypt")
        || source.contains("hash")
        || source.contains("bcrypt")
        || source.contains("argon2");

    if has_pii_fields && !has_encryption {
        for (i, line) in source.lines().enumerate() {
            let line_num = i as u32 + 1;
            if line.contains("email") || line.contains("phone") || line.contains("ssn") {
                findings.push(
                    Finding::new(
                        "ComplianceAgent",
                        Severity::High,
                        path,
                        "gdpr_pii_unencrypted",
                        "PII field without encryption — GDPR violation risk",
                    )
                    .with_lines(line_num, line_num)
                    .with_cvss(7.0)
                    .with_correction("Encrypt PII fields at rest and in transit", "2 hours"),
                );
                break;
            }
        }
    }

    findings
}

fn check_logging(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    let has_logging = source.contains("log::")
        || source.contains("tracing::")
        || source.contains("println!")
        || source.contains("eprintln!")
        || source.contains("console.log")
        || source.contains("logging.");

    if !has_logging && source.len() > 500 {
        findings.push(
            Finding::new(
                "ComplianceAgent",
                Severity::Medium,
                path,
                "no_logging",
                "No logging framework detected in significant source file",
            )
            .with_cvss(3.0)
            .with_correction("Add structured logging for audit trail", "30 minutes"),
        );
    }

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("password") && (line.contains("log") || line.contains("print") || line.contains("debug")) {
            findings.push(
                Finding::new(
                    "ComplianceAgent",
                    Severity::Critical,
                    path,
                    "credential_logging",
                    "Password/credential logged in plaintext",
                )
                .with_lines(line_num, line_num)
                .with_cvss(9.0)
                .with_correction("Never log credentials, use redacted fields", "15 minutes"),
            );
        }
    }

    findings
}

fn check_error_handling_compliance(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("catch") && line.contains("{") && line.contains("}") {
            let next_lines: String = source
                .lines()
                .skip(i + 1)
                .take(3)
                .collect::<Vec<_>>()
                .join("\n");
            if next_lines.trim().is_empty() || next_lines.contains("pass") || next_lines.contains("// ignore") {
                findings.push(
                    Finding::new(
                        "ComplianceAgent",
                        Severity::Medium,
                        path,
                        "swallowed_error",
                        "Empty catch block — errors silently ignored",
                    )
                    .with_lines(line_num, line_num + 3)
                    .with_cvss(4.0)
                    .with_correction("Log error and handle appropriately", "15 minutes"),
                );
            }
        }
    }

    findings
}
