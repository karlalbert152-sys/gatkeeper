import PDFDocument from 'pdfkit';
import fs from 'fs';

const BLUE = '#1a365d';
const LIGHT_BLUE = '#2b6cb0';
const GRAY = '#4a5568';
const LIGHT_GRAY = '#e2e8f0';
const DARK = '#1a202c';
const WHITE = '#ffffff';

const doc = new PDFDocument({
  size: 'A4',
  margins: { top: 60, bottom: 60, left: 60, right: 60 },
  info: {
    Title: 'GatKeeper — Plateforme d\'Intelligence de Sécurité AI-Native',
    Author: 'Karla Albert',
    Subject: 'Document d\'Analyse & Architecture du Projet',
    CreationDate: new Date(),
  }
});

const stream = fs.createWriteStream('./docs/GatKeeper_Analysis_FR.pdf');
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
  doc.font('Helvetica-Bold').fontSize(9).fillColor(WHITE);
  let x = startX;
  headers.forEach((h, i) => {
    doc.rect(x, y, colWidths[i], 22).fill(BLUE);
    doc.fillColor(WHITE).text(h, x + 5, y + 6, { width: colWidths[i] - 10 });
    x += colWidths[i];
  });
  y += 22;
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

// ==================== PAGE DE COUVERTURE ====================
doc.rect(0, 0, 595, 842).fill(BLUE);
doc.fontSize(42).font('Helvetica-Bold').fillColor(WHITE).text('GatKeeper', 60, 200, { width: 480 });
doc.fontSize(20).font('Helvetica').fillColor(LIGHT_BLUE).text('Plateforme d\'Intelligence de Sécurité AI-Native', 60, 260, { width: 480 });
doc.moveDown(2);
doc.fontSize(14).fillColor(GRAY).font('Helvetica').text('Document d\'Analyse & Architecture du Projet', 60, 340);
doc.moveDown(1);
doc.fontSize(12).fillColor(WHITE).text('Version 1.0  |  Juin 2026', 60, 380);
doc.moveDown(0.5);
doc.text('Auteur : Karla Albert <karlalbert152@gmail.com>', 60, 400);
doc.moveDown(0.5);
doc.text('Statut : Phase 1 — Architecture MVP', 60, 420);
doc.moveDown(4);
doc.fontSize(10).fillColor(GRAY).text('CONFIDENTIEL', 60, 580);
doc.addPage();

// ==================== TABLE DES MATIÈRES ====================
addHeader('Table des Matières');
const toc = [
  ['1', 'Résumé Exécutif', '3'],
  ['2', 'Problématique & Analyse du Marché', '4'],
  ['3', 'Solution & Positionnement Stratégique', '5'],
  ['4', 'Architecture — Design 4 Couches', '6'],
  ['5', 'Stack Technique & Justifications', '7'],
  ['6', 'Phase 1 MVP — Structure des Dossiers', '8'],
  ['7', 'Système d\'Analyse Multi-Agents', '9'],
  ['8', 'Plongée dans le Subconscious Engine', '10'],
  ['9', 'Analyse Concurrentielle', '11'],
  ['10', 'Spécification du Format .gat', '12'],
  ['11', 'Roadmap Phase 1 (3 Mois)', '13'],
  ['12', 'Monétisation & Stratégie Go-to-Market', '14'],
];

toc.forEach(([num, title, page]) => {
  doc.fontSize(11).font('Helvetica').fillColor(DARK)
    .text(`${num}.  ${title}`, 80, doc.y, { continued: true, width: 380 })
    .fillColor(GRAY).text(`  ${page}`, { align: 'right' });
  doc.moveDown(0.2);
});
doc.addPage();

