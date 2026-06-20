use crate::orchestrator::AgentOrchestrator;

pub mod injections;
pub mod crypto;
pub mod owasp;

pub struct SecurityAgent;

impl SecurityAgent {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl crate::traits::Agent for SecurityAgent {
    fn name(&self) -> &str {
        "SecurityAgent"
    }

    fn description(&self) -> &str {
        "Detects vulnerabilities and attack vectors: SQL/XSS/LDAP injections, timing attacks, weak cryptography, OWASP Top 10"
    }

    async fn analyze(&self, file: &std::path::Path, source: &str) -> Vec<gatkeeper_core::Finding> {
        let mut findings = Vec::new();

        // SQL Injection patterns
        findings.extend(injections::check_sql_injections(file, source));

        // Weak cryptography
        findings.extend(crypto::check_weak_crypto(file, source));

        // OWASP Top 10 patterns
        findings.extend(owasp::check_owasp_patterns(file, source));

        findings
    }
}
