pub mod detector;

pub struct SecretAgent;

impl SecretAgent {
    pub fn new() -> Self { Self }
}

#[async_trait::async_trait]
impl crate::traits::Agent for SecretAgent {
    fn name(&self) -> &str { "SecretAgent" }
    fn description(&self) -> &str {
        "Detects hardcoded secrets: API keys, JWT tokens, passwords in plaintext"
    }
    async fn analyze(&self, file: &std::path::Path, source: &str) -> Vec<gatkeeper_core::Finding> {
        detector::detect_secrets(file, source)
    }
}
