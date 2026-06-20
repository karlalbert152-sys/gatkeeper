use colored::Colorize;

pub fn print_banner() {
    println!("{}", "
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║   ██████╗  █████╗ ███████╗████████╗██╗ ██████╗ ███╗   ██╗║
║  ██╔════╝ ██╔══██╗██╔════╝╚══██╔══╝██║██╔═══██╗████╗  ██║║
║  ██║  ███╗███████║███████╗   ██║   ██║██║   ██║██╔██╗ ██║║
║  ██║   ██║██╔══██║╚════██║   ██║   ██║██║   ██║██║╚██╗██║║
║  ╚██████╔╝██║  ██║███████║   ██║   ██║╚██████╔╝██║ ╚████║║
║   ╚═════╝ ╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══║║
║                                                          ║
║   AI-Native Security Intelligence Platform               ║
║   The first security tool that thinks.                   ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝"
        .cyan()
    );
}

pub fn print_finding(severity: &str, file: &str, line: usize, message: &str) {
    let icon = match severity {
        "CRITICAL" => "🔴",
        "HIGH" => "🟠",
        "MEDIUM" => "🟡",
        "LOW" => "🟢",
        _ => "⚪",
    };
    println!(
        "  {} [{}] {}:{} — {}",
        icon,
        severity.red().bold(),
        file,
        line,
        message
    );
}
