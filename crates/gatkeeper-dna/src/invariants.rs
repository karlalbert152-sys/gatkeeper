use gatkeeper_core::{Finding, Severity};
use std::path::Path;

pub struct Invariant {
    pub name: String,
    pub rule: String,
    pub file: String,
    pub line: usize,
}

pub fn extract_invariants(file: &Path, source: &str) -> Vec<Invariant> {
    let mut invariants = Vec::new();
    let file_str = file.to_string_lossy();

    for (i, line) in source.lines().enumerate() {
        // Extract invariants from test assertions
        if line.contains("assert!") || line.contains("assert_eq!") || line.contains("expect(") {
            invariants.push(Invariant {
                name: format!("invariant_{}", i + 1),
                rule: line.trim().to_string(),
                file: file_str.to_string(),
                line: i + 1,
            });
        }

        // Extract invariants from comments with "must", "never", "always"
        let lower = line.to_lowercase();
        if lower.contains("// must") || lower.contains("// never") || lower.contains("// always") {
            invariants.push(Invariant {
                name: format!("doc_invariant_{}", i + 1),
                rule: line.trim().to_string(),
                file: file_str.to_string(),
                line: i + 1,
            });
        }
    }

    invariants
}
