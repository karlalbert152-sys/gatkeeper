use crate::{Dream, LayerResult};
use gatkeeper_core::finding::Finding;
use std::path::Path;

pub struct VerteLayer;

impl VerteLayer {
    pub fn analyze(_project_root: &Path, findings: &[Finding]) -> LayerResult {
        let mut dreams = Vec::new();
        let mut intuitions = Vec::new();
        let mut scenarios = 0u32;

        let code_smells: Vec<_> = findings
            .iter()
            .filter(|f| {
                f.finding_type.contains("long_function")
                    || f.finding_type.contains("clone")
                    || f.finding_type.contains("eager")
                    || f.finding_type.contains("deep_nesting")
            })
            .collect();

        for finding in &code_smells {
            scenarios += 1;
            dreams.push(Dream {
                id: format!("VERTE-{:03}", dreams.len() + 1),
                couche: "Verte (Évolution)".to_string(),
                decouverte_a: chrono::Utc::now().format("%H:%M").to_string(),
                horizon: "30 jours".to_string(),
                probabilite: 0.70,
                impact: format!(
                    "Amélioration suggérée: {} dans {}",
                    finding.description, finding.file
                ),
                condition: format!(
                    "Refactoring de '{}' améliorerait la maintenabilité",
                    finding.finding_type
                ),
            });
        }

        let has_unused_imports = findings.iter().any(|f| f.finding_type.contains("unused"));
        if has_unused_imports {
            intuitions.push(
                "Nettoyage possible: imports/fonctions inutilisés détectés — dette technique"
                    .to_string(),
            );
        }

        let has_todo = findings.iter().any(|f| f.finding_type.contains("todo"));
        if has_todo {
            intuitions.push(
                "TODO markers: travail incomplet — prioriser avant la prochaine release"
                    .to_string(),
            );
        }

        if findings.is_empty() {
            intuitions.push(
                "Aucun finding détecté — le code est dans un état sain, maintenir la discipline"
                    .to_string(),
            );
        }

        let security_count = findings
            .iter()
            .filter(|f| f.agent == "SecurityAgent")
            .count();
        let perf_count = findings
            .iter()
            .filter(|f| f.agent == "PerformanceAgent")
            .count();

        if security_count > 0 && perf_count > 0 {
            intuitions.push(
                "Opportunité: résoudre les issues sécurité ET performance en parallèle pour un gain double".to_string(),
            );
        }

        LayerResult {
            dreams,
            intuitions,
            scenarios,
        }
    }
}