// ==================== 1. RÉSUMÉ EXÉCUTIF ====================
addHeader('1. Résumé Exécutif');
addBody('GatKeeper est une plateforme d\'intelligence de code de nouvelle génération conçue pour répondre à l\'une des problématiques les plus critiques de notre ère numérique : la sécurité des logiciels à grande échelle. Contrairement aux outils existants qui se contentent de détecter des problèmes déjà présents, GatKeeper adopte une approche radicalement différente : anticiper, simuler et prédire les failles avant qu\'elles n\'existent.');
addBody('La plateforme combine une architecture multi-agents basée sur des modèles d\'IA spécialisés, un moteur de simulation autonome (le Subconscious Engine), et une mémoire évolutive du codebase pour offrir une couverture de sécurité sans précédent. GatKeeper s\'adresse aussi bien aux développeurs individuels qu\'aux grandes organisations (Fortune 500, institutions financières, gouvernements) ayant des exigences de conformité strictes (GDPR, SOC2, ISO27001, HIPAA).');

addSubHeader('Chiffres Clés');
addTable(
  ['Indicateur', 'Valeur', 'Contexte'],
  [
    ['Scénarios simulés / nuit', '48 000+', 'Par le Subconscious Engine'],
    ['Agents IA spécialisés', '6 agents', 'Chacun fine-tuné sur son domaine'],
    ['Projection temporelle', '30 jours', 'Simulation prospective du code'],
    ['Langages supportés', '15+', 'C, C++, Rust, Python, JS, Go...'],
    ['Taille de marché adressable', '25 Mds USD', 'Sécurité logicielle mondiale 2025'],
    ['Économie client estimée', '80%', 'des coûts vs audit externe classique'],
  ],
  [160, 100, 240]
);
doc.addPage();

// ==================== 2. PROBLÉMATIQUE ====================
addHeader('2. Problématique & Analyse du Marché');
addSubHeader('2.1 La Crise Silencieuse de la Sécurité Logicielle');
addBody('Nous vivons une époque paradoxale : jamais les outils de développement n\'ont été aussi puissants, et pourtant les failles de sécurité logicielle atteignent des niveaux records. En 2024, le coût moyen d\'une violation de données dépassait 4,5 millions de dollars par incident. Les grandes organisations font face à plusieurs problèmes structurels simultanés :');
addBullet('Codebases massifs (millions de lignes) impossibles à auditer manuellement');
addBullet('Dizaines d\'équipes développant en parallèle créant des interactions imprévues');
addBullet('Dépendances externes (npm, crates.io, PyPI) introduisant des vulnérabilités tierces');
addBullet('Audits de sécurité externes coûteux (100K à 500K par audit) et peu fréquents');
addBullet('Outils fragmentés qui ne se parlentnt : SAST, DAST, SCA, scanners de secrets');
addBullet('Conformité réglementaire (GDPR, SOC2, HIPAA) de plus en plus exigeante');

addSubHeader('2.2 Les Limites des Outils Actuels');
addBody('Les solutions existantes souffrent d\'un défaut fondamental : elles sont réactives. Elles trouvent ce qui est déjà cassé, jamais ce qui va l\'être :');
addBullet('SonarQube et outils SAST : analyse statique sans compréhension sémantique');
addBullet('Snyk et outils SCA : focalisés sur les dépendances, ignorent la logique interne');
addBullet('Outils de pentest manuel : coûteux, ponctuels, non intégrés au cycle de dev');
addBullet('GitHub Copilot et assistants IA : suggèrent du code mais n\'auditent pas');
addBullet('Aucun outil ne combine mémoire évolutive, simulation prospective et analyse multi-couches');
doc.addPage();

// ==================== 3. SOLUTION ====================
addHeader('3. Solution & Positionnement Stratégique');
addSubHeader('3.1 La Vision Fondatrice');
addBody('GatKeeper est la première plateforme d\'intelligence de code vivante — un système qui apprend l\'ADN de votre codebase, simule des milliers de futurs possibles en arrière-plan, et alerte proactivement avant que les problèmes n\'arrivent. Comme un médecin qui prédit la maladie avant les symptômes plutôt que de la traiter une fois déclarée.');
addBody('GatKeeper unifie en une seule plateforme ce qui nécessite aujourd\'hui 5 à 8 outils distincts : analyse statique, analyse de dépendances, scan de secrets, modélisation des menaces, compliance automatique, simulation de charge et simulation d\'attaque.');

