use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatKeeperConfig {
    pub project: ProjectConfig,
    pub scan: ScanConfig,
    pub risk: RiskConfig,
    pub subconscious: SubconsciousConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub agents: Vec<String>,
    pub languages: Vec<String>,
    pub max_depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    pub critical_threshold: usize,
    pub high_threshold: usize,
    pub medium_threshold: usize,
    pub block_merge_below: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubconsciousConfig {
    pub enabled: bool,
    pub cycle: String,
    pub projection_hours: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String,
    pub directory: String,
}

impl GatKeeperConfig {
    pub fn load(path: impl AsRef<Path>) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default_config() -> Self {
        Self {
            project: ProjectConfig {
                name: "unknown".to_string(),
                version: "0.1.0".to_string(),
            },
            scan: ScanConfig {
                agents: vec![
                    "security".to_string(),
                    "logic".to_string(),
                    "performance".to_string(),
                    "compliance".to_string(),
                    "secrets".to_string(),
                    "supply_chain".to_string(),
                ],
                languages: vec![
                    "rust".to_string(),
                    "python".to_string(),
                    "javascript".to_string(),
                    "c".to_string(),
                    "go".to_string(),
                ],
                max_depth: 50,
            },
            risk: RiskConfig {
                critical_threshold: 90,
                high_threshold: 70,
                medium_threshold: 40,
                block_merge_below: 60,
            },
            subconscious: SubconsciousConfig {
                enabled: true,
                cycle: "hourly".to_string(),
                projection_hours: 24,
            },
            output: OutputConfig {
                format: "gat".to_string(),
                directory: ".gatkeeper".to_string(),
            },
        }
    }
}
