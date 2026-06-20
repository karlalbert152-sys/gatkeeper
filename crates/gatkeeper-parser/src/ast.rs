use tree_sitter::Parser;

pub struct AstNode {
    pub kind: String,
    pub text: String,
    pub line: usize,
    pub column: usize,
    pub children: Vec<AstNode>,
}

pub struct ParsedFile {
    pub path: String,
    pub language: super::languages::Language,
    pub tree: tree_sitter::Tree,
    pub source: String,
}

impl ParsedFile {
    pub fn parse(path: &str, source: &str, lang: super::languages::Language) -> Result<Self, String> {
        let mut parser = Parser::new();

        let ts_lang = lang.tree_sitter_language()
            .ok_or_else(|| format!("Unsupported language: {:?}", lang))?;

        parser.set_language(&ts_lang)
            .map_err(|e| format!("Failed to set language: {}", e))?;

        let tree = parser.parse(source, None)
            .ok_or_else(|| "Failed to parse file".to_string())?;

        Ok(Self {
            path: path.to_string(),
            language: lang,
            tree,
            source: source.to_string(),
        })
    }

    pub fn root_node(&self) -> tree_sitter::Node {
        self.tree.root_node()
    }
}