addSubHeader('3.2 Positionnement Marché');
addTable(
  ['Dimension', 'SAST classique', 'Assistants IA', 'GatKeeper'],
  [
    ['Approche', 'Réactive', 'Assistive', 'Prédictive & proactive'],
    ['Compréhension', 'Syntaxique', 'Contextuelle partielle', 'Sémantique + intentionnelle'],
    ['Mémoire projet', '✗', '✗', '✓'],
    ['Simulation futur', '✗', '✗', '✓'],
    ['100% on-premise', 'Partiel', '✗', '✓'],
    ['Multi-agents', '✗', '✗', '✓'],
    ['Compliance auto', 'Partiel', '✗', '✓'],
  ],
  [120, 120, 120, 140]
);
doc.addPage();

// ==================== 4. ARCHITECTURE ====================
addHeader('4. Architecture — Design 4 Couches');
addBody('La plateforme s\'articule autour de quatre couches principales, chacune avec des responsabilités clairement définies et des composants intégrés de la stack technique.');

doc.fontSize(9).font('Courier').fillColor(DARK);
const archDiagram = `
┌─────────────────────────────────────────────────────────────┐
│              COUCHE 1 : INTERFACE                            │
│  CLI (Rust/Clap)  │  Plugin Neovim  │  Extension VSCode     │
│  Dashboard Web (React + TypeScript + WebSockets)             │
├─────────────────────────────────────────────────────────────┤
│              COUCHE 2 : INTÉGRATION                          │
│  Git Hooks │ GitHub Actions │ GitLab CI │ Jenkins │ API      │
├─────────────────────────────────────────────────────────────┤
│              COUCHE 3 : INTELLIGENCE                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Tokio Async Runtime                                  │  │
│  │  ┌─────────────────┐  ┌────────────────────────────┐  │  │
│  │  │ Subconscious Eng│  │ 6 Agents IA (Security,     │  │  │
│  │  │ Rouge/Jaune/Vert│  │ Logic, Perf, Comply,       │  │  │
│  │  │ simulation 24/7 │  │ Secret, SupplyChain)       │  │  │
│  │  └─────────────────┘  └────────────────────────────┘  │  │
│  │  ┌─────────────────┐  ┌────────────────────────────┐  │  │
│  │  │ DNA Engine      │  │ Tree-sitter Parser (15+)   │  │  │
│  │  │ (mémoire Git)   │  │ C,C++,Rust,Python,JS,Go    │  │  │
│  │  └─────────────────┘  └────────────────────────────┘  │  │
│  │  ┌─────────────────┐  ┌────────────────────────────┐  │  │
│  │  │ Ollama (local)  │  │ Sandbox Attaque (gVisor)   │  │  │
│  │  │ codellama       │  │ PoC gen, fuzzing           │  │  │
│  │  └─────────────────┘  └────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│              COUCHE 4 : DONNÉES                              │
│  SQLite (local) │ PostgreSQL (Ent.) │ CVE DB (OSV + NVD)     │
└─────────────────────────────────────────────────────────────┘`.trim();

doc.text(archDiagram, 60, doc.y, { width: 480 });
doc.moveDown(1);

addSubHeader('Responsabilités des Composants');
addBullet('INTERFACE : Point d\'entrée pour les développeurs. CLI pour les workflows terminal, plugins IDE pour le feedback temps réel, Dashboard pour la visibilité d\'équipe.');
addBullet('INTÉGRATION : Automatise les vérifications de sécurité dans le cycle de développement. Chaque commit déclenche une analyse.');
addBullet('INTELLIGENCE : Cerveau principal. Les agents analysent, le Subconscious simule, le DNA se souvient, Tree-sitter parse.');
addBullet('DONNÉES : Stockage persistant. SQLite pour local/portable, PostgreSQL pour l\'échelle entreprise, bases CVE pour le renseignement sur les menaces.');
doc.addPage();

