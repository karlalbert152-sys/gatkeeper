use gatkeeper_agents::all_agents;
use gatkeeper_core::{GatKeeperConfig, RiskScore, Store};
use gatkeeper_parser::parse_file;
use gatkeeper_subconscious::SubconsciousEngine;
use std::path::Path;
use std::time::Instant;

pub async fn execute(
    project_root: &Path,
    agents_filter: Option<Vec<String>>,
) -> anyhow::Result<()> {
    let config = GatKeeperConfig::load(project_root).unwrap_or_else(|_| {
        println!("No gatkeeper.toml found, using defaults");
        GatKeeperConfig::default()
    });

    let start = Instant::now();
    println!("Scanning project '{}'...", config.project.name);

    let source_files = collect_source_files(project_root, &config.scan.languages)?;
    println!("  Found {} source files", source_files.len());

    let mut parsed_files = Vec::new();
    let mut errors = 0u32;
    for path in &source_files {
        match parse_file(path) {
            Ok(parsed) => parsed_files.push(parsed),
            Err(e) => {
                tracing::warn!("Failed to parse {}: {}", path.display(), e);
                errors += 1;
            }
        }
    }
    println!("  Parsed {} files ({} errors)", parsed_files.len(), errors);

    let total_lines: u64 = parsed_files.iter().map(|f| f.line_count as u64).sum();
    println!("  Total lines: {}", total_lines);

    let agents = all_agents();
    let mut all_findings = Vec::new();
    let mut agents_used = Vec::new();

    for agent in &agents {
        if let Some(ref filter) = agents_filter {
            if !filter.iter().any(|a| a == agent.name()) {
                continue;
            }
        }

        println!("  Running {}...", agent.name());
        let findings = agent.analyze(&parsed_files);
        println!("    Found {} issues", findings.len());
        all_findings.extend(findings);
        agents_used.push(agent.name().to_string());
    }

    let risk_score = RiskScore::compute(&all_findings, None);

    let duration = start.elapsed();
    println!();
    println!("Scan complete in {}ms", duration.as_millis());
    println!("  Files analyzed: {}", parsed_files.len());
    println!("  Lines analyzed: {}", total_lines);
    println!("  Findings: {}", all_findings.len());
    println!("  Risk score: {}/100", risk_score.global);

    if !all_findings.is_empty() {
        println!();
        println!("Findings by severity:");
        for severity in &[
            gatkeeper_core::finding::Severity::Critical,
            gatkeeper_core::finding::Severity::High,
            gatkeeper_core::finding::Severity::Medium,
            gatkeeper_core::finding::Severity::Low,
            gatkeeper_core::finding::Severity::Info,
        ] {
            let count = all_findings
                .iter()
                .filter(|f| f.severity == *severity)
                .count();
            if count > 0 {
                println!("  {}: {}", severity.label(), count);
            }
        }
    }

    if config.subconscious.enabled {
        println!();
        println!("Running subconscious engine...");
        let result = SubconsciousEngine::run_cycle(
            project_root,
            &all_findings,
            config.subconscious.projection_hours,
        );
        println!("  Scenarios simulated: {}", result.scenarios_simulated);
        println!("  Dreams: {}", result.dreams.len());
        println!("  Intuitions: {}", result.intuitions.len());
    }

    let scan = gatkeeper_core::ScanResult {
        project_name: config.project.name.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        duration_ms: duration.as_millis() as u64,
        files_analyzed: parsed_files.len() as u32,
        lines_analyzed: total_lines,
        agents_used,
        findings: all_findings,
        risk_score,
    };

    save_scan_results(project_root, &config, &scan)?;

    match Store::open(project_root) {
        Ok(store) => {
            let root_path = project_root.display().to_string();
            if let Ok(project_id) = store.ensure_project(&config.project.name, &root_path) {
                if let Ok(scan_id) = store.save_scan(project_id, &scan) {
                    println!("  Saved to database (scan #{})", scan_id);
                }
            }
        }
        Err(e) => {
            tracing::warn!("Could not save to database: {}", e);
        }
    }

    Ok(())
}

fn collect_source_files(
    root: &Path,
    languages: &[String],
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let extensions: Vec<&str> = languages
        .iter()
        .flat_map(|lang| match lang.as_str() {
            "rust" => vec!["rs"],
            "python" => vec!["py"],
            "javascript" => vec!["js", "jsx"],
            "c" => vec!["c", "h"],
            "go" => vec!["go"],
            _ => vec![],
        })
        .collect();

    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if !name.starts_with('.')
                        && name != "target"
                        && name != "node_modules"
                        && name != "vendor"
                        && name != ".gatkeeper"
                    {
                        stack.push(path);
                    }
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if extensions.contains(&ext) {
                        files.push(path);
                    }
                }
            }
        }
    }

    Ok(files)
}

fn save_scan_results(
    project_root: &Path,
    config: &GatKeeperConfig,
    scan: &gatkeeper_core::ScanResult,
) -> anyhow::Result<()> {
    let gat_dir = config.gatkeeper_dir(project_root);
    std::fs::create_dir_all(&gat_dir)?;

    let report = gatkeeper_core::GatReport::from_scan(scan, None);
    let report_path = gat_dir.join("last_scan.toml");
    std::fs::write(&report_path, report.to_toml())?;

    println!("  Report saved: {}", report_path.display());

    Ok(())
}
