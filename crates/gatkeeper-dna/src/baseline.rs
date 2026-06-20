use chrono::{DateTime, Utc};
use std::path::Path;

pub struct Baseline {
    pub established: DateTime<Utc>,
    pub healthy_patterns: Vec<String>,
    pub file_hashes: Vec<FileBaseline>,
}

pub struct FileBaseline {
    pub path: String,
    pub hash: String,
    pub line_count: usize,
    pub last_modified: DateTime<Utc>,
}

impl Baseline {
    pub fn establish(root: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file_baselines = Vec::new();

        for entry in std::fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();
                    if matches!(ext_str.as_ref(), "rs" | "py" | "js" | "ts" | "c" | "go") {
                        let content = std::fs::read_to_string(&path)?;
                        let hash = format!("{:x}", sha2::Sha256::digest(content.as_bytes()));

                        file_baselines.push(FileBaseline {
                            path: path.to_string_lossy().to_string(),
                            hash,
                            line_count: content.lines().count(),
                            last_modified: Utc::now(),
                        });
                    }
                }
            }
        }

        Ok(Self {
            established: Utc::now(),
            healthy_patterns: Vec::new(),
            file_hashes: file_baselines,
        })
    }

    pub fn detect_divergence(&self, current_root: &Path) -> Vec<String> {
        let mut divergences = Vec::new();

        for baseline in &self.file_hashes {
            let current_path = current_root.join(&baseline.path);
            if current_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&current_path) {
                    let current_hash = format!("{:x}", sha2::Sha256::digest(content.as_bytes()));
                    if current_hash != baseline.hash {
                        divergences.push(format!(
                            "File {} has diverged from baseline (expected: {}...)",
                            baseline.path,
                            &baseline.hash[..12]
                        ));
                    }
                }
            } else {
                divergences.push(format!("File {} has been deleted", baseline.path));
            }
        }

        divergences
    }
}