// ==================== 5. STACK TECHNIQUE ====================
addHeader('5. Stack Technique & Justifications');
addTable(
  ['Composant', 'Technologie', 'Version', 'Justification'],
  [
    ['CLI principal', 'Rust', '1.75+', 'Performances natives, sécurité mémoire garantie'],
    ['Parsing du code', 'Tree-sitter', '0.22+', 'Multi-langage (15+), robuste, incrémental, embarquable'],
    ['Orchestration agents', 'Tokio (async Rust)', '1.35+', 'Millions de tâches concurrentes sans overhead'],
    ['LLMs d\'analyse', 'Ollama (local)', 'Latest', 'Zéro coût API pour le subconscient nocturne'],
    ['LLMs cloud (Pro/Ent.)', 'Anthropic Claude', 'claude-3', 'Qualité d\'analyse supérieure pour les tiers payants'],
    ['Dashboard web', 'React + TypeScript', '18+/5+', 'Interface riche, temps réel via WebSockets'],
    ['Backend API', 'Axum (Rust)', '0.7+', 'Cohérence avec le CLI, performances HTTP excellentes'],
    ['Stockage local', 'SQLite (rusqlite)', '3.45+', 'Léger, portable, versionnable, pas de serveur requis'],
    ['Stockage enterprise', 'PostgreSQL', '16+', 'Scalabilité, transactions, full-text search'],
    ['Analyse CVE', 'OSV + NIST NVD API', 'Latest', 'Bases officielles, open source, MAJ temps réel'],
    ['Sandbox attaques', 'gVisor + Docker', 'Latest', 'Isolation kernel-level pour exécution sécurisée PoC'],
    ['Communication temps réel', 'WebSockets', 'Latest', 'Dashboard live, alertes push immédiates'],
    ['Format rapport', 'TOML', '1.0', 'Lisible humainement, parsable machine, versionnable Git'],
  ],
  [100, 100, 60, 240]
);
doc.addPage();

// ==================== 6. STRUCTURE PHASE 1 ====================
addHeader('6. Phase 1 MVP — Structure des Dossiers');
addBody('La structure de la Phase 1 MVP suit un modèle Cargo workspace avec des crates clairement séparées pour chaque composant majeur.');

doc.fontSize(8).font('Courier').fillColor(DARK);
const dirTree = `
gatkeeper/
├── Cargo.toml                         # Racine workspace
├── rust-toolchain.toml
├── LICENSE
├── README.md / README_FR.md
├── .gitignore
│
├── crates/
│   ├── gatkeeper-cli/                 # Binaire CLI principal
│   │   └── src/
│   │       ├── main.rs                # Entrée (clap)
│   │       ├── commands/              # scan, report, init, status, whatif
│   │       └── output.rs              # Formatage terminal
│   ├── gatkeeper-core/                # Logique métier partagée
│   │   └── src/
│   │       ├── config.rs              # Configuration projet
│   │       ├── error.rs               # Types d'erreur
│   │       ├── project.rs             # Détection projet
│   │       └── report/                # .gat, risk_score, findings
│   ├── gatkeeper-parser/              # Tree-sitter multi-langage
│   │   └── src/
│   │       ├── languages/             # Rust, Python, JS, C, Go
│   │       ├── ast.rs                 # Wrapper AST unifié
│   │       └── walker.rs             # Patterns de parcours AST
│   ├── gatkeeper-agents/              # 6 agents IA spécialisés
│   │   └── src/
│   │       ├── orchestrator.rs        # Orchestration multi-agents
│   │       ├── traits.rs              # Définition du trait Agent
│   │       ├── security/              # SQL/XSS, crypto, OWASP
│   │       ├── secrets/               # Détecteur de secrets hardcodés
│   │       └── supply_chain/          # Vérification CVE dépendances
│   ├── gatkeeper-subconscious/        # Subconscious Engine
│   │   └── src/
│   │       ├── engine.rs              # Boucle principale (Tokio spawn)
│   │       ├── scheduler.rs           # Planification cron-like
│   │       ├── threat/                # Rouge : simulation attaquants
│   │       ├── chaos/                 # Jaune : injection pannes
│   │       └── evolution/             # Verte : génération code
│   └── gatkeeper-dna/                 # DNA Engine (mémoire long terme)
│       └── src/
│           ├── fingerprint.rs         # Empreinte cryptographique
│           ├── patterns.rs           # Détection patterns archi
│           ├── invariants.rs         # Invariants de code
│           ├── baseline.rs           # Baseline comportementale Git
│           └── store.rs             # Persistance SQLite
│
├── gatkeeper-lenses/                  # Compréhension de l'intention
├── migrations/                        # Schéma SQLite
├── templates/                         # Template .gat, exemple config
├── tests/                             # Tests d'intégration + fixtures
├── scripts/                           # Génération PDF, helpers CI
├── docs/                              # Documentation + PDFs
└── .github/workflows/                # Pipeline CI/CD`.trim();

