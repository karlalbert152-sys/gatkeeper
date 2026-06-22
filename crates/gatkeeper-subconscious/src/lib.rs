pub mod jaune;
pub mod rouge;
pub mod verte;

use gatkeeper_core::finding::Finding;

#[derive(Debug, Clone)]
pub struct SubconsciousResult {
    pub dreams: Vec<Dream>,
    pub intuitions: Vec<String>,
    pub scenarios_simulated: u32,
}

#[derive(Debug, Clone)]
pub struct Dream {
    pub id: String,
    pub couche: String,
    pub decouverte_a: String,
    pub horizon: String,
    pub probabilite: f64,
    pub impact: String,
    pub condition: String,
}

pub struct SubconsciousEngine;

impl SubconsciousEngine {
    pub fn run_cycle(
        project_root: &std::path::Path,
        findings: &[Finding],
        projection_hours: u32,
    ) -> SubconsciousResult {
        let mut dreams = Vec::new();
        let mut intuitions = Vec::new();
        let mut scenarios = 0u32;

        let rouge_results = rouge::RougeLayer::analyze(project_root, findings, projection_hours);
        dreams.extend(rouge_results.dreams);
        intuitions.extend(rouge_results.intuitions);
        scenarios += rouge_results.scenarios;

        let jaune_results = jaune::JauneLayer::analyze(project_root, findings);
        dreams.extend(jaune_results.dreams);
        intuitions.extend(jaune_results.intuitions);
        scenarios += jaune_results.scenarios;

        let verte_results = verte::VerteLayer::analyze(project_root, findings);
        dreams.extend(verte_results.dreams);
        intuitions.extend(verte_results.intuitions);
        scenarios += verte_results.scenarios;

        SubconsciousResult {
            dreams,
            intuitions,
            scenarios_simulated: scenarios,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerResult {
    pub dreams: Vec<Dream>,
    pub intuitions: Vec<String>,
    pub scenarios: u32,
}
