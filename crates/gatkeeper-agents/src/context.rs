use gatkeeper_core::Finding;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AnalysisContext {
    pub project_root: PathBuf,
    pub files: Vec<PathBuf>,
    pub findings: Vec<Finding>,
    pub lines_analyzed: usize,
}

impl AnalysisContext {
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            files: Vec::new(),
            findings: Vec::new(),
            lines_analyzed: 0,
        }
    }

    pub fn add_finding(&mut self, finding: Finding) {
        self.findings.push(finding);
    }
}
