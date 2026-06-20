use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;

pub struct CodebaseFingerprint {
    pub hash: String,
    pub file_count: usize,
    pub total_lines: usize,
    pub timestamp: String,
}

impl CodebaseFingerprint {
    pub fn compute(root: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut hasher = Sha256::new();
        let mut file_count = 0;
        let mut total_lines = 0;

        // Simple fingerprint: hash all source files
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();
                    if matches!(ext_str.as_ref(), "rs" | "py" | "js" | "ts" | "c" | "h" | "go") {
                        let content = fs::read_to_string(&path)?;
                        hasher.update(content.as_bytes());
                        file_count += 1;
                        total_lines += content.lines().count();
                    }
                }
            }
        }

        let hash = format!("{:x}", hasher.finalize());

        Ok(Self {
            hash,
            file_count,
            total_lines,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
}
