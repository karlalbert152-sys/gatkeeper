use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(
    name = "gatkeeper",
    about = "AI-Native Security Intelligence Platform",
    version,
    long_about = "GatKeeper predicts vulnerabilities before they exist.\nSubconscious Engine monitors your code 24/7."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize GatKeeper in a project
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Run a full security scan
    Scan {
        /// Only run specific agents
        #[arg(short, long)]
        agents: Option<Vec<String>>,
        /// Project root directory
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Generate a .gat report from last scan
    Report {
        /// Output format (gat, json)
        #[arg(short, long, default_value = "gat")]
        format: String,
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Show project status and risk scores
    Status {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Show DNA fingerprint of the project
    Dna {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Run the subconscious engine cycle
    Subconscious {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        #[arg(long, default_value = "24")]
        projection_hours: u32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose { "debug" } else { "info" };

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| filter.into()),
        )
        .init();

    match cli.command {
        Commands::Init { name } => commands::init::execute(name),
        Commands::Scan { agents, path } => commands::scan::execute(&path, agents).await,
        Commands::Report { format, path } => commands::report::execute(&path, &format),
        Commands::Status { path } => commands::status::execute(&path),
        Commands::Dna { path } => commands::dna::execute(&path),
        Commands::Subconscious {
            path,
            projection_hours,
        } => commands::subconscious::execute(&path, projection_hours),
    }
}
