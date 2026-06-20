pub mod injector;

pub enum ChaosScenario {
    DatabaseDown,
    ServiceTimeout,
    NetworkDegraded,
    CorruptedInput,
    HighConcurrency,
    SinglePointOfFailure,
}

pub struct ChaosInjection {
    pub scenario: ChaosScenario,
    pub severity: String,
    pub affected_components: Vec<String>,
}