doc.text(dirTree, 50, doc.y, { width: 500 });
doc.moveDown(1);
doc.addPage();

// ==================== 7. SYSTÈME MULTI-AGENTS ====================
addHeader('7. Système d\'Analyse Multi-Agents');
addBody('Six agents IA indépendants, chacun fine-tuné sur son domaine spécifique, travaillent en parallèle sur chaque analyse. Cette architecture multi-agents permet une profondeur impossible avec un seul modèle généraliste.');

addTable(
  ['Agent', 'Domaine', 'Exemples de détections'],
  [
    ['SecurityAgent', 'Vulnérabilités & attaques', 'Injection SQL/XSS/LDAP, timing attacks, crypto faible, OWASP Top 10'],
    ['LogicAgent', 'Bugs logiques & cohérence', 'Race conditions, deadlocks, edge cases, incohérences inter-modules'],
    ['PerformanceAgent', 'Efficacité & ressources', 'Memory leaks, N+1 queries, allocations inutiles, hot paths'],
    ['ComplianceAgent', 'Conformité réglementaire', 'GDPR Art.32, SOC2, ISO27001 A.12, HIPAA HITECH'],
    ['SecretAgent', 'Secrets & credentials', 'API keys hardcodées, tokens JWT exposés, mots de passe en clair'],
    ['SupplyChainAgent', 'Sécurité chaîne d\'appro.', 'Typosquatting, dépendances orphelines, CVE tierces, licences'],
  ],
  [110, 110, 280]
);
doc.moveDown(0.5);
addBody('L\'AgentOrchestrator gère l\'exécution parallèle via le runtime async Tokio, collectant les findings de tous les agents et les agrégant dans un rapport unifié avec scores de risque calculés.');
doc.addPage();

// ==================== 8. SUBCONSCIOUS ENGINE ====================
addHeader('8. Plongée dans le Subconscious Engine');
addSubHeader('8.1 Concept & Philosophie');
addBody('Le Subconscious Engine est la fonctionnalité la plus innovante de GatKeeper et celle qui le distingue radicalement de tout ce qui existe sur le marché. Inspiré par le fonctionnement du cerveau humain qui continue de traiter des informations pendant le sommeil — consolidant les apprentissages, résolvant des problèmes complexes, établissant des connexions non évidentes — le Subconscious Engine opère en arrière-plan 24 heures sur 24.');

addSubHeader('8.2 Les Trois Couches du Subconscient');
addBullet('ROUGE — Threat Simulation : Simule des attaquants réels opérant contre votre système. Modélise différents profils d\'adversaires et calcule les chemins d\'attaque les plus probables.');
addBullet('JAUNE — Chaos Engineering : Simule des conditions adverses pour identifier les points de rupture cachés dans l\'architecture. Inspiré du Chaos Engineering de Netflix et des pratiques SRE de Google.');
addBullet('VERTE — Code Evolution : Génère et évalue des alternatives améliorées au code existant. Agit comme un pair-programmer IA qui travaille pendant que l\'équipe dort.');

