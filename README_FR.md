<div align="center">

# 🛡️ GatKeeper

### Plateforme d'Intelligence de Sécurité AI-Native avec un Subconscious Engine

**Pas un linter. Pas un scanner. Un collègue qui ne cesse jamais de penser.**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue)](LICENSE)
[![Phase](https://img.shields.io/badge/Phase-1%20MVP-green)]()

---

[📄 Document d'Analyse (FR)](docs/GatKeeper_Analysis_FR.pdf) •
[🇬🇧 English](README.md)

</div>

---

## 🧠 Qu'est-ce que GatKeeper ?

GatKeeper est une **plateforme d'intelligence de code de nouvelle génération** qui ne se contente pas de scanner votre code — elle **y réfléchit, simule son avenir et prédit les vulnérabilités avant qu'elles n'existent**.

Contrairement aux outils de sécurité classiques (SonarQube, Snyk, Semgrep) qui sont **réactifs**, GatKeeper est **prédictif et proactif** :

| Dimension | SAST classique | Assistants IA | **GatKeeper** |
|-----------|---------------|---------------|---------------|
| Approche | Réactive | Assistive | **Prédictive & proactive** |
| Compréhension | Syntaxique | Contextuelle partielle | **Sémantique + intentionnelle** |
| Mémoire projet | ✗ | ✗ | **✓** |
| Simulation futur | ✗ | ✗ | **✓** |
| 100% on-premise | Partiel | ✗ | **✓** |
| Multi-agents | ✗ | ✗ | **✓** |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              COUCHE 1 : INTERFACE                            │
│  CLI (Rust) │ Plugin Neovim │ Extension VSCode │ Dashboard   │
├─────────────────────────────────────────────────────────────┤
│              COUCHE 2 : INTÉGRATION                          │
│  Git Hooks │ GitHub Actions │ GitLab CI │ Jenkins │ API      │
├─────────────────────────────────────────────────────────────┤
│              COUCHE 3 : INTELLIGENCE                         │
│  ┌─────────────────┐  ┌──────────────────────────────────┐  │
│  │ Subconscious Eng │  │ 6 Agents IA (Security, Logic,   │  │
│  │ simulation 24/7  │  │ Performance, Compliance, Secret,│  │
│  │ Rouge/Jaune/Vert │  │ SupplyChain)                     │  │
│  └─────────────────┘  └──────────────────────────────────┘  │
│  ┌─────────────────┐  ┌──────────────────────────────────┐  │
│  │ DNA Engine      │  │ Tree-sitter Parser (15+ lang.)   │  │
│  │ mémoire Git     │  │ C, C++, Rust, Python, JS, Go...  │  │
│  └─────────────────┘  └──────────────────────────────────┘  │
│  ┌─────────────────┐  ┌──────────────────────────────────┐  │
│  │ Ollama (local)  │  │ Sandbox Attaque (gVisor+Docker)  │  │
│  └─────────────────┘  └──────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│              COUCHE 4 : DONNÉES                              │
│  SQLite (local) │ PostgreSQL (Entreprise) │ CVE DB (OSV)    │
└─────────────────────────────────────────────────────────────┘
```

---

## ✨ Fonctionnalités Clés

### 🧠 Subconscious Engine
Le différenciateur clé. Tourne 24/7 en arrière-plan :
- **Rouge** — Simulation de menaces : Modélise des attaquants réels et calcule les chemins d'attaque
- **Jaune** — Chaos Engineering : Injecte des pannes pour trouver les points de rupture cachés
- **Verte** — Évolution du code : Génère des alternatives plus sûres et plus rapides

### 🤖 6 Agents IA Spécialisés
| Agent | Domaine |
|-------|---------|
| **SecurityAgent** | Injection SQL/XSS/LDAP, timing attacks, crypto faible, OWASP Top 10 |
| **LogicAgent** | Race conditions, deadlocks, edge cases |
| **PerformanceAgent** | Memory leaks, N+1 queries, allocations inutiles |
| **ComplianceAgent** | GDPR, SOC2, ISO27001, HIPAA |
| **SecretAgent** | API keys hardcodées, tokens JWT, mots de passe |
| **SupplyChainAgent** | Typosquatting, dépendances orphelines, CVE, licences |

### 🧬 DNA Engine
Construit une mémoire vivante de votre codebase :
- Empreinte cryptographique
- Détection de patterns architecturaux
- Extraction d'invariants de code
- Baseline comportementale Git

### 📄 Format .gat
Un rapport structuré, versionnable et lisible par machine, attaché à chaque scan.

---

## 🚀 Démarrage Rapide

### Prérequis
- **Rust** 1.75+ ([installer](https://rustup.rs/))
- **Git**

### Installation

```bash
git clone https://github.com/karlalbert152-sys/gatkeeper.git
cd gatkeeper
cargo build --release
```

### Utilisation

```bash
# Initialiser GatKeeper dans votre projet
gatkeeper init

# Lancer un scan de sécurité complet
gatkeeper scan

# Générer un rapport
gatkeeper report

# Vérifier l'état du projet
gatkeeper status
```

---

## 📁 Structure du Projet

```
gatkeeper/
├── crates/
│   ├── gatkeeper-cli/           # Binaire CLI
│   ├── gatkeeper-core/          # Types & config core
│   ├── gatkeeper-parser/        # Parser multi-langage Tree-sitter
│   ├── gatkeeper-agents/        # 6 agents IA spécialisés
│   ├── gatkeeper-subconscious/  # Subconscious Engine
│   └── gatkeeper-dna/           # DNA Engine (mémoire)
├── gatkeeper-lenses/            # Compréhension de l'intention
├── migrations/                  # Schéma SQLite
├── templates/                   # Template .gat, config
├── tests/                       # Tests d'intégration
├── scripts/                     # Génération PDF
└── docs/                        # Documentation + PDFs
```

---

## 🛠️ Stack Technique

| Composant | Technologie | Pourquoi |
|-----------|------------|----------|
| CLI | **Rust** + Clap | Performances natives, sécurité mémoire |
| Parsing | **Tree-sitter** | 15+ langages, incrémental, embarquable |
| Async | **Tokio** | Millions de tâches concurrentes, zéro overhead |
| LLMs locaux | **Ollama** | Zéro coût API, confidentialité absolue |
| Stockage | **SQLite** | Léger, portable, pas de serveur |
| Dashboard | **React** + TypeScript | Interface riche temps réel |
| API | **Axum** (Rust) | Cohérence avec CLI, performances excellentes |

---

## 📊 Chiffres Clés

| Indicateur | Valeur |
|------------|--------|
| Scénarios simulés / nuit | **48 000+** |
| Agents IA spécialisés | **6** |
| Projection temporelle | **30 jours** |
| Langages supportés | **15+** |
| Taille de marché adressable | **25 Mds $** |
| Économie client vs audit | **80%** |

---

## 📄 Documentation

- [Analyse du Projet (FR)](docs/GatKeeper_Analysis_FR.pdf) — Document d'analyse complet de 14 pages
- [Project Analysis (EN)](docs/GatKeeper_Analysis_EN.pdf) — Version anglaise

---

## 🗺️ Roadmap

- [x] **Phase 1 — MVP** (Mois 1-3) : CLI, Tree-sitter, 6 agents, format .gat, Subconscious basique
- [ ] **Phase 2 — Alpha** (Mois 4-6) : Dashboard web, DNA Engine v1, intégration CI/CD
- [ ] **Phase 3 — Beta** (Mois 7-12) : Sandbox, Compliance, What-If, lancement public
- [ ] **Phase 4 — Enterprise** (Année 2) : On-premise, fine-tuning, Telepathy, Quantum

---

## 🤝 Contribuer

Les contributions sont les bienvenues ! Veuillez lire [CONTRIBUTING.md](CONTRIBUTING.md) avant de soumettre des PRs.

---

## 📜 Licence

Sous licence :
- [MIT License](LICENSE)
- [Apache License, Version 2.0](LICENSE)

à votre choix.

---

<div align="center">

**"Pendant que tu dormais, j'ai trouvé 3 choses que tu dois savoir."**

*GatKeeper — Le premier outil de sécurité qui pense en permanence.*

</div>
