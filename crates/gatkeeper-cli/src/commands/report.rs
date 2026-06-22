use gatkeeper_core::GatKeeperConfig;
use std::path::Path;

pub fn execute(project_root: &Path, format: &str) -> anyhow::Result<()> {
    let config = GatKeeperConfig::load(project_root).unwrap_or_default();
    let gat_dir = config.gatkeeper_dir(project_root);
    let scan_path = gat_dir.join("last_scan.toml");

    if !scan_path.exists() {
        anyhow::bail!("No scan results found. Run 'gatkeeper scan' first.");
    }

    let content = std::fs::read_to_string(&scan_path)?;
    let report: gatkeeper_core::GatReport = toml::from_str(&content)?;

    let output = match format {
        "json" => report.to_json(),
        "gat" | "toml" => report.to_toml(),
        _ => {
            anyhow::bail!("Unsupported format: {} (use 'gat' or 'json')", format);
        }
    };

    let output_name = format!("report.{}", format);
    let output_path = gat_dir.join(&output_name);
    std::fs::write(&output_path, &output)?;

    println!("Report generated: {}", output_path.display());
    println!("  Format: {}", format);
    println!("  Project: {}", report.session.project);
    println!("  Risk: {}/100", report.risk_score.global);

    let (c, h, m, l, i) = report.finding_counts();
    println!(
        "  Findings: {} critical, {} high, {} medium, {} low, {} info",
        c, h, m, l, i
    );

    Ok(())
}
