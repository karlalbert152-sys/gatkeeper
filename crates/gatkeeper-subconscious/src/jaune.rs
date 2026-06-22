use crate::{Dream, LayerResult};
use gatkeeper_core::finding::Finding;
use std::path::Path;

pub struct JauneLayer;

impl JauneLayer {
    pub fn analyze(_project_root: &Path, findings: &[Finding]) -> LayerResult {
        let mut dreams = Vec::new();
        let mut intuitions = Vec::new();
        let mut scenarios = 0u32;

        let perf_findings: Vec<_> = findings
            .iter()
            .filter(|f| f.agent == "PerformanceAgent")
            .collect();

        let logic_findings: Vec<_> = findings
            .iter()
            .filter(|f| f.agent == "LogicAgent")
            .collect();

        for finding in &perf_findings {
            scenarios += 1;
            dreams.push(Dream {
                id: format!("JAUNE-{:03}", dreams.len() + 1),
                couche: "Jaune (Chaos)".to_string(),
                decouverte_a: chrono::Utc::now().format("%H:%M").to_string(),
                horizon: "7 jours".to_string(),
                probabilite: 0.55,
                impact: format!(
                    "Dégradation performance: {} dans {}",
                    finding.description, finding.file
                ),
                condition: format!(
                    "Sous charge élevée, '{}' amplifiera la latence",
                    finding.finding_type
                ),
            });
        }

        for finding in &logic_findings {
            scenarios += 1;
            dreams.push(Dream {
                id: format!("JAUNE-{:03}", dreams.len() + 1),
                couche: "Jaune (Chaos)".to_string(),
                decouverte_a: chrono::Utc::now().format("%H:%M").to_string(),
                horizon: "30 jours".to_string(),
                probabilite: 0.40,
                impact: format!(
                    "Défaillance logique: {} dans {}",
                    finding.description, finding.file
                ),
                condition: format!(
                    "En cas de conditions concurrentes, '{}' peut causer un deadlock",
                    finding.finding_type
                ),
            });
        }

        let has_n_plus_one = findings
            .iter()
            .any(|f| f.finding_type.contains("n_plus_one"));
        if has_n_plus_one {
            intuitions.push(
                "Point de chaos: N+1 queries — sous charge, la base de données sera le goulot"
                    .to_string(),
            );
        }

        let has_memory = findings.iter().any(|f| {
            f.finding_type.contains("memory")
                || f.finding_type.contains("leak")
                || f.finding_type.contains("allocation")
        });
        if has_memory {
            intuitions.push(
                "Risque mémoire: fuites détectées — OOM probable en production prolongée"
                    .to_string(),
            );
        }

        let has_deadlock = findings.iter().any(|f| f.finding_type.contains("deadlock"));
        if has_deadlock {
            intuitions
                .push("Deadlock potentielt: vérifier l'ordre d'acquisition des locks".to_string());
        }

        LayerResult {
            dreams,
            intuitions,
            scenarios,
        }
    }
}
