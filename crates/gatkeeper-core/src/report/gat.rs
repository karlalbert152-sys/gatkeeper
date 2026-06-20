use super::findings::Finding;
use super::risk_score::RiskScore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatReport {
    pub session: SessionInfo,
    pub risk_score: RiskScore,
    pub findings: Vec<Finding>,
    pub subconscious: SubconsciousInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub organisation: String,
    pub project: String,
    pub version: String,
    pub timestamp: String,
    pub agents_used: Vec<String>,
    pub lines_analyzed: usize,
    pub scenarios_simulated: usize,
    pub duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubconsciousInfo {
    pub dreams: Vec<Dream>,
    pub intuitions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dream {
    pub id: String,
    pub layer: String,
    pub discovery_time: String,
    pub horizon_days: usize,
    pub probability: f64,
    pub impact: String,
    pub condition: String,
}

impl GatReport {
    pub fn to_toml(&self) -> String {
        toml::to_string_pretty(self).unwrap_or_else(|_| "Error serializing report".to_string())
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "Error serializing report".to_string())
    }
}
