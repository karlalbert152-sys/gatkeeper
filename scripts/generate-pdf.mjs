import PDFDocument from 'pdfkit';
import fs from 'fs';

const BLUE = '#1a365d';
const LIGHT_BLUE = '#2b6cb0';
const GRAY = '#4a5568';
const LIGHT_GRAY = '#e2e8f0';
const DARK = '#1a202c';
const WHITE = '#ffffff';
const RED = '#e53e3e';
const GREEN = '#38a169';
const YELLOW = '#d69e2e';

const doc = new PDFDocument({
  size: 'A4',
  margins: { top: 60, bottom: 60, left: 60, right: 60 },
  info: {
    Title: 'GatKeeper — AI-Native Security Intelligence Platform',
    Author: 'Karla Albert',
    Subject: 'Project Analysis & Architecture Document',
    CreationDate: new Date(),
  }
});

const stream = fs.createWriteStream('./docs/GatKeeper_Analysis_EN.pdf');
doc.pipe(stream);

function addHeader(text, size = 24) {
  doc.fontSize(size).font('Helvetica-Bold').fillColor(BLUE).text(text);
  doc.moveDown(0.3);
  doc.strokeColor(LIGHT_BLUE).lineWidth(2).moveTo(doc.x, doc.y).lineTo(doc.x + 400, doc.y).stroke();
  doc.moveDown(0.5);
}

function addSubHeader(text, size = 16) {
  doc.fontSize(size).font('Helvetica-Bold').fillColor(LIGHT_BLUE).text(text);
  doc.moveDown(0.3);
}

function addBody(text, size = 10) {
  doc.fontSize(size).font('Helvetica').fillColor(DARK).text(text, { lineGap: 4 });
  doc.moveDown(0.3);
}

function addBullet(text, indent = 20) {
  doc.fontSize(10).font('Helvetica').fillColor(DARK).text(`•  ${text}`, indent, doc.y, { width: 480 - indent });
  doc.moveDown(0.15);
}

function addTable(headers, rows, colWidths) {
  const startX = 60;
  let y = doc.y;

  // Header
  doc.font('Helvetica-Bold').fontSize(9).fillColor(WHITE);
  let x = startX;
  headers.forEach((h, i) => {
    doc.rect(x, y, colWidths[i], 22).fill(BLUE);
    doc.fillColor(WHITE).text(h, x + 5, y + 6, { width: colWidths[i] - 10 });
    x += colWidths[i];
  });
  y += 22;

  // Rows
  doc.font('Helvetica').fontSize(8).fillColor(DARK);
  rows.forEach((row, ri) => {
    const bgColor = ri % 2 === 0 ? LIGHT_GRAY : WHITE;
    x = startX;
    row.forEach((cell, ci) => {
      doc.rect(x, y, colWidths[ci], 18).fill(bgColor);
      doc.fillColor(DARK).text(String(cell).substring(0, 60), x + 5, y + 4, { width: colWidths[ci] - 10 });
      x += colWidths[ci];
    });
    y += 18;
  });

  doc.y = y + 5;
}

// ==================== COVER PAGE ====================
doc.rect(0, 0, 595, 842).fill(BLUE);
doc.fontSize(42).font('Helvetica-Bold').fillColor(WHITE).text('GatKeeper', 60, 200, { width: 480 });
doc.fontSize(20).font('Helvetica').fillColor(LIGHT_BLUE).text('AI-Native Security Intelligence Platform', 60, 260, { width: 480 });
doc.moveDown(2);
doc.fontSize(14).fillColor(GRAY).font('Helvetica').text('Project Analysis & Architecture Document', 60, 340);
doc.moveDown(1);
doc.fontSize(12).fillColor(WHITE).text('Version 1.0  |  June 2026', 60, 380);
doc.moveDown(0.5);
doc.text('Author: Karla Albert <karlalbert152@gmail.com>', 60, 400);
doc.moveDown(0.5);
doc.text('Status: Phase 1 — MVP Architecture', 60, 420);
doc.moveDown(4);
doc.fontSize(10).fillColor(GRAY).text('CONFIDENTIAL', 60, 580);
doc.addPage();

