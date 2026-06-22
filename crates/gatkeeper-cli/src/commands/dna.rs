use gatkeeper_core::GatKeeperConfig;
use gatkeeper_dna::{DnaFingerprint, PatternDetector, InvariantExtractor};
use std::path::Path;

pub fn execute(project_root: &Path) -> anyhow::Result<()> {
    let _config = GatKeeperConfig::load(project_root).unwrap_or_default();

    println!("Computing DNA fingerprint...");
    let fingerprint = DnaFingerprint::compute(project_root);

    println!();
    println!("DNA Fingerprint:");
    println!("  Root hash:   {}", &fingerprint.root_hash[..16]);
    println!("  Files:       {}", fingerprint.file_count);
    println!("  Total lines: {}", fingerprint.total_lines);

    println!();
    println!("Detecting patterns...");
    let patterns = PatternDetector::detect(project_root);
    if patterns.is_empty() {
        println!("  No architectural patterns detected");
    } else {
        for p in &patterns {
            println!(
                "  {} (confidence: {:.0}%) — {} files",
                p.name,
                p.confidence * 100.0,
                p.files.len()
            );
        }
    }

    println!();
    println!("Extracting invariants...");
    let invariants = InvariantExtractor::extract(project_root);
    if invariants.is_empty() {
        println!("  No invariants detected");
    } else {
        for inv in invariants.iter().take(10) {
            println!(
                "  {} — {} ({}:{})",
                inv.name,
                inv.rule,
                inv.file,
                inv.line.unwrap_or(0)
            );
        }
        if invariants.len() > 10 {
            println!("  ... and {} more", invariants.len() - 10);
        }
    }

    Ok(())
}
