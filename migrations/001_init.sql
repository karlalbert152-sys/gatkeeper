-- GatKeeper Database Schema v0.1.0
-- Phase 1: Core tables for DNA Engine and findings history

CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    root_path TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS scans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    duration_ms INTEGER,
    files_analyzed INTEGER,
    lines_analyzed INTEGER,
    agents_used TEXT, -- JSON array
    risk_score_global INTEGER,
    risk_score_security INTEGER,
    risk_score_performance INTEGER,
    risk_score_compliance INTEGER,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS findings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id INTEGER NOT NULL,
    finding_id TEXT NOT NULL,
    agent TEXT NOT NULL,
    severity TEXT NOT NULL,
    file TEXT NOT NULL,
    line_start INTEGER,
    line_end INTEGER,
    finding_type TEXT NOT NULL,
    cvss_score REAL,
    description TEXT NOT NULL,
    correction TEXT,
    effort_correction TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (scan_id) REFERENCES scans(id)
);

CREATE INDEX IF NOT EXISTS idx_findings_severity ON findings(severity);
CREATE INDEX IF NOT EXISTS idx_findings_agent ON findings(agent);
CREATE INDEX IF NOT EXISTS idx_findings_file ON findings(file);
CREATE INDEX IF NOT EXISTS idx_scans_project ON scans(project_id);
