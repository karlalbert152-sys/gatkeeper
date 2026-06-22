use crate::Agent;
use gatkeeper_core::finding::{Finding, Severity};
use gatkeeper_parser::ParsedFile;

pub struct LogicAgent;

impl Agent for LogicAgent {
    fn name(&self) -> &str {
        "LogicAgent"
    }

    fn analyze(&self, files: &[ParsedFile]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for file in files {
            let source = match std::fs::read_to_string(&file.path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            findings.extend(check_deadlocks(&file.path, &source));
            findings.extend(check_race_conditions(&file.path, &source));
            findings.extend(check_unhandled_errors(&file.path, &source));
            findings.extend(check_infinite_loops(&file.path, &source));
        }

        findings
    }
}

fn check_deadlocks(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let has_mutex = source.contains("Mutex") || source.contains("RwLock");
    let has_lock = source.contains(".lock()") || source.contains(".read()") || source.contains(".write()");

    if has_mutex && has_lock {
        let nested_locks = source.matches(".lock()").count() + source.matches(".read()").count() + source.matches(".write()").count();
        if nested_locks > 2 {
            for (i, line) in source.lines().enumerate() {
                let line_num = i as u32 + 1;
                if line.contains(".lock()") || line.contains(".read()") || line.contains(".write()") {
                    findings.push(
                        Finding::new(
                            "LogicAgent",
                            Severity::High,
                            path,
                            "potential_deadlock",
                            "Multiple lock acquisitions detected — risk of deadlock",
                        )
                        .with_lines(line_num, line_num)
                        .with_cvss(6.0)
                        .with_correction("Use consistent lock ordering or try_lock", "2 hours"),
                    );
                    break;
                }
            }
        }
    }

    findings
}

fn check_race_conditions(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("static mut") {
            findings.push(
                Finding::new(
                    "LogicAgent",
                    Severity::High,
                    path,
                    "race_condition",
                    "Mutable static variable — data race risk",
                )
                .with_lines(line_num, line_num)
                .with_cvss(7.0)
                .with_correction("Use atomics, Mutex, or thread-local storage", "2 hours"),
            );
        }

        if line.contains("thread::spawn") && source.contains("shared") {
            findings.push(
                Finding::new(
                    "LogicAgent",
                    Severity::Medium,
                    path,
                    "shared_state",
                    "Thread with shared state — verify synchronization",
                )
                .with_lines(line_num, line_num)
                .with_cvss(5.0)
                .with_correction("Ensure proper synchronization with Arc/Mutex", "1 hour"),
            );
        }
    }

    findings
}

fn check_unhandled_errors(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("unwrap()") && !line.contains("// safe") {
            findings.push(
                Finding::new(
                    "LogicAgent",
                    Severity::Medium,
                    path,
                    "unwrap_usage",
                    "unwrap() may panic on None/Err — handle gracefully",
                )
                .with_lines(line_num, line_num)
                .with_cvss(4.0)
                .with_correction("Use match, if-let, or .unwrap_or_default()", "15 minutes"),
            );
        }

        if line.contains("panic!") {
            findings.push(
                Finding::new(
                    "LogicAgent",
                    Severity::Medium,
                    path,
                    "explicit_panic",
                    "Explicit panic in application code",
                )
                .with_lines(line_num, line_num)
                .with_cvss(4.0)
                .with_correction("Return Result instead of panicking", "30 minutes"),
            );
        }
    }

    findings
}

fn check_infinite_loops(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.trim_start().starts_with("loop") && !source.contains("break") {
            findings.push(
                Finding::new(
                    "LogicAgent",
                    Severity::Low,
                    path,
                    "infinite_loop",
                    "Loop without visible break condition",
                )
                .with_lines(line_num, line_num)
                .with_cvss(3.0)
                .with_correction("Add break condition or timeout", "15 minutes"),
            );
        }
    }

    findings
}
