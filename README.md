<div align="center">

# 🛡️ GatKeeper

### AI-Native Security Intelligence Platform with a Subconscious Engine

**Not a linter. Not a scanner. A colleague who never stops thinking.**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org/)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)
[![Phase](https://img.shields.io/badge/Phase-1%20MVP-green)]()
[![CI](https://img.shields.io/badge/CI-GitHub%20Actions-yellow?logo=github-actions)](.github/workflows/ci.yml)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Issues](https://img.shields.io/github/issues/karlalbert152-sys/gatkeeper)](https://github.com/karlalbert152-sys/gatkeeper/issues)
[![Stars](https://img.shields.io/github/stars/karlalbert152-sys/gatkeeper)](https://github.com/karlalbert152-sys/gatkeeper/stargazers)
[![Discord](https://img.shields.io/discord/1234567890?label=Discord&logo=discord)](https://discord.gg/gatkeeper)

---

[📄 Analysis (EN)](docs/GatKeeper_Analysis_EN.pdf) •
[🇫🇷 Français](README_FR.md) •
[🤝 Contributing](CONTRIBUTING.md) •
[💬 Discord](https://discord.gg/gatkeeper)

</div>

---

## 🧠 What is GatKeeper?

GatKeeper is a **next-generation code intelligence platform** that doesn't just scan your code — it **thinks about it, simulates its future, and predicts vulnerabilities before they exist**.

Unlike traditional security tools (SonarQube, Snyk, Semgrep) that are **reactive**, GatKeeper is **predictive and proactive**:

| Dimension | Classic SAST | AI Assistants | **GatKeeper** |
|-----------|-------------|---------------|---------------|
| Approach | Reactive | Assistive | **Predictive & proactive** |
| Understanding | Syntactic | Partial contextual | **Semantic + intentional** |
| Project memory | ✗ | ✗ | **✓** |
| Future simulation | ✗ | ✗ | **✓** |
| 100% on-premise | Partial | ✗ | **✓** |
| Multi-agents | ✗ | ✗ | **✓** |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  LAYER 1: INTERFACE                          │
│  CLI (Rust) │ Neovim Plugin │ VSCode Extension │ Dashboard  │
├─────────────────────────────────────────────────────────────┤
│                  LAYER 2: INTEGRATION                        │
│  Git Hooks │ GitHub Actions │ GitLab CI │ Jenkins │ API      │
├─────────────────────────────────────────────────────────────┤
│                  LAYER 3: INTELLIGENCE                       │
│  ┌─────────────────┐  ┌──────────────────────────────────┐  │
│  │ Subconscious Eng │  │ 6 AI Agents (Security, Logic,   │  │
│  │ 24/7 simulation  │  │ Performance, Compliance, Secret,│  │
│  │ Rouge/Jaune/Vert │  │ SupplyChain)                     │  │
│  └─────────────────┘  └──────────────────────────────────┘  │
│  ┌─────────────────┐  ┌──────────────────────────────────┐  │
│  │ DNA Engine      │  │ Tree-sitter Parser (15+ langs)   │  │
│  │ Git memory      │  │ C, C++, Rust, Python, JS, Go...  │  │
│  └─────────────────┘  └──────────────────────────────────┘  │
│  ┌─────────────────┐  ┌──────────────────────────────────┐  │
│  │ Ollama (local)  │  │ Sandbox Attack (gVisor + Docker) │  │
│  └─────────────────┘  └──────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                  LAYER 4: DATA                               │
│  SQLite (local) │ PostgreSQL (Enterprise) │ CVE DB (OSV)    │
└─────────────────────────────────────────────────────────────┘
```

---

## ✨ Key Features

### 🧠 Subconscious Engine
The core differentiator. Runs 24/7 in the background:
- **Rouge (Red)** — Threat Simulation: Models real attackers and calculates attack paths
- **Jaune (Yellow)** — Chaos Engineering: Injects failures to find hidden breaking points
- **Verte (Green)** — Code Evolution: Generates safer, faster code alternatives

### 🤖 6 Specialized AI Agents
| Agent | Domain |
|-------|--------|
| **SecurityAgent** | SQL/XSS/LDAP injection, timing attacks, weak crypto, OWASP Top 10 |
| **LogicAgent** | Race conditions, deadlocks, edge cases |
| **PerformanceAgent** | Memory leaks, N+1 queries, unnecessary allocations |
| **ComplianceAgent** | GDPR, SOC2, ISO27001, HIPAA |
| **SecretAgent** | Hardcoded API keys, JWT tokens, passwords |
| **SupplyChainAgent** | Typosquatting, abandoned deps, CVEs, license conflicts |

### 🧬 DNA Engine
Builds a living memory of your codebase:
- Cryptographic fingerprinting
- Architectural pattern detection
- Code invariant extraction
- Git behavioral baseline

### 📄 .gat Report Format
A structured, versionable, machine-readable report attached to every scan.

---

## 🚀 Quick Start

### Prerequisites
- **Rust** 1.75+ ([install](https://rustup.rs/))
- **Git**

### Install

```bash
git clone https://github.com/karlalbert152-sys/gatkeeper.git
cd gatkeeper
cargo build --release
```

### Usage

```bash
# Initialize GatKeeper in your project
gatkeeper init

# Run a full security scan
gatkeeper scan

# Generate a report
gatkeeper report

# Check project status
gatkeeper status
```

---

## 📁 Project Structure

```
gatkeeper/
├── crates/
│   ├── gatkeeper-cli/           # CLI binary
│   ├── gatkeeper-core/          # Core types & config
│   ├── gatkeeper-parser/        # Tree-sitter multi-lang parser
│   ├── gatkeeper-agents/        # 6 specialized AI agents
│   ├── gatkeeper-subconscious/  # Subconscious Engine
│   └── gatkeeper-dna/           # DNA Engine (memory)
├── gatkeeper-lenses/            # Intent comprehension
├── migrations/                  # SQLite schema
├── templates/                   # .gat template, config
├── tests/                       # Integration tests
├── scripts/                     # PDF generation
└── docs/                        # Documentation + PDFs
```

---

## 🛠️ Tech Stack

| Component | Technology | Why |
|-----------|-----------|-----|
| CLI | **Rust** + Clap | Native performance, memory safety |
| Parsing | **Tree-sitter** | 15+ languages, incremental, embeddable |
| Async | **Tokio** | Millions of concurrent tasks, zero overhead |
| Local LLMs | **Ollama** | Zero API cost, absolute privacy |
| Storage | **SQLite** | Lightweight, portable, no server |
| Dashboard | **React** + TypeScript | Rich real-time UI |
| API | **Axum** (Rust) | Consistent with CLI, excellent performance |

---

## 📊 Key Metrics

| Metric | Value |
|--------|-------|
| Scenarios simulated / night | **48,000+** |
| Specialized AI agents | **6** |
| Temporal projection | **30 days** |
| Supported languages | **15+** |
| Addressable market | **$25B** |
| Client savings vs. audit | **80%** |

---

## 📄 Documentation

- [Project Analysis (EN)](docs/GatKeeper_Analysis_EN.pdf) — Full 14-page analysis document
- [Analyse du Projet (FR)](docs/GatKeeper_Analysis_FR.md) — Version française

---

## 🗺️ Roadmap

- [x] **Phase 1 — MVP** (Months 1-3): CLI, Tree-sitter, 6 agents, .gat format, basic Subconscious
- [ ] **Phase 2 — Alpha** (Months 4-6): Dashboard web, DNA Engine v1, CI/CD integration
- [ ] **Phase 3 — Beta** (Months 7-12): Sandbox, Compliance, What-If, public launch
- [ ] **Phase 4 — Enterprise** (Year 2): On-premise, fine-tuning, Telepathy, Quantum

---

## 🤝 Contributing

We love contributions! GatKeeper is an open-source project and we welcome developers of all skill levels.

**Ways to contribute:**
- 🐛 [Report bugs](https://github.com/karlalbert152-sys/gatkeeper/issues/new?template=bug_report.md)
- 💡 [Request features](https://github.com/karlalbert152-sys/gatkeeper/issues/new?template=feature_request.md)
- 📝 [Improve docs](https://github.com/karlalbert152-sys/gatkeeper/issues/new?template=documentation.md)
- 💻 Submit a Pull Request
- ⭐ Star the repo to show support

**Good first issues:** Look for [`good first issue`](https://github.com/karlalbert152-sys/gatkeeper/labels/good%20first%20issue) labels.

👉 **Read [CONTRIBUTING.md](CONTRIBUTING.md)** for setup instructions and guidelines.

---

## 📜 License

This project is licensed under the **GNU Affero General Public License v3.0** — see the [LICENSE](LICENSE) file for details.

This means:
- ✅ You can use, modify, and distribute this software
- ✅ You can use it commercially
- ⚠️ If you modify and distribute it, you **must** release your changes under AGPLv3
- ⚠️ If you use it as a network service (SaaS), you **must** provide the source code to users

---

<div align="center">

**"While you slept, I found 3 things you need to know."**

*GatKeeper — The first security tool that thinks.*

</div>