addSubHeader('8.3 Cycles de Simulation');
addTable(
  ['Phase', 'Fréquence', 'Durée', 'Contenu'],
  [
    ['Scan Léger', 'Toutes les heures', '~2 min', 'Nouveaux commits, alertes critiques, vérif. secrets'],
    ['Nightly', 'Chaque nuit', '~45 min', 'Projection 7 jours, MAJ DNA, corrélation KB'],
    ['Deep Sim.', 'Chaque semaine', '~6 heures', 'Projection 30 jours, rééval. architecture complète'],
    ['Grand Audit', 'Chaque trimestre', '~24 heures', 'Sim. exhaustive, rapport conformité, recs stratégiques'],
  ],
  [110, 100, 80, 210]
);
doc.addPage();

// ==================== 9. ANALYSE CONCURRENTIELLE ====================
addHeader('9. Analyse Concurrentielle');
addTable(
  ['Fonctionnalité', 'SonarQube', 'Snyk', 'Veracode', 'Semgrep', 'GatKeeper'],
  [
    ['Analyse statique (SAST)', '✓', 'Partiel', '✓', '✓', '✓'],
    ['Analyse dépendances (SCA)', 'Partiel', '✓', '✓', 'Partiel', '✓'],
    ['Scan de secrets', 'Partiel', '✓', 'Partiel', '✓', '✓'],
    ['IA multi-agents', '✗', '✗', '✗', '✗', '✓'],
    ['Mémoire évolutive (DNA)', '✗', '✗', '✗', '✗', '✓'],
    ['Subconscious Engine 24/7', '✗', '✗', '✗', '✗', '✓'],
    ['Simulation 30 jours', '✗', '✗', '✗', '✗', '✓'],
    ['Sandbox d\'attaque (PoC)', '✗', '✗', 'Partiel', '✗', '✓'],
    ['Mode What-If', '✗', '✗', '✗', '✗', '✓'],
    ['Compliance automatique', 'Partiel', 'Partiel', '✓', '✗', '✓'],
    ['Knowledge Base collective', '✗', '✗', '✗', '✗', '✓'],
    ['Cross-repository', '✗', 'Partiel', '✗', '✗', '✓'],
    ['Prép. post-quantique', '✗', '✗', '✗', '✗', '✓'],
    ['100% on-premise', '✓', '✗', '✗', '✓', '✓'],
    ['Rapport versionnable', 'Partiel', '✗', '✗', '✗', '✓'],
  ],
  [110, 70, 60, 70, 70, 90]
);
doc.addPage();

// ==================== 10. FORMAT .GAT ====================
addHeader('10. Spécification du Format .gat');
addBody('Le fichier .gat est bien plus qu\'un simple rapport de sortie. C\'est un artefact structuré, versionnable et exploitable en machine qui documente l\'intégralité de l\'état de santé d\'un projet à un instant T. Conçu pour être attaché aux commits Git, intégré dans les pipelines CI/CD et consommé par des outils tiers via API.');

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
duree_analyse = "6h32m (arrière-plan)"

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
description = "Comparaison token non-constant-time, exploitable"
correction = "Utiliser subtle::ConstantTimeEq"
effort_correction = "15 minutes"

[[subconscious.dreams]]
id = "DREAM-001"
couche = "threat"
decouverte_a = "03:47 AM"
horizon = "23 jours"
probabilite = 94
impact = "accès admin non autorisé"
condition = "trafic > 10k req/s"

