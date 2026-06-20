pub struct ArchitecturalPatterns {
    pub patterns: Vec<Pattern>,
}

pub struct Pattern {
    pub name: String,
    pub confidence: f64,
    pub files: Vec<String>,
}

impl ArchitecturalPatterns {
    pub fn detect(source: &str, file: &str) -> Vec<Pattern> {
        let mut patterns = Vec::new();

        // MVC pattern
        if file.contains("controller") || file.contains("model") || file.contains("view") {
            patterns.push(Pattern {
                name: "MVC".to_string(),
                confidence: 0.7,
                files: vec![file.to_string()],
            });
        }

        // Repository pattern
        if file.contains("repository") || file.contains("repo") {
            patterns.push(Pattern {
                name: "Repository".to_string(),
                confidence: 0.8,
                files: vec![file.to_string()],
            });
        }

        // Service layer
        if file.contains("service") {
            patterns.push(Pattern {
                name: "Service Layer".to_string(),
                confidence: 0.75,
                files: vec![file.to_string()],
            });
        }

        patterns
    }
}
