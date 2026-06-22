use std::path::Path;

#[derive(Debug, Clone)]
pub struct Invariant {
    pub name: String,
    pub rule: String,
    pub file: String,
    pub line: Option<u32>,
}

pub struct InvariantExtractor;

impl InvariantExtractor {
    pub fn extract(project_root: &Path) -> Vec<Invariant> {
        let mut invariants = Vec::new();

        if let Ok(files) = collect_rust_files(project_root) {
            for path_str in &files {
                let path = Path::new(path_str);
                if let Ok(source) = std::fs::read_to_string(path) {
                    invariants.extend(extract_error_handling_invariants(path_str, &source));
                    invariants.extend(extract_type_invariants(path_str, &source));
                    invariants.extend(extract_api_invariants(path_str, &source));
                }
            }
        }

        invariants
    }
}

fn extract_error_handling_invariants(path: &str, source: &str) -> Vec<Invariant> {
    let mut invariants = Vec::new();

    for (i, line) in source.lines().enumerate() {
        if line.contains("Result<") && line.contains("->") && !line.contains("unwrap()") {
            invariants.push(Invariant {
                name: "Error propagation".to_string(),
                rule: "Function returns Result — errors must be handled".to_string(),
                file: path.to_string(),
                line: Some(i as u32 + 1),
            });
        }
    }

    invariants
}

fn extract_type_invariants(path: &str, source: &str) -> Vec<Invariant> {
    let mut invariants = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("pub struct") && trimmed.contains("id:") {
            let struct_name = trimmed
                .split_whitespace()
                .nth(2)
                .unwrap_or("Unknown")
                .replace('{', "")
                .trim()
                .to_string();
            invariants.push(Invariant {
                name: "Entity identity".to_string(),
                rule: format!("Struct '{}' has id field — must be unique", struct_name),
                file: path.to_string(),
                line: Some(i as u32 + 1),
            });
        }

        if trimmed.starts_with("pub enum") {
            let enum_name = trimmed
                .split_whitespace()
                .nth(2)
                .unwrap_or("Unknown")
                .replace('{', "")
                .trim()
                .to_string();
            if !enum_name.is_empty() && !enum_name.contains('"') {
                invariants.push(Invariant {
                    name: format!("{} exhaustiveness", enum_name),
                    rule: format!("All {} variants must be handled in match", enum_name),
                    file: path.to_string(),
                    line: Some(i as u32 + 1),
                });
            }
        }
    }

    invariants
}

fn extract_api_invariants(path: &str, source: &str) -> Vec<Invariant> {
    let mut invariants = Vec::new();

    for (i, line) in source.lines().enumerate() {
        if line.contains("#[get]") || line.contains("#[post]") || line.contains("#[put]") || line.contains("#[delete]") {
            invariants.push(Invariant {
                name: "API endpoint".to_string(),
                rule: "HTTP endpoint must validate input and return proper status codes".to_string(),
                file: path.to_string(),
                line: Some(i as u32 + 1),
            });
        }
    }

    invariants
}

fn collect_rust_files(root: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if !name.starts_with('.') && name != "target" && name != "node_modules" {
                        stack.push(path);
                    }
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    files.push(path.display().to_string());
                }
            }
        }
    }

    Ok(files)
}
