use tracing::info;

pub struct ChaosInjector;

impl ChaosInjector {
    pub fn new() -> Self {
        Self
    }

    pub fn inject(&self, scenario: &super::ChaosScenario) {
        match scenario {
            super::ChaosScenario::DatabaseDown => {
                info!("Chaos: Simulating database unavailability...");
            }
            super::ChaosScenario::ServiceTimeout => {
                info!("Chaos: Simulating service timeout cascade...");
            }
            super::ChaosScenario::NetworkDegraded => {
                info!("Chaos: Simulating network degradation...");
            }
            super::ChaosScenario::CorruptedInput => {
                info!("Chaos: Injecting malformed/corrupted data...");
            }
            super::ChaosScenario::HighConcurrency => {
                info!("Chaos: Simulating 10x-100x normal traffic...");
            }
            super::ChaosScenario::SinglePointOfFailure => {
                info!("Chaos: Identifying single points of failure...");
            }
        }
    }
}
