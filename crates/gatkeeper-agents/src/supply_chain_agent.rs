use crate::Agent;
use gatkeeper_core::finding::{Finding, Severity};
use gatkeeper_parser::ParsedFile;

pub struct SupplyChainAgent;

impl Agent for SupplyChainAgent {
    fn name(&self) -> &str {
        "SupplyChainAgent"
    }

    fn analyze(&self, files: &[ParsedFile]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for file in files {
            if file.path.ends_with("Cargo.toml") {
                findings.extend(check_cargo_deps(&file.path));
            } else if file.path.ends_with("package.json") {
                findings.extend(check_npm_deps(&file.path));
            } else if file.path.ends_with("requirements.txt") || file.path.ends_with("pyproject.toml") {
                findings.extend(check_python_deps(&file.path));
            } else if file.path.ends_with("go.mod") {
                findings.extend(check_go_deps(&file.path));
            }
        }

        findings
    }
}

fn check_cargo_deps(path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return findings,
    };

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("git = ") && !line.contains("tag = ") && !line.contains("rev = ") {
            findings.push(
                Finding::new(
                    "SupplyChainAgent",
                    Severity::Medium,
                    path,
                    "unpinned_git_dep",
                    "Git dependency without pinned tag/rev",
                )
                .with_lines(line_num, line_num)
                .with_cvss(5.0)
                .with_correction("Pin to specific tag or commit hash", "15 minutes"),
            );
        }

        if line.contains("path = ") && !path.contains("workspace") {
            findings.push(
                Finding::new(
                    "SupplyChainAgent",
                    Severity::Low,
                    path,
                    "local_dep",
                    "Local path dependency — not publishable",
                )
                .with_lines(line_num, line_num)
                .with_cvss(2.0)
                .with_correction("Consider publishing to crates.io for reproducibility", "1 hour"),
            );
        }
    }

    findings
}

fn check_npm_deps(path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return findings,
    };

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if line.contains("\"*\"") || line.contains("\">=\"") {
            findings.push(
                Finding::new(
                    "SupplyChainAgent",
                    Severity::Medium,
                    path,
                    "loose_version",
                    "Loose version constraint in dependency",
                )
                .with_lines(line_num, line_num)
                .with_cvss(4.0)
                .with_correction("Pin to specific version range", "15 minutes"),
            );
        }

        if line.contains("github.com/") && !line.contains("#") {
            findings.push(
                Finding::new(
                    "SupplyChainAgent",
                    Severity::Medium,
                    path,
                    "unpinned_git_dep",
                    "GitHub dependency without commit pin",
                )
                .with_lines(line_num, line_num)
                .with_cvss(5.0)
                .with_correction("Pin to specific commit hash", "15 minutes"),
            );
        }
    }

    findings
}

fn check_python_deps(path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return findings,
    };

    for (i, line) in source.lines().enumerate() {
        let line_num = i as u32 + 1;

        if !line.trim().is_empty() && !line.starts_with('#') && !line.contains("==") && !line.contains(">=") && !line.contains("<=") {
            findings.push(
                Finding::new(
                    "SupplyChainAgent",
                    Severity::Medium,
                    path,
                    "unpinned_version",
                    "Python dependency without version pin",
                )
                .with_lines(line_num, line_num)
                .with_cvss(4.0)
                .with_correction("Pin to specific version (package==x.y.z)", "15 minutes"),
            );
        }
    }

    findings
}

fn check_go_deps(path: &str) -> Vec<Finding> {
    let findings = Vec::new();
    let _source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return findings,
    };

    findings
}
