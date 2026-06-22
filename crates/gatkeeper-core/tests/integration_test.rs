use gatkeeper_core::{Finding, GatKeeperConfig, GatReport, RiskScore, Severity};

#[test]
fn test_finding_creation() {
    let f = Finding::new(
        "SecurityAgent",
        Severity::Critical,
        "src/main.rs",
        "test",
        "test desc",
    );
    assert_eq!(f.agent, "SecurityAgent");
    assert_eq!(f.severity, Severity::Critical);
    assert!(f.id.starts_with("GAT-CRITICAL-"));
}

#[test]
fn test_finding_with_lines() {
    let f = Finding::new("Agent", Severity::High, "file.rs", "type", "desc").with_lines(10, 20);
    assert_eq!(f.line_start, Some(10));
    assert_eq!(f.line_end, Some(20));
}

#[test]
fn test_risk_score_no_findings() {
    let score = RiskScore::compute(&[], None);
    assert_eq!(score.global, 100);
    assert_eq!(score.security, 100);
}

#[test]
fn test_risk_score_critical_findings() {
    let findings = vec![
        Finding::new("SecurityAgent", Severity::Critical, "f.rs", "type", "desc"),
        Finding::new("SecurityAgent", Severity::Critical, "f.rs", "type", "desc"),
    ];
    let score = RiskScore::compute(&findings, None);
    assert!(score.global < 100);
    assert!(score.security < 100);
}

#[test]
fn test_risk_score_tendency() {
    let prev = RiskScore {
        global: 80,
        security: 80,
        performance: 80,
        compliance: 80,
        tendency: "stable".to_string(),
    };
    let findings = vec![
        Finding::new("SecurityAgent", Severity::Critical, "f.rs", "type", "desc"),
        Finding::new("SecurityAgent", Severity::Critical, "f.rs", "type", "desc"),
        Finding::new("SecurityAgent", Severity::Critical, "f.rs", "type", "desc"),
    ];
    let current = RiskScore::compute(&findings, Some(&prev));
    assert!(current.global < prev.global);
    assert_eq!(current.tendency, "down");
}

#[test]
fn test_report_serialization() {
    let scan = gatkeeper_core::ScanResult {
        project_name: "test".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
        duration_ms: 100,
        files_analyzed: 5,
        lines_analyzed: 100,
        agents_used: vec!["SecurityAgent".to_string()],
        findings: vec![],
        risk_score: RiskScore::default(),
    };

    let report = GatReport::from_scan(&scan, None);
    let toml = report.to_toml();
    assert!(toml.contains("test"));

    let json = report.to_json();
    assert!(json.contains("test"));
}

#[test]
fn test_config_default() {
    let config = GatKeeperConfig::default();
    assert_eq!(config.project.name, "unknown");
    assert_eq!(config.scan.languages.len(), 5);
}
