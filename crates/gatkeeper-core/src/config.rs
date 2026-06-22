use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatKeeperConfig {
    pub project: ProjectConfig,
    pub scan: ScanConfig,
    pub risk: RiskConfig,
    pub subconscious: SubconsciousConfig,
    pub output: OutputConfig,
    pub agents: AgentsConfig,
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
    pub critical_threshold: u32,
    pub high_threshold: u32,
    pub medium_threshold: u32,
    pub block_merge_below: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubconsciousConfig {
    pub enabled: bool,
    pub cycle: String,
    pub projection_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String,
    pub directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AgentsConfig {
    #[serde(default)]
    pub security: AgentDetail,
    #[serde(default)]
    pub secrets: SecretsConfig,
    #[serde(default)]
    pub supply_chain: SupplyChainConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDetail {
    pub enabled: bool,
    pub depth: String,
}

impl Default for AgentDetail {
    fn default() -> Self {
        Self {
            enabled: true,
            depth: "standard".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsConfig {
    pub enabled: bool,
    #[serde(default)]
    pub custom_patterns: Vec<String>,
}

impl Default for SecretsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            custom_patterns: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainConfig {
    pub enabled: bool,
    pub check_licenses: bool,
    pub check_abandoned: bool,
}

impl Default for SupplyChainConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_licenses: true,
            check_abandoned: true,
        }
    }
}


impl Default for GatKeeperConfig {
    fn default() -> Self {
        Self {
            project: ProjectConfig {
                name: "unknown".to_string(),
                version: "0.0.1".to_string(),
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
            agents: AgentsConfig::default(),
        }
    }
}

impl GatKeeperConfig {
    pub fn load(project_root: &Path) -> Result<Self, ConfigError> {
        let config_path = project_root.join("gatkeeper.toml");
        if !config_path.exists() {
            return Err(ConfigError::NotFound(config_path));
        }
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::Io(config_path.clone(), e))?;
        let config: Self =
            toml::from_str(&content).map_err(|e| ConfigError::Parse(config_path, e))?;
        Ok(config)
    }

    pub fn save(&self, project_root: &Path) -> Result<(), ConfigError> {
        let config_path = project_root.join("gatkeeper.toml");
        let content =
            toml::to_string_pretty(self).map_err(ConfigError::Serialize)?;
        std::fs::write(&config_path, content)
            .map_err(|e| ConfigError::Io(config_path, e))?;
        Ok(())
    }

    pub fn gatkeeper_dir(&self, project_root: &Path) -> PathBuf {
        project_root.join(&self.output.directory)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration not found: {0}")]
    NotFound(PathBuf),
    #[error("IO error reading {0}: {1}")]
    Io(PathBuf, #[source] std::io::Error),
    #[error("TOML parse error in {0}: {1}")]
    Parse(PathBuf, #[source] toml::de::Error),
    #[error("TOML serialize error: {0}")]
    Serialize(#[source] toml::ser::Error),
}
