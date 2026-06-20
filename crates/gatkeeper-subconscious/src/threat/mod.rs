pub mod simulator;

pub enum AttackerProfile {
    ScriptKiddie,
    CybercriminalOrganized,
    APTNationState,
}

pub struct ThreatSimulation {
    pub profile: AttackerProfile,
    pub target_vectors: Vec<String>,
    pub probability: f64,
}