// ==================== TABLE OF CONTENTS ====================
addHeader('Table of Contents');
const toc = [
  ['1', 'Executive Summary', '3'],
  ['2', 'Problem Statement & Market Analysis', '4'],
  ['3', 'Solution & Strategic Positioning', '5'],
  ['4', 'Architecture — 4-Layer Design', '6'],
  ['5', 'Tech Stack & Design Justifications', '7'],
  ['6', 'Phase 1 MVP — Directory Structure', '8'],
  ['7', 'Multi-Agent Analysis System', '9'],
  ['8', 'Subconscious Engine Deep Dive', '10'],
  ['9', 'Competitive Analysis', '11'],
  ['10', '.gat Report Format Specification', '12'],
  ['11', 'Phase 1 Roadmap (3 Months)', '13'],
  ['12', 'Monetization & Go-to-Market', '14'],
];

toc.forEach(([num, title, page]) => {
  doc.fontSize(11).font('Helvetica').fillColor(DARK)
    .text(`${num}.  ${title}`, 80, doc.y, { continued: true, width: 380 })
    .fillColor(GRAY).text(`  ${page}`, { align: 'right' });
  doc.moveDown(0.2);
});
doc.addPage();

// ==================== 1. EXECUTIVE SUMMARY ====================
addHeader('1. Executive Summary');
addBody('GatKeeper is a next-generation code intelligence platform designed to address one of the most critical challenges of our digital era: software security at scale. Unlike existing tools that only detect problems already present in the code, GatKeeper adopts a radically different approach: anticipate, simulate, and predict vulnerabilities before they exist.');
addBody('The platform combines a multi-agent architecture based on specialized AI models, an autonomous simulation engine (the Subconscious Engine), and an evolving codebase memory to deliver unprecedented security coverage. GatKeeper targets both individual developers and large organizations (Fortune 500, financial institutions, governments) with strict compliance requirements (GDPR, SOC2, ISO27001, HIPAA).');

addSubHeader('Key Metrics');
addTable(
  ['Metric', 'Value', 'Context'],
  [
    ['Scenarios simulated / night', '48,000+', 'By Subconscious Engine'],
    ['Specialized AI agents', '6 agents', 'Each fine-tuned on its domain'],
    ['Temporal projection', '30 days', 'Prospective code simulation'],
    ['Supported languages', '15+', 'C, C++, Rust, Python, JS, Go...'],
    ['Addressable market', '$25B', 'Global software security 2025'],
    ['Estimated client savings', '80%', 'vs. traditional external audit'],
  ],
  [160, 100, 240]
);
doc.addPage();

// ==================== 2. PROBLEM STATEMENT ====================
addHeader('2. Problem Statement & Market Analysis');
addSubHeader('2.1 The Silent Crisis of Software Security');
addBody('We live in a paradoxical era: never have development tools been so powerful, and yet software security vulnerabilities are reaching record levels. In 2024, the average cost of a data breach exceeded $4.5 million per incident. Large organizations face multiple structural problems simultaneously:');
addBullet('Massive codebases (millions of lines) impossible to manually audit with rigor');
addBullet('Dozens of teams developing in parallel creating unforeseen interactions between modules');
addBullet('External dependencies (npm, crates.io, PyPI) introducing third-party vulnerabilities at each update');
addBullet('Expensive external security audits ($100K-$500K per audit) that are infrequent');
addBullet('Fragmented tools that don\'t communicate: SAST, DAST, SCA, secret scanners — each in its silo');
addBullet('Increasingly stringent regulatory compliance (GDPR, SOC2, HIPAA) with sanctions');

