use tree_sitter::Node;

pub struct AstWalker;

impl AstWalker {
    pub fn new() -> Self {
        Self
    }

    pub fn walk(&self, node: Node, source: &str, depth: usize) {
        let indent = "  ".repeat(depth);
        let text = node
            .utf8_text(source.as_bytes())
            .unwrap_or("<binary>");

        println!(
            "{}{} [{}] (line {})",
            indent,
            node.kind(),
            &text[..text.len().min(50)],
            node.start_position().row + 1
        );

        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            self.walk(child, source, depth + 1);
        }
    }

    pub fn find_functions(&self, node: Node, source: &str) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();
        self.collect_functions(node, source, &mut functions);
        functions
    }

    fn collect_functions(&self, node: Node, source: &str, functions: &mut Vec<FunctionInfo>) {
        if node.kind() == "function_item" || node.kind() == "function_definition" {
            let name = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                .unwrap_or("anonymous")
                .to_string();

            functions.push(FunctionInfo {
                name,
                line: node.start_position().row + 1,
                kind: node.kind().to_string(),
            });
        }

        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            self.collect_functions(child, source, functions);
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub line: usize,
    pub kind: String,
}
