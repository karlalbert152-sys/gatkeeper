use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod output;

#[derive(Parser)]
#[command(
    name = "gatkeeper",
    about = "GatKeeper — AI-Native Security Intelligence Platform",
    version,
    long_about = "A living security platform that thinks, predicts, and protects your codebase 24/7."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize GatKeeper in the current project
    Init,
    /// Run a full security scan on the codebase
    Scan {
        /// Target directory to scan
        #[arg(default_value = ".")]
        path: String,
        /// Agent(s) to run (security, logic, performance, compliance, secrets, supply_chain, all)
        #[arg(short, long, default_value = "all")]
        agent: String,
        /// Output format (text, json, gat)
        #[arg(short, long, default_value = "gat")]
        output: String,
    },
    /// Generate a report from the last scan
    Report {
        /// Report format (gat, json, html)
        #[arg(short, long, default_value = "gat")]
        format: String,
    },
    /// Show project status and risk score
    Status,
    /// Simulate what-if scenarios (Phase 2+)
    Whatif {
        /// The scenario to simulate
        scenario: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gatkeeper=info".into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::execute().await,
        Commands::Scan { path, agent, output } => commands::scan::execute(&path, &agent, &output).await,
        Commands::Report { format } => commands::report::execute(&format).await,
        Commands::Status => commands::status::execute().await,
        Commands::Whatif { scenario } => commands::whatif::execute(&scenario).await,
    }
}