addSubHeader('2.2 Limitations of Current Tools');
addBody('Existing solutions suffer from a fundamental flaw: they are reactive. They find what is already broken, never what will be. They analyze syntax and some known patterns, but do not understand the code as a whole:');
addBullet('SonarQube and SAST tools: static analysis without semantic understanding, high false positive rate');
addBullet('Snyk and SCA tools: focused on dependencies, ignoring internal logic');
addBullet('Manual pentest tools: expensive, episodic, not integrated into the development cycle');
addBullet('GitHub Copilot and AI assistants: suggest code but do not audit in depth');
addBullet('No tool combines evolutionary memory, prospective analysis, and multi-layer analysis');
doc.addPage();

// ==================== 3. SOLUTION ====================
addHeader('3. Solution & Strategic Positioning');
addSubHeader('3.1 Founding Vision');
addBody('GatKeeper is the first living code intelligence platform — a system that learns the DNA of your codebase, simulates thousands of possible futures in the background, and proactively alerts before problems arrive. Like a doctor who predicts disease before symptoms rather than treating it once declared.');
addBody('GatKeeper unifies in a single platform what today requires 5 to 8 distinct tools: static analysis, dependency analysis, secret scanning, threat modeling, automatic compliance, load simulation, and attack simulation.');

addSubHeader('3.2 Strategic Positioning');
addTable(
  ['Dimension', 'Classic SAST', 'AI Assistants', 'GatKeeper'],
  [
    ['Approach', 'Reactive', 'Assistive', 'Predictive & proactive'],
    ['Understanding', 'Syntactic', 'Partial contextual', 'Semantic + intentional'],
    ['Project memory', '✗', '✗', '✓'],
    ['Future simulation', '✗', '✗', '✓'],
    ['100% on-premise', 'Partial', '✗', '✓'],
    ['Multi-agents', '✗', '✗', '✓'],
    ['Auto compliance', 'Partial', '✗', '✓'],
  ],
  [120, 120, 120, 140]
);
doc.addPage();

// ==================== 4. ARCHITECTURE ====================
addHeader('4. Architecture — 4-Layer Design');
addBody('The GatKeeper platform is structured around four main layers, each with clearly defined responsibilities and integrated components from the technical stack.');

// ASCII Diagram
doc.fontSize(9).font('Courier').fillColor(DARK);
const archDiagram = `
┌─────────────────────────────────────────────────────────────┐
│                  LAYER 1: INTERFACE                         │
│  CLI (Rust/Clap)  │  Neovim Plugin  │  VSCode Extension    │
│  Dashboard Web (React + TypeScript + WebSockets)            │
├─────────────────────────────────────────────────────────────┤
│                  LAYER 2: INTEGRATION                       │
│  Git Hooks │ GitHub Actions │ GitLab CI │ Jenkins │ API     │
├─────────────────────────────────────────────────────────────┤
│                  LAYER 3: INTELLIGENCE                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Tokio Async Runtime                                  │  │
│  │  ┌─────────────────┐  ┌────────────────────────────┐  │  │
│  │  │ Subconscious Eng│  │ 6 AI Agents (Security,     │  │  │
│  │  │ Rouge/Jaune/Vert│  │ Logic, Perf, Comply,       │  │  │
│  │  │ 24/7 simulation │  │ Secret, SupplyChain)       │  │  │
│  │  └─────────────────┘  └────────────────────────────┘  │  │
│  │  ┌─────────────────┐  ┌────────────────────────────┐  │  │
│  │  │ DNA Engine      │  │ Tree-sitter Parser (15+)   │  │  │
│  │  │ (Git memory)    │  │ C,C++,Rust,Python,JS,Go    │  │  │
│  │  └─────────────────┘  └────────────────────────────┘  │  │
│  │  ┌─────────────────┐  ┌────────────────────────────┐  │  │
│  │  │ Ollama (local)  │  │ Sandbox Attack (gVisor)    │  │  │
│  │  │ codellama       │  │ PoC gen, fuzzing           │  │  │
│  │  └─────────────────┘  └────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                  LAYER 4: DATA                               │
│  SQLite (local) │ PostgreSQL (Ent.) │ CVE DB (OSV + NVD)    │
└─────────────────────────────────────────────────────────────┘`.trim();

