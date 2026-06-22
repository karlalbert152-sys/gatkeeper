use sha2::{Sha256, Digest};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct DnaFingerprint {
    pub root_hash: String,
    pub file_count: u32,
    pub total_lines: u64,
    pub file_hashes: Vec<(String, String)>,
}

impl DnaFingerprint {
    pub fn compute(project_root: &Path) -> Self {
        let mut file_hashes = Vec::new();
        let mut total_lines = 0u64;
        let mut file_count = 0u32;

        if let Ok(entries) = collect_source_files(project_root) {
            for path in entries {
                if let Ok(source) = std::fs::read_to_string(&path) {
                    let mut hasher = Sha256::new();
                    hasher.update(source.as_bytes());
                    let hash = format!("{:x}", hasher.finalize());

                    let relative = path
                        .strip_prefix(project_root)
                        .unwrap_or(&path)
                        .display()
                        .to_string();

                    total_lines += source.lines().count() as u64;
                    file_count += 1;
                    file_hashes.push((relative, hash));
                }
            }
        }

        file_hashes.sort_by(|a, b| a.0.cmp(&b.0));

        let mut root_hasher = Sha256::new();
        for (path, hash) in &file_hashes {
            root_hasher.update(path.as_bytes());
            root_hasher.update(hash.as_bytes());
        }
        let root_hash = format!("{:x}", root_hasher.finalize());

        Self {
            root_hash,
            file_count,
            total_lines,
            file_hashes,
        }
    }

    pub fn diff(&self, other: &DnaFingerprint) -> FingerprintDiff {
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();

        let self_map: std::collections::HashMap<_, _> =
            self.file_hashes.iter().cloned().collect();
        let other_map: std::collections::HashMap<_, _> =
            other.file_hashes.iter().cloned().collect();

        for (path, hash) in &other_map {
            match self_map.get(path) {
                Some(existing) if existing != hash => {
                    modified.push(path.clone());
                }
                None => {
                    added.push(path.clone());
                }
                _ => {}
            }
        }

        for path in self_map.keys() {
            if !other_map.contains_key(path) {
                removed.push(path.clone());
            }
        }

        FingerprintDiff {
            added,
            removed,
            modified,
            root_hash_changed: self.root_hash != other.root_hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FingerprintDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
    pub root_hash_changed: bool,
}

impl FingerprintDiff {
    pub fn has_changes(&self) -> bool {
        self.root_hash_changed || !self.added.is_empty() || !self.removed.is_empty() || !self.modified.is_empty()
    }

    pub fn summary(&self) -> String {
        format!(
            "+{}/-{}/~{} files (hash changed: {})",
            self.added.len(),
            self.removed.len(),
            self.modified.len(),
            self.root_hash_changed
        )
    }
}

fn collect_source_files(root: &Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let extensions = ["rs", "py", "js", "jsx", "c", "h", "go", "toml", "json"];
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if !name.starts_with('.') && name != "target" && name != "node_modules" && name != "vendor" {
                        stack.push(path);
                    }
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if extensions.contains(&ext) {
                        files.push(path);
                    }
                }
            }
        }
    }

    Ok(files)
}
