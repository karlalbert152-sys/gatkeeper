use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub root: PathBuf,
    pub name: String,
    pub languages: Vec<String>,
    pub has_git: bool,
    pub has_config: bool,
}

impl ProjectInfo {
    pub fn detect(path: impl AsRef<Path>) -> Self {
        let root = path.as_ref().to_path_buf();
        let name = root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let has_git = root.join(".git").exists();
        let has_config = root.join("gatkeeper.toml").exists();

        let mut languages = Vec::new();
        if has_git {
            languages = detect_languages(&root);
        }

        Self {
            root,
            name,
            languages,
            has_git,
            has_config,
        }
    }
}

fn detect_languages(root: &Path) -> Vec<String> {
    let mut langs = Vec::new();

    let checks: &[(&str, &str)] = &[
        ("Cargo.toml", "rust"),
        ("pyproject.toml", "python"),
        ("package.json", "javascript"),
        ("go.mod", "go"),
        ("pom.xml", "java"),
        ("Gemfile", "ruby"),
        ("build.gradle", "kotlin"),
        ("*.c", "c"),
        ("*.cpp", "cpp"),
    ];

    for (pattern, lang) in checks {
        if root.join(pattern).exists() {
            langs.push(lang.to_string());
        }
    }

    if langs.is_empty() {
        langs.push("unknown".to_string());
    }

    langs
}