doc.text(archDiagram, 60, doc.y, { width: 480 });
doc.moveDown(1);

addSubHeader('Component Responsibilities');
addBullet('INTERFACE: Entry point for developers. CLI for terminal workflows, IDE plugins for real-time feedback, Dashboard for team visibility.');
addBullet('INTEGRATION: Automates security checks in the development cycle. Every commit triggers analysis.');
addBullet('INTELLIGENCE: Core brain. Agents analyze, Subconscious simulates, DNA remembers, Tree-sitter parses.');
addBullet('DATA: Persistent storage. SQLite for local/portable, PostgreSQL for enterprise scale, CVE databases for threat intelligence.');
doc.addPage();

// ==================== 5. TECH STACK ====================
addHeader('5. Tech Stack & Design Justifications');
addTable(
  ['Component', 'Technology', 'Version', 'Justification'],
  [
    ['CLI main', 'Rust', '1.75+', 'Native performance, memory safety guarantees'],
    ['Code parsing', 'Tree-sitter', '0.22+', 'Multi-language (15+), robust, incremental, embeddable'],
    ['Agent orchestration', 'Tokio (async Rust)', '1.35+', 'Millions of concurrent tasks, zero overhead'],
    ['Local LLMs', 'Ollama', 'Latest', 'Zero API cost for nightly subconscious, absolute privacy'],
    ['Cloud LLMs', 'Anthropic Claude', 'claude-3', 'Superior analysis quality for paid tiers'],
    ['Web dashboard', 'React + TypeScript', '18+/5+', 'Rich real-time UI via WebSockets'],
    ['API backend', 'Axum (Rust)', '0.7+', 'Consistent with CLI, excellent HTTP performance'],
    ['Local storage', 'SQLite', '3.45+', 'Lightweight, portable, versionable, no server needed'],
    ['Enterprise DB', 'PostgreSQL', '16+', 'Scalability, transactions, full-text search'],
    ['CVE analysis', 'OSV + NIST NVD API', 'Latest', 'Official databases, open source, real-time updates'],
    ['Attack sandbox', 'gVisor + Docker', 'Latest', 'Kernel-level isolation for secure PoC execution'],
    ['Real-time comms', 'WebSockets', 'Latest', 'Live dashboard, immediate push alerts'],
    ['Report format', 'TOML', '1.0', 'Human-readable, machine-parseable, Git-versionable'],
  ],
  [95, 100, 65, 240]
);
doc.addPage();

// ==================== 6. PHASE 1 DIRECTORY STRUCTURE ====================
addHeader('6. Phase 1 MVP — Directory Structure');
addBody('The Phase 1 MVP directory structure follows a Cargo workspace pattern with clearly separated crates for each major component.');

doc.fontSize(8).font('Courier').fillColor(DARK);
const dirTree = `
gatkeeper/
├── Cargo.toml                         # Workspace root
├── rust-toolchain.toml
├── LICENSE
├── README.md
├── .gitignore
│
├── crates/
│   ├── gatkeeper-cli/                 # Main CLI binary
│   │   └── src/
│   │       ├── main.rs                # Entry (clap)
│   │       ├── commands/              # scan, report, init, status, whatif
│   │       └── output.rs              # Terminal formatting
│   ├── gatkeeper-core/                # Shared business logic
│   │   └── src/
│   │       ├── config.rs              # Project config
│   │       ├── error.rs               # Error types
│   │       ├── project.rs             # Project detection
│   │       └── report/                # .gat, risk_score, findings
│   ├── gatkeeper-parser/              # Tree-sitter multi-language
│   │   └── src/
│   │       ├── languages/             # Rust, Python, JS, C, Go
│   │       ├── ast.rs                 # Unified AST wrapper
│   │       └── walker.rs             # AST walking patterns
│   ├── gatkeeper-agents/              # 6 specialized AI agents
│   │   └── src/
│   │       ├── orchestrator.rs        # Multi-agent orchestration
│   │       ├── traits.rs              # Agent trait definition
│   │       ├── security/              # SQL/XSS, crypto, OWASP
│   │       ├── secrets/               # Hardcoded secrets detector
│   │       └── supply_chain/          # Dependency CVE checking
│   ├── gatkeeper-subconscious/        # Subconscious Engine
│   │   └── src/
│   │       ├── engine.rs              # Main loop (Tokio spawn)
│   │       ├── scheduler.rs           # Cron-like scheduling
│   │       ├── threat/                # Rouge: attacker simulation
│   │       ├── chaos/                 # Jaune: failure injection
│   │       └── evolution/             # Verte: code generation
│   └── gatkeeper-dna/                 # DNA Engine (long-term memory)
│       └── src/
│           ├── fingerprint.rs         # Cryptographic fingerprint
│           ├── patterns.rs           # Architectural pattern detection
│           ├── invariants.rs         # Code invariants
│           ├── baseline.rs           # Git behavioral baseline
│           └── store.rs             # SQLite persistence
│
├── gatkeeper-lenses/                  # Intent comprehension
├── migrations/                        # SQLite schema
├── templates/                         # .gat template, config example
├── tests/                             # Integration tests + fixtures
├── scripts/                           # PDF generation, CI helpers
├── docs/                              # Documentation + PDFs
└── .github/workflows/                # CI/CD pipeline`.trim();

