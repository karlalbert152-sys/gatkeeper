use chrono::{DateTime, Utc};
use tracing::info;

pub enum CyclePhase {
    ScanLight,      // Every hour, ~2 min
    Nightly,        // Every night, ~45 min
    DeepSimulation, // Weekly, ~6 hours
    GrandAudit,     // Quarterly, ~24 hours
}

impl CyclePhase {
    pub fn interval_secs(&self) -> u64 {
        match self {
            CyclePhase::ScanLight => 3600,
            CyclePhase::Nightly => 86400,
            CyclePhase::DeepSimulation => 604800,
            CyclePhase::GrandAudit => 7776000, // 90 days
        }
    }

    pub fn duration_estimate(&self) -> &'static str {
        match self {
            CyclePhase::ScanLight => "~2 minutes",
            CyclePhase::Nightly => "~45 minutes",
            CyclePhase::DeepSimulation => "~6 hours",
            CyclePhase::GrandAudit => "~24 hours",
        }
    }
}

pub struct SimulationScheduler {
    last_nightly: Option<DateTime<Utc>>,
    last_deep: Option<DateTime<Utc>>,
    last_audit: Option<DateTime<Utc>>,
}

impl SimulationScheduler {
    pub fn new() -> Self {
        Self {
            last_nightly: None,
            last_deep: None,
            last_audit: None,
        }
    }

    pub fn should_run_nightly(&self) -> bool {
        match self.last_nightly {
            Some(last) => Utc::now().signed_duration_since(last).num_hours() >= 24,
            None => true,
        }
    }

    pub fn should_run_deep(&self) -> bool {
        match self.last_deep {
            Some(last) => Utc::now().signed_duration_since(last).num_days() >= 7,
            None => true,
        }
    }

    pub fn should_run_audit(&self) -> bool {
        match self.last_audit {
            Some(last) => Utc::now().signed_duration_since(last).num_days() >= 90,
            None => true,
        }
    }
}
