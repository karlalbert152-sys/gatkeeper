use crate::{Dream, LayerResult};
use gatkeeper_core::finding::{Finding, Severity};
use std::path::Path;

pub struct RougeLayer;

impl RougeLayer {
    pub fn analyze(
        _project_root: &Path,
        findings: &[Finding],
        projection_hours: u32,
    ) -> LayerResult {
        let mut dreams = Vec::new();
        let mut intuitions = Vec::new();
        let mut scenarios = 0u32;

        let critical_findings: Vec<_> = findings
            .iter()
            .filter(|f| matches!(f.severity, Severity::Critical | Severity::High))
            .collect();

        for finding in &critical_findings {
            scenarios += 1;

            let (_horizon, probability) = match finding.severity {
                Severity::Critical => ("24 heures", 0.85),
                Severity::High => ("7 jours", 0.65),
                _ => ("30 jours", 0.35),
            };

            let horizon = if projection_hours <= 24 {
                "24 heures".to_string()
            } else if projection_hours <= 168 {
                "7 jours".to_string()
            } else {
                "30 jours".to_string()
            };

            dreams.push(Dream {
                id: format!("ROUGE-{:03}", dreams.len() + 1),
                couche: "Rouge (Menace)".to_string(),
                decouverte_a: chrono::Utc::now().format("%H:%M").to_string(),
                horizon: horizon.to_string(),
                probabilite: probability,
                impact: format!(
                    "{}: {} dans {}",
                    finding.severity.label(),
                    finding.description,
                    finding.file
                ),
                condition: format!(
                    "Si la vulnérabilité '{}' n'est pas corrigée",
                    finding.finding_type
                ),
            });
        }

        if critical_findings.len() > 3 {
            intuitions.push(format!(
                "Pattern de menace: {} vulnérabilités critiques/haut — attaque en chaîne possible",
                critical_findings.len()
            ));
        }

        let has_auth_issues = findings.iter().any(|f| {
            f.finding_type.contains("auth")
                || f.finding_type.contains("password")
                || f.finding_type.contains("token")
        });
        if has_auth_issues {
            intuitions.push(
                "Zone de faiblesse: authentification — cible prioritaire pour attaquants".to_string(),
            );
        }

        let has_crypto = findings.iter().any(|f| f.finding_type.contains("crypto"));
        if has_crypto {
            intuitions.push(
                "Risque cryptographique: algorithmes faibles détectés — chiffrement compromettable".to_string(),
            );
        }

        LayerResult {
            dreams,
            intuitions,
            scenarios,
        }
    }
}
