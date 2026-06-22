use gatkeeper_core::GatKeeperConfig;
use std::path::Path;

pub fn execute(project_root: &Path) -> anyhow::Result<()> {
    let config = match GatKeeperConfig::load(project_root) {
        Ok(c) => c,
        Err(_) => {
            println!("No GatKeeper project found in {}", project_root.display());
            println!("Run 'gatkeeper init' to initialize.");
            return Ok(());
        }
    };

    println!("Project: {}", config.project.name);
    println!("Version: {}", config.project.version);
    println!();

    let gat_dir = config.gatkeeper_dir(project_root);
    let scan_path = gat_dir.join("last_scan.toml");

    if scan_path.exists() {
        let content = std::fs::read_to_string(&scan_path)?;
        let report: gatkeeper_core::GatReport = toml::from_str(&content)?;

        println!("Last scan: {}", report.session.timestamp);
        println!("Duration: {}", report.session.duration);
        println!("Files analyzed: {}", report.session.lines_analyzed);
        println!();

        println!("Risk scores:");
        println!("  Global:      {}/100", report.risk_score.global);
        println!("  Security:    {}/100", report.risk_score.security);
        println!("  Performance: {}/100", report.risk_score.performance);
        println!("  Compliance:  {}/100", report.risk_score.compliance);
        println!("  Tendency:    {}", report.risk_score.tendency);

        let (c, h, m, l, i) = report.finding_counts();
        println!();
        println!(
            "Findings: {} critical, {} high, {} medium, {} low, {} info",
            c, h, m, l, i
        );

        if report.risk_score.is_blocking(config.risk.block_merge_below) {
            println!();
            println!(
                "⚠ Risk score {} is below merge threshold {}",
                report.risk_score.global, config.risk.block_merge_below
            );
        }
    } else {
        println!("No scan results yet. Run 'gatkeeper scan' first.");
    }

    Ok(())
}
