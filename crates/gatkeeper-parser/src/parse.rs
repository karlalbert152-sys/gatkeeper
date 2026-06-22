use std::path::Path;
use tree_sitter::Parser;

#[derive(Debug, Clone)]
pub struct ParsedFile {
    pub path: String,
    pub language: String,
    pub line_count: u32,
    pub functions: Vec<FunctionInfo>,
    pub imports: Vec<String>,
    pub has_unsafe: bool,
    pub has_error_handling: bool,
    pub complexity_hints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub start_line: u32,
    pub end_line: u32,
    pub is_public: bool,
    pub parameters: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unsupported file extension: {0}")]
    UnsupportedExtension(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error in {0}: {1}")]
    TreeSitter(String, String),
}

pub fn parse_file(path: &Path) -> Result<ParsedFile, ParseError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| ParseError::UnsupportedExtension("unknown".to_string()))?;

    let (lang, lang_def) = match ext {
        "rs" => super::languages::rust_lang(),
        "py" => super::languages::python_lang(),
        "js" | "jsx" => super::languages::javascript_lang(),
        "c" | "h" => super::languages::c_lang(),
        "go" => super::languages::go_lang(),
        _ => return Err(ParseError::UnsupportedExtension(ext.to_string())),
    };

    let source = std::fs::read_to_string(path)?;
    let line_count = source.lines().count() as u32;
    let has_unsafe = source.contains("unsafe");
    let has_error_handling = source.contains("Result<")
        || source.contains("unwrap()")
        || source.contains("panic!")
        || source.contains("try:")
        || source.contains("catch");

    let mut parser = Parser::new();
    parser
        .set_language(&lang)
        .map_err(|e| ParseError::TreeSitter(path.display().to_string(), e.to_string()))?;

    let tree = parser.parse(&source, None).ok_or_else(|| {
        ParseError::TreeSitter(path.display().to_string(), "parse failed".to_string())
    })?;

    let root = tree.root_node();
    let mut functions = Vec::new();
    let mut imports = Vec::new();

    extract_functions(root, &source, &mut functions);
    extract_imports(root, &source, &mut imports);

    let complexity_hints = detect_complexity_hints(&source, &functions);

    Ok(ParsedFile {
        path: path.display().to_string(),
        language: lang_def.name.to_string(),
        line_count,
        functions,
        imports,
        has_unsafe,
        has_error_handling,
        complexity_hints,
    })
}

fn extract_functions(node: tree_sitter::Node, source: &str, out: &mut Vec<FunctionInfo>) {
    let kind = node.kind();
    if matches!(
        kind,
        "function_item"
            | "function_definition"
            | "method_definition"
            | "function_declaration"
            | "arrow_function"
    ) {
        let name = node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("property"))
            .and_then(|n| n.utf8_text(source.as_bytes()).ok())
            .unwrap_or("<anonymous>")
            .to_string();

        let start_line = node.start_position().row as u32 + 1;
        let end_line = node.end_position().row as u32 + 1;

        let is_public = source[node.start_byte()..node.end_byte()].contains("pub ");

        out.push(FunctionInfo {
            name,
            start_line,
            end_line,
            is_public,
            parameters: Vec::new(),
        });
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        extract_functions(child, source, out);
    }
}

fn extract_imports(node: tree_sitter::Node, source: &str, out: &mut Vec<String>) {
    let kind = node.kind();
    if matches!(
        kind,
        "use_declaration"
            | "import_statement"
            | "import_from_statement"
            | "preproc_include"
            | "import_spec"
    ) {
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            let cleaned = text
                .replace("use ", "")
                .replace("import ", "")
                .replace("from ", "")
                .replace(";", "")
                .trim()
                .to_string();
            if !cleaned.is_empty() {
                out.push(cleaned);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        extract_imports(child, source, out);
    }
}

fn detect_complexity_hints(source: &str, functions: &[FunctionInfo]) -> Vec<String> {
    let mut hints = Vec::new();

    for func in functions {
        let line_span = func.end_line.saturating_sub(func.start_line);
        if line_span > 50 {
            hints.push(format!(
                "Long function '{}' ({} lines) at line {}",
                func.name, line_span, func.start_line
            ));
        }
    }

    let indent_levels = source
        .lines()
        .filter(|l| l.starts_with('\t') || l.starts_with("    "))
        .count();
    if indent_levels > 20 {
        hints.push(format!(
            "Deep nesting detected ({} indented lines)",
            indent_levels
        ));
    }

    hints
}
