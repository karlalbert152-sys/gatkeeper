use gatkeeper_core::{Finding, Severity};
use gatkeeper_subconscious::SubconsciousEngine;
use std::path::Path;

#[test]
fn test_subconscious_engine() {
    let dir = std::env::temp_dir().join("gatkeeper_test_sub");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.rs"), "fn main() {}").unwrap();

    let findings = vec![
        Finding::new(
            "SecurityAgent",
            Severity::Critical,
            "f.rs",
            "code_injection",
            "eval",
        ),
        Finding::new(
            "SecurityAgent",
            Severity::High,
            "f.rs",
            "auth_password",
            "password check",
        ),
        Finding::new(
            "PerformanceAgent",
            Severity::Medium,
            "f.rs",
            "n_plus_one",
            "N+1 query",
        ),
    ];

    let result = SubconsciousEngine::run_cycle(&dir, &findings, 24);
    assert!(result.scenarios_simulated > 0);
    assert!(!result.dreams.is_empty());
    assert!(!result.intuitions.is_empty());

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_rouge_layer() {
    use gatkeeper_subconscious::rouge::RougeLayer;

    let findings = vec![
        Finding::new("SecurityAgent", Severity::Critical, "f.rs", "type", "desc"),
        Finding::new("SecurityAgent", Severity::High, "f.rs", "type", "desc"),
    ];

    let result = RougeLayer::analyze(Path::new("."), &findings, 24);
    assert!(!result.dreams.is_empty());
    assert_eq!(result.dreams[0].couche, "Rouge (Menace)");
}

#[test]
fn test_jaune_layer() {
    use gatkeeper_subconscious::jaune::JauneLayer;

    let findings = vec![Finding::new(
        "PerformanceAgent",
        Severity::High,
        "f.rs",
        "n_plus_one",
        "desc",
    )];

    let result = JauneLayer::analyze(Path::new("."), &findings);
    assert!(!result.dreams.is_empty());
    assert_eq!(result.dreams[0].couche, "Jaune (Chaos)");
}

#[test]
fn test_verte_layer() {
    use gatkeeper_subconscious::verte::VerteLayer;

    let findings = vec![];

    let result = VerteLayer::analyze(Path::new("."), &findings);
    assert!(!result.intuitions.is_empty());
}
