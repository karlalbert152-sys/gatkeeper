use tree_sitter::Language as TsLanguage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    C,
    Go,
    Unknown,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" | "jsx" | "ts" | "tsx" => Language::JavaScript,
            "c" | "h" => Language::C,
            "go" => Language::Go,
            _ => Language::Unknown,
        }
    }

    pub fn tree_sitter_language(&self) -> Option<TsLanguage> {
        match self {
            Language::Rust => Some(tree_sitter_rust::LANGUAGE),
            Language::Python => Some(tree_sitter_python::LANGUAGE),
            Language::JavaScript => Some(tree_sitter_javascript::LANGUAGE),
            Language::C => Some(tree_sitter_c::LANGUAGE),
            Language::Go => Some(tree_sitter_go::LANGUAGE),
            Language::Unknown => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::C => "c",
            Language::Go => "go",
            Language::Unknown => "unknown",
        }
    }
}
