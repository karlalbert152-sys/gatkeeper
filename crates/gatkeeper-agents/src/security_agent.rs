use crate::Agent;
use gatkeeper_core::finding::{Finding, Severity};
use gatkeeper_parser::ParsedFile;

pub struct SecurityAgent;

impl Agent for SecurityAgent {
    fn name(&self) -> &str {
        "SecurityAgent"
    }

    fn analyze(&self, files: &[ParsedFile]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for file in files {
            let source = match std::fs::read_to_string(&file.path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            findings.extend(check_injection_patterns(&file.path, &source));
            findings.extend(check_crypto_weaknesses(&file.path, &source));
            findings.extend(check_auth_issues(&file.path, &source));
            findings.extend(check_memory_safety(&file.path, &source));
        }

        findings
    }
}

fn check_injection_patterns(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("format!(")
            && line.contains("{}")
            && (line.contains("execute") || line.contains("query") || line.contains("exec"))
        {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::Critical,
                    path,
                    "sql_injection",
                    "Potential SQL injection via format string",
                )
                .with_lines(line_num, line_num)
                .with_cvss(9.8)
                .with_correction(
                    "Use parameterized queries instead of format strings",
                    "30 minutes",
                ),
            );
        }

        if line.contains("eval(") || line.contains("exec(") {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::Critical,
                    path,
                    "code_injection",
                    "Dynamic code execution via eval/exec",
                )
                .with_lines(line_num, line_num)
                .with_cvss(9.0)
                .with_correction(
                    "Avoid dynamic code execution, use safe alternatives",
                    "1 hour",
                ),
            );
        }

        if line.contains("innerHTML") || line.contains("dangerouslySetInnerHTML") {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::High,
                    path,
                    "xss",
                    "Potential XSS via unescaped HTML injection",
                )
                .with_lines(line_num, line_num)
                .with_cvss(7.5)
                .with_correction("Use textContent or sanitize HTML input", "30 minutes"),
            );
        }
    }

    findings
}

fn check_crypto_weaknesses(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("md5") || line.contains("MD5") {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::High,
                    path,
                    "weak_crypto",
                    "MD5 is cryptographically broken",
                )
                .with_lines(line_num, line_num)
                .with_cvss(7.0)
                .with_correction("Use SHA-256 or SHA-3 instead", "1 hour"),
            );
        }

        if line.contains("sha1") || line.contains("SHA1") {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::Medium,
                    path,
                    "weak_crypto",
                    "SHA-1 is deprecated for security use",
                )
                .with_lines(line_num, line_num)
                .with_cvss(5.0)
                .with_correction("Migrate to SHA-256 or stronger", "2 hours"),
            );
        }

        if line.contains("rand::thread_rng")
            && !line.contains("OsRng")
            && (source.contains("password")
                || source.contains("secret")
                || source.contains("token"))
        {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::High,
                    path,
                    "weak_random",
                    "Non-cryptographic RNG used for security-sensitive value",
                )
                .with_lines(line_num, line_num)
                .with_cvss(6.5)
                .with_correction("Use OsRng or a CSPRNG for security contexts", "30 minutes"),
            );
        }
    }

    findings
}

fn check_auth_issues(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("password") && (line.contains("==") || line.contains("equals")) {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::High,
                    path,
                    "plaintext_comparison",
                    "Password compared as plaintext string",
                )
                .with_lines(line_num, line_num)
                .with_cvss(7.5)
                .with_correction(
                    "Use constant-time comparison (e.g. subtle::ConstantTimeEq)",
                    "1 hour",
                ),
            );
        }

        if line.contains("Authorization") && line.contains("Bearer") && line.contains("TODO") {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::Medium,
                    path,
                    "auth_todo",
                    "TODO marker near Bearer token handling",
                )
                .with_lines(line_num, line_num)
                .with_cvss(5.0)
                .with_correction("Complete the authentication implementation", "2 hours"),
            );
        }
    }

    findings
}

fn check_memory_safety(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    if !path.ends_with(".rs") {
        return findings;
    }

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("unsafe") && line.contains("{") {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::Medium,
                    path,
                    "unsafe_block",
                    "Unsafe Rust block detected — verify memory safety guarantees",
                )
                .with_lines(line_num, line_num)
                .with_cvss(4.0)
                .with_correction(
                    "Audit unsafe block, minimize scope, add safety comments",
                    "1 hour",
                ),
            );
        }

        if line.contains("transmute") {
            findings.push(
                Finding::new(
                    "SecurityAgent",
                    Severity::High,
                    path,
                    "unsafe_transmute",
                    "Transmute can cause undefined behavior",
                )
                .with_lines(line_num, line_num)
                .with_cvss(7.0)
                .with_correction(
                    "Replace transmute with safe alternatives (From/Into)",
                    "2 hours",
                ),
            );
        }
    }

    findings
}
