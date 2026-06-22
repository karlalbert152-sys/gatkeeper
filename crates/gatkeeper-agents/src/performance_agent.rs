use crate::Agent;
use gatkeeper_core::finding::{Finding, Severity};
use gatkeeper_parser::ParsedFile;

pub struct PerformanceAgent;

impl Agent for PerformanceAgent {
    fn name(&self) -> &str {
        "PerformanceAgent"
    }

    fn analyze(&self, files: &[ParsedFile]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for file in files {
            let source = match std::fs::read_to_string(&file.path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            findings.extend(check_n_plus_one(&file.path, &source));
            findings.extend(check_memory_leaks(&file.path, &source));
            findings.extend(check_inefficient_loops(&file.path, &source));
            findings.extend(check_large_allocations(&file.path, &source));
        }

        findings
    }
}

fn check_n_plus_one(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains(".await") && (line.contains("for ") || line.contains("while ")) {
            findings.push(
                Finding::new(
                    "PerformanceAgent",
                    Severity::High,
                    path,
                    "n_plus_one",
                    "Async call inside loop — potential N+1 query pattern",
                )
                .with_lines(line_num, line_num)
                .with_cvss(6.0)
                .with_correction("Batch async calls or use join_all", "1 hour"),
            );
        }

        if line.contains("SELECT")
            && line.contains("WHERE")
            && source
                .lines()
                .nth(i + 1)
                .is_some_and(|l| l.contains("SELECT"))
        {
            findings.push(
                Finding::new(
                    "PerformanceAgent",
                    Severity::Medium,
                    path,
                    "sequential_queries",
                    "Multiple sequential SELECT queries",
                )
                .with_lines(line_num, line_num + 1)
                .with_cvss(5.0)
                .with_correction("Use JOIN or batch queries", "30 minutes"),
            );
        }
    }

    findings
}

fn check_memory_leaks(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("Vec::new()")
            && line.contains("push")
            && (source.contains("loop") || source.contains("while"))
        {
            findings.push(
                Finding::new(
                    "PerformanceAgent",
                    Severity::Medium,
                    path,
                    "unbounded_growth",
                    "Vec growth inside loop without capacity limit",
                )
                .with_lines(line_num, line_num)
                .with_cvss(4.0)
                .with_correction("Set initial capacity or add size limit", "15 minutes"),
            );
        }

        if line.contains("Box::leak") || line.contains("std::mem::forget") {
            findings.push(
                Finding::new(
                    "PerformanceAgent",
                    Severity::High,
                    path,
                    "explicit_leak",
                    "Explicit memory leak detected",
                )
                .with_lines(line_num, line_num)
                .with_cvss(6.0)
                .with_correction(
                    "Remove explicit leak, use proper lifecycle management",
                    "30 minutes",
                ),
            );
        }
    }

    findings
}

fn check_inefficient_loops(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains(".clone()") && (line.contains("for ") || line.contains(".iter()")) {
            findings.push(
                Finding::new(
                    "PerformanceAgent",
                    Severity::Medium,
                    path,
                    "unnecessary_clone",
                    "Clone inside iteration — use references instead",
                )
                .with_lines(line_num, line_num)
                .with_cvss(3.0)
                .with_correction("Use & references or Cow<str>", "15 minutes"),
            );
        }

        if line.contains("collect::<Vec<_>>()") && line.contains(".iter()") {
            findings.push(
                Finding::new(
                    "PerformanceAgent",
                    Severity::Low,
                    path,
                    "eager_collection",
                    "Eager collection — consider lazy iterator",
                )
                .with_lines(line_num, line_num)
                .with_cvss(2.0)
                .with_correction("Use iterator combinators directly", "15 minutes"),
            );
        }
    }

    findings
}

fn check_large_allocations(path: &str, source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("vec![0;") || line.contains("vec![0u8;") {
            findings.push(
                Finding::new(
                    "PerformanceAgent",
                    Severity::Medium,
                    path,
                    "large_allocation",
                    "Large zero-initialized allocation",
                )
                .with_lines(line_num, line_num)
                .with_cvss(3.0)
                .with_correction("Consider memory-mapped files for large data", "1 hour"),
            );
        }

        if line.contains("read_to_string") && !line.contains("limit") {
            findings.push(
                Finding::new(
                    "PerformanceAgent",
                    Severity::Medium,
                    path,
                    "unbounded_read",
                    "Reading entire file to string without size limit",
                )
                .with_lines(line_num, line_num)
                .with_cvss(4.0)
                .with_correction(
                    "Add file size check or use BufReader with limit",
                    "30 minutes",
                ),
            );
        }
    }

    findings
}
