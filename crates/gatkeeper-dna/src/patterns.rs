use std::path::Path;

#[derive(Debug, Clone)]
pub struct DetectedPattern {
    pub name: String,
    pub confidence: f64,
    pub files: Vec<String>,
}

pub struct PatternDetector;

impl PatternDetector {
    pub fn detect(project_root: &Path) -> Vec<DetectedPattern> {
        let mut patterns = Vec::new();

        if let Ok(files) = collect_all_files(project_root) {
            patterns.extend(detect_mvc(&files));
            patterns.extend(detect_repository(&files));
            patterns.extend(detect_factory(&files));
            patterns.extend(detect_observer(&files));
            patterns.extend(detect_middleware(&files));
        }

        patterns
    }
}

fn detect_mvc(files: &[String]) -> Vec<DetectedPattern> {
    let has_models = files.iter().any(|f| f.contains("model"));
    let has_views = files.iter().any(|f| f.contains("view") || f.contains("template"));
    let has_controllers = files.iter().any(|f| f.contains("controller") || f.contains("handler"));

    if has_models && has_views && has_controllers {
        vec![DetectedPattern {
            name: "MVC".to_string(),
            confidence: 0.8,
            files: files.iter().take(5).cloned().collect(),
        }]
    } else {
        vec![]
    }
}

fn detect_repository(files: &[String]) -> Vec<DetectedPattern> {
    let repo_files: Vec<_> = files
        .iter()
        .filter(|f| f.contains("repository") || f.contains("repo"))
        .cloned()
        .collect();

    if repo_files.len() >= 2 {
        vec![DetectedPattern {
            name: "Repository".to_string(),
            confidence: 0.7,
            files: repo_files,
        }]
    } else {
        vec![]
    }
}

fn detect_factory(files: &[String]) -> Vec<DetectedPattern> {
    let factory_files: Vec<_> = files
        .iter()
        .filter(|f| f.contains("factory") || f.contains("builder"))
        .cloned()
        .collect();

    if !factory_files.is_empty() {
        vec![DetectedPattern {
            name: "Factory".to_string(),
            confidence: 0.75,
            files: factory_files,
        }]
    } else {
        vec![]
    }
}

fn detect_observer(files: &[String]) -> Vec<DetectedPattern> {
    let observer_files: Vec<_> = files
        .iter()
        .filter(|f| f.contains("event") || f.contains("listener") || f.contains("observer"))
        .cloned()
        .collect();

    if observer_files.len() >= 2 {
        vec![DetectedPattern {
            name: "Observer".to_string(),
            confidence: 0.65,
            files: observer_files,
        }]
    } else {
        vec![]
    }
}

fn detect_middleware(files: &[String]) -> Vec<DetectedPattern> {
    let middleware_files: Vec<_> = files
        .iter()
        .filter(|f| f.contains("middleware"))
        .cloned()
        .collect();

    if !middleware_files.is_empty() {
        vec![DetectedPattern {
            name: "Middleware".to_string(),
            confidence: 0.85,
            files: middleware_files,
        }]
    } else {
        vec![]
    }
}

fn collect_all_files(root: &Path) -> Result<Vec<String>, std::io::Error> {
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
                } else {
                    files.push(path.display().to_string());
                }
            }
        }
    }

    Ok(files)
}