doc.text(dirTree, 50, doc.y, { width: 500 });
doc.moveDown(1);
doc.addPage();

// ==================== 7. MULTI-AGENT SYSTEM ====================
addHeader('7. Multi-Agent Analysis System');
addBody('Six independent AI agents, each fine-tuned on its specific domain, work in parallel on each analysis. This multi-agent architecture enables depth impossible with a single generalist model.');

addTable(
  ['Agent', 'Domain', 'Example Detections'],
  [
    ['SecurityAgent', 'Vulnerabilities & attacks', 'SQL/XSS/LDAP injection, timing attacks, weak crypto, OWASP Top 10'],
    ['LogicAgent', 'Logic bugs & coherence', 'Race conditions, deadlocks, edge cases, inter-module inconsistencies'],
    ['PerformanceAgent', 'Efficiency & resources', 'Memory leaks, N+1 queries, unnecessary allocations, hot paths'],
    ['ComplianceAgent', 'Regulatory compliance', 'GDPR Art.32, SOC2, ISO27001 A.12, HIPAA HITECH'],
    ['SecretAgent', 'Secrets & credentials', 'Hardcoded API keys, exposed JWT tokens, plaintext passwords'],
    ['SupplyChainAgent', 'Supply chain security', 'Typosquatting, abandoned deps, third-party CVEs, licenses'],
  ],
  [110, 110, 280]
);
doc.moveDown(0.5);
addBody('The AgentOrchestrator manages parallel execution via Tokio async runtime, collecting findings from all agents and aggregating them into a unified report with calculated risk scores.');
doc.addPage();

// ==================== 8. SUBCONSCIOUS ENGINE ====================
addHeader('8. Subconscious Engine Deep Dive');
addSubHeader('8.1 Concept & Philosophy');
addBody('The Subconscious Engine is the most innovative feature of GatKeeper and the one that radically distinguishes it from everything else on the market. Inspired by the human brain which continues to process information during sleep — consolidating learning, solving complex problems, establishing non-obvious connections — the Subconscious Engine operates in the background 24/7.');

addSubHeader('8.2 Three Layers of the Subconscious');
addBullet('ROUGE (Red) — Threat Simulation: Simulates real attackers operating against your system. Models different adversary profiles and calculates most probable attack paths.');
addBullet('JAUNE (Yellow) — Chaos Engineering: Simulates adverse conditions to identify hidden breaking points. Inspired by Netflix Chaos Engineering and Google SRE practices.');
addBullet('VERTE (Green) — Code Evolution: Generates and evaluates improved alternatives to existing code. Acts as an AI pair-programmer working while the team sleeps.');

