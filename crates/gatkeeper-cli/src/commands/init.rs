use gatkeeper_core::GatKeeperConfig;

pub fn execute(name: Option<String>) -> anyhow::Result<()> {
    let root = std::env::current_dir()?;
    let gat_dir = root.join(".gatkeeper");

    if gat_dir.exists() {
        anyhow::bail!("GatKeeper already initialized in this directory");
    }

    std::fs::create_dir_all(&gat_dir)?;

    let project_name = name.unwrap_or_else(|| {
        root.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    });

    let config = GatKeeperConfig {
        project: gatkeeper_core::config::ProjectConfig {
            name: project_name.clone(),
            version: "0.0.1".to_string(),
        },
        ..Default::default()
    };

    config.save(&root)?;

    println!("GatKeeper initialized for project '{}'", project_name);
    println!("  Config: gatkeeper.toml");
    println!("  Data:   .gatkeeper/");
    println!();
    println!("Next steps:");
    println!("  1. Review gatkeeper.toml");
    println!("  2. Run: gatkeeper scan");

    Ok(())
}
