use gatkeeper_agents::all_agents;
use gatkeeper_parser::parse_file;
use std::path::Path;

#[test]
fn test_all_agents_have_names() {
    let agents = all_agents();
    assert_eq!(agents.len(), 6);
    for agent in &agents {
        assert!(!agent.name().is_empty());
    }
}

#[test]
fn test_security_agent_finds_unsafe() {
    let dir = std::env::temp_dir().join("gatkeeper_test_sec");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("main.rs");
    std::fs::write(&file, "fn main() { let x = unsafe { 5 }; }").unwrap();

    let parsed = parse_file(&file).unwrap();
    let agents = all_agents();
    let security = agents.iter().find(|a| a.name() == "SecurityAgent").unwrap();
    let findings = security.analyze(&[parsed]);
    assert!(!findings.is_empty());

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_secret_agent_finds_api_key() {
    let dir = std::env::temp_dir().join("gatkeeper_test_secret");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("config.py");
    std::fs::write(&file, "api_key = \"super_secret_key_12345678\"").unwrap();

    let parsed = parse_file(&file).unwrap();
    let agents = all_agents();
    let secret = agents.iter().find(|a| a.name() == "SecretAgent").unwrap();
    let findings = secret.analyze(&[parsed]);
    assert!(!findings.is_empty());

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_performance_agent_finds_clone_in_loop() {
    let dir = std::env::temp_dir().join("gatkeeper_test_perf");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("main.rs");
    std::fs::write(&file, "fn main() { for i in vec.clone() {} }").unwrap();

    let parsed = parse_file(&file).unwrap();
    let agents = all_agents();
    let perf = agents.iter().find(|a| a.name() == "PerformanceAgent").unwrap();
    let findings = perf.analyze(&[parsed]);
    assert!(!findings.is_empty());

    std::fs::remove_dir_all(&dir).unwrap();
}