addSubHeader('8.3 Simulation Cycles');
addTable(
  ['Phase', 'Frequency', 'Duration', 'Content'],
  [
    ['Light Scan', 'Every hour', '~2 min', 'New commits, critical alerts, secret check'],
    ['Nightly Simulation', 'Every night', '~45 min', '7-day projection, DNA update, KB correlation'],
    ['Deep Simulation', 'Weekly', '~6 hours', '30-day projection, full architecture re-evaluation'],
    ['Grand Audit', 'Quarterly', '~24 hours', 'Exhaustive simulation, compliance report, strategic recs'],
  ],
  [110, 100, 80, 210]
);
doc.addPage();

// ==================== 9. COMPETITIVE ANALYSIS ====================
addHeader('9. Competitive Analysis');
addTable(
  ['Feature', 'SonarQube', 'Snyk', 'Veracode', 'Semgrep', 'GatKeeper'],
  [
    ['Static analysis (SAST)', '✓', 'Partial', '✓', '✓', '✓'],
    ['Dependency analysis (SCA)', 'Partial', '✓', '✓', 'Partial', '✓'],
    ['Secret scanning', 'Partial', '✓', 'Partial', '✓', '✓'],
    ['Multi-agent AI', '✗', '✗', '✗', '✗', '✓'],
    ['Evolving memory (DNA)', '✗', '✗', '✗', '✗', '✓'],
    ['Subconscious Engine 24/7', '✗', '✗', '✗', '✗', '✓'],
    ['30-day simulation', '✗', '✗', '✗', '✗', '✓'],
    ['Attack sandbox (PoC)', '✗', '✗', 'Partial', '✗', '✓'],
    ['What-If mode', '✗', '✗', '✗', '✗', '✓'],
    ['Auto compliance', 'Partial', 'Partial', '✓', '✗', '✓'],
    ['Collective KB', '✗', '✗', '✗', '✗', '✓'],
    ['Cross-repository', '✗', 'Partial', '✗', '✗', '✓'],
    ['Post-quantum prep', '✗', '✗', '✗', '✗', '✓'],
    ['100% on-premise', '✓', '✗', '✗', '✓', '✓'],
    ['Versioned report', 'Partial', '✗', '✗', '✗', '✓'],
  ],
  [110, 70, 60, 70, 70, 90]
);
doc.addPage();

// ==================== 10. .GAT FORMAT ====================
addHeader('10. .gat Report Format Specification');
addBody('The .gat file is far more than a simple output report. It is a structured, versionable, and machine-exploitable artifact that documents the complete health state of a project at a given moment. Designed to be attached to Git commits, integrated into CI/CD pipelines, and consumed by third-party tools via API.');

doc.fontSize(9).font('Courier').fillColor(DARK);
const gatExample = `
[session]
organisation = "TechCorp SA"
project = "payment-service"
version = "2.4.1"
timestamp = "2026-06-03T08:00:00Z"
agents_utilises = ["SecurityAgent", "LogicAgent", "ComplianceAgent"]
lignes_analysees = 847_293
scenarios_sub = 48_293
duree_analyse = "6h32m (background)"

[risk_score]
global = 73/100
securite = 61/100
performance = 88/100
compliance = 79/100
tendance = "down (-7pts sur 14 jours)"

[[findings.critical]]
id = "GAT-CRIT-001"
agent = "SecurityAgent"
fichier = "src/auth/token.rs"
lignes = [234, 267]
type = "timing_attack"
cvss_score = 8.7
description = "Non-constant-time token comparison, exploitable via timing"
correction = "Use subtle::ConstantTimeEq"
effort_correction = "15 minutes"

[[subconscious.dreams]]
id = "DREAM-001"
couche = "threat"
decouverte_a = "03:47 AM"
horizon = "23 jours"
probabilite = 94
impact = "unauthorized admin access"
condition = "traffic > 10k req/s"

[subconscious.intuitions]
items = [
  "auth/ and payments/ diverging silently. Incompatible in 2 sprints.",
  "crypto-lib abandoned. CVE probability 60d: 71%."
]`.trim();

doc.text(gatExample, 50, doc.y, { width: 500 });
doc.moveDown(1);
doc.addPage();