[subconscious.intuitions]
items = [
  "auth/ et payments/ divergent silencieusement.",
  "crypto-lib abandonnée. Probabilité CVE 60j: 71%."
]`.trim();

doc.text(gatExample, 50, doc.y, { width: 500 });
doc.moveDown(1);
doc.addPage();

// ==================== 11. ROADMAP ====================
addHeader('11. Roadmap Phase 1 (3 Mois)');
addSubHeader('Mois 1 — Fondations');
addBullet('Mise en place du workspace Cargo avec toute la structure de crates');
addBullet('Implémentation du CLI avec clap (init, scan, report, status)');
addBullet('Intégration du parser Tree-sitter pour 5 langages (Rust, Python, JS, C, Go)');
addBullet('Implémentation basique du SecurityAgent (injection, crypto, OWASP)');
addBullet('Implémentation du SecretAgent avec détection de credentials hardcodés');
addBullet('Génération du rapport .gat au format TOML');
addBullet('Schéma SQLite pour le stockage du DNA Engine');

addSubHeader('Mois 2 — Intelligence');
addBullet('Complétion des 6 agents avec analyse basée sur les patterns');
addBullet('Implémentation de l\'AgentOrchestrator avec exécution parallèle Tokio');
addBullet('Subconscious Engine basique avec cycle horaire et projection 24h');
addBullet('DNA Engine v1 : fingerprinting, baseline, détection de patterns');
addBullet('SupplyChainAgent avec intégration API OSV/NVD pour vérification CVE');
addBullet('Intégration CI/CD GitHub Actions');

addSubHeader('Mois 3 — Polish & Lancement');
addBullet('Plugin Neovim basique');
addBullet('Documentation complète et guide de démarrage');
addBullet('Suite de tests d\'intégration avec fixtures de projets vulnérables');
addBullet('Optimisation des performances et durcissement de la gestion d\'erreurs');
addBullet('README, guide de contribution et mise en place de la communauté');
addBullet('Lancement du dépôt GitHub public');
doc.addPage();

// ==================== 12. MONÉTISATION ====================
addHeader('12. Monétisation & Stratégie Go-to-Market');
addSubHeader('Plans Tarifaires');
addTable(
  ['Plan', 'Cible', 'Prix', 'Fonctionnalités clés'],
  [
    ['Community', 'Dév. individuels, OSS', 'Gratuit', '1 projet, CLI, SecurityAgent basique, .gat, sub. 24h'],
    ['Pro', 'Freelances, startups', '49€/mois', 'Projets illimités, 6 agents, dashboard, CI/CD, sub. 7j'],
    ['Business', 'Équipes 5-50', '299€/mois/équipe', 'Compliance, sandbox PoC, Horizon, KB, sub. 30j'],
    ['Enterprise', 'Grandes orgs 50+', '50K-500K$/an', 'On-premise, fine-tuning, Telepathy, Quantum, SLA'],
  ],
  [85, 110, 85, 220]
);

addSubHeader('Stratégie Go-to-Market');
addBullet('Phase 1 : Community open source → adoption organique par les développeurs');
addBullet('Phase 2 : Product-led growth → upgrade naturel vers Pro via feature gates');
addBullet('Phase 3 : Sales-led growth → équipe commerciale ciblant les CISO');
addBullet('Partenariats stratégiques : intégrateurs (Accenture, Deloitte), cloud providers');
addBullet('Conférences : DEF CON, Black Hat, RSA Conference, KubeCon');

// ==================== FINAL ====================
doc.addPage();
doc.rect(0, 0, 595, 842).fill(BLUE);
doc.fontSize(32).font('Helvetica-Bold').fillColor(WHITE).text('GatKeeper', 60, 300, { width: 480, align: 'center' });
doc.moveDown(1);
doc.fontSize(16).font('Helvetica').fillColor(LIGHT_BLUE).text('Le premier outil de sécurité qui pense en permanence.', 60, 360, { align: 'center' });
doc.moveDown(2);
doc.fontSize(12).fillColor(GRAY).text('Document généré par l\'Analyse de Projet GatKeeper', 60, 420, { align: 'center' });
doc.text('Version 1.0 — Juin 2026', 60, 440, { align: 'center' });
doc.moveDown(3);
doc.fontSize(10).fillColor(GRAY).text('"Pendant que tu dormais, j\'ai trouvé 3 choses que tu dois savoir."', 60, 500, { align: 'center' });

doc.end();

stream.on('finish', () => {
  console.log('✅ PDF FR généré : docs/GatKeeper_Analysis_FR.pdf');
});
