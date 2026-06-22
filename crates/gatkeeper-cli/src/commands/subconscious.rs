use gatkeeper_agents::all_agents;
use gatkeeper_core::GatKeeperConfig;
use gatkeeper_parser::parse_file;
use gatkeeper_subconscious::SubconsciousEngine;
use std::path::Path;

pub fn execute(project_root: &Path, projection_hours: u32) -> anyhow::Result<()> {
    let _config = match GatKeeperConfig::load(project_root) {
        Ok(c) => c,
        Err(_) => {
            println!("Using default configuration");
            GatKeeperConfig::default()
        }
    };

    println!("Running subconscious engine...");
    println!("  Projection: {}h", projection_hours);

    let source_files = collect_source_files(project_root)?;
    let mut parsed_files = Vec::new();
    for path in &source_files {
        if let Ok(parsed) = parse_file(path) {
            parsed_files.push(parsed);
        }
    }

    let agents = all_agents();
    let mut all_findings = Vec::new();
    for agent in &agents {
        let findings = agent.analyze(&parsed_files);
        all_findings.extend(findings);
    }

    println!("  Findings to analyze: {}", all_findings.len());

    let result = SubconsciousEngine::run_cycle(project_root, &all_findings, projection_hours);

    println!();
    println!("Subconscious Results:");
    println!("  Scenarios simulated: {}", result.scenarios_simulated);
    println!("  Dreams: {}", result.dreams.len());
    println!("  Intuitions: {}", result.intuitions.len());

    if !result.dreams.is_empty() {
        println!();
        println!("Dreams (Threat Simulations):");
        for dream in &result.dreams {
            println!(
                "  [{}] {} — prob: {:.0}%, horizon: {}",
                dream.id,
                dream.couche,
                dream.probabilite * 100.0,
                dream.horizon
            );
            println!("    Impact: {}", dream.impact);
            println!("    Condition: {}", dream.condition);
        }
    }

    if !result.intuitions.is_empty() {
        println!();
        println!("Intuitions:");
        for intuition in &result.intuitions {
            println!("  • {}", intuition);
        }
    }

    Ok(())
}

fn collect_source_files(root: &Path) -> anyhow::Result<Vec<std::path::PathBuf>> {
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
                        && name != ".gatkeeper"
                    {
                        stack.push(path);
                    }
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if matches!(ext, "rs" | "py" | "js" | "jsx" | "c" | "h" | "go") {
                        files.push(path);
                    }
                }
            }
        }
    }

    Ok(files)
}