// ==================== 11. ROADMAP ====================
addHeader('11. Phase 1 Roadmap (3 Months)');
addSubHeader('Month 1 — Foundation');
addBullet('Set up Cargo workspace with all crate structure');
addBullet('Implement CLI with clap (init, scan, report, status commands)');
addBullet('Integrate Tree-sitter parser for 5 languages (Rust, Python, JS, C, Go)');
addBullet('Implement SecurityAgent basic patterns (injection, crypto, OWASP)');
addBullet('Implement SecretAgent with hardcoded credential detection');
addBullet('Generate .gat report in TOML format');
addBullet('SQLite schema for DNA Engine storage');

addSubHeader('Month 2 — Intelligence');
addBullet('Complete all 6 agents with pattern-based analysis');
addBullet('Implement AgentOrchestrator with Tokio parallel execution');
addBullet('Basic Subconscious Engine with hourly cycle and 24h projection');
addBullet('DNA Engine v1: fingerprinting, baseline, pattern detection');
addBullet('SupplyChainAgent with OSV/NVD API integration for CVE checking');
addBullet('GitHub Actions CI/CD integration');

addSubHeader('Month 3 — Polish & Launch');
addBullet('Plugin Neovim basic integration');
addBullet('Comprehensive documentation and getting-started guide');
addBullet('Integration test suite with vulnerable project fixtures');
addBullet('Performance optimization and error handling hardening');
addBullet('README, contributing guide, and community setup');
addBullet('Public GitHub repository launch');
doc.addPage();

// ==================== 12. MONETIZATION ====================
addHeader('12. Monetization & Go-to-Market');
addSubHeader('Pricing Plans');
addTable(
  ['Plan', 'Target', 'Price', 'Key Features'],
  [
    ['Community', 'Individual devs, OSS', 'Free', '1 project, CLI, basic SecurityAgent, .gat, 24h subconsc.'],
    ['Pro', 'Freelancers, startups', '€49/mo', 'Unlimited projects, 6 agents, dashboard, CI/CD, 7-day sub.'],
    ['Business', 'Teams 5-50', '€299/mo/team', 'Compliance, sandbox PoC, Horizon, KB, 30-day subconsc.'],
    ['Enterprise', 'Large orgs 50+', '$50K-500K/yr', 'On-premise, fine-tuning, Telepathy, Quantum, SLA, support'],
  ],
  [85, 110, 85, 220]
);

addSubHeader('Go-to-Market Strategy');
addBullet('Phase 1: Community open source → organic adoption by developers, GitHub contributions');
addBullet('Phase 2: Product-led growth → natural Pro upgrade via feature gates');
addBullet('Phase 3: Sales-led growth → commercial team targeting CISOs of large enterprises');
addBullet('Strategic partners: integrators (Accenture, Deloitte), cloud providers (AWS, Azure, GCP)');
addBullet('Conferences: DEF CON, Black Hat, RSA Conference, KubeCon for security credibility');

// ==================== FINAL ====================
doc.addPage();
doc.rect(0, 0, 595, 842).fill(BLUE);
doc.fontSize(32).font('Helvetica-Bold').fillColor(WHITE).text('GatKeeper', 60, 300, { width: 480, align: 'center' });
doc.moveDown(1);
doc.fontSize(16).font('Helvetica').fillColor(LIGHT_BLUE).text('The first security tool that thinks.', 60, 360, { align: 'center' });
doc.moveDown(2);
doc.fontSize(12).fillColor(GRAY).text('Document generated by GatKeeper Project Analysis', 60, 420, { align: 'center' });
doc.text('Version 1.0 — June 2026', 60, 440, { align: 'center' });
doc.moveDown(3);
doc.fontSize(10).fillColor(GRAY).text('"While you slept, I found 3 things you need to know."', 60, 500, { align: 'center' });

doc.end();

stream.on('finish', () => {
  console.log('✅ PDF EN generated: docs/GatKeeper_Analysis_EN.pdf');
});
