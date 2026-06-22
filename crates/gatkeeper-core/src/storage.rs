use crate::finding::{Finding, Severity};
use crate::risk::RiskScore;
use crate::scan::ScanResult;
use rusqlite::{params, Connection};
use std::path::Path;

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(project_root: &Path) -> Result<Self, StorageError> {
        let db_path = project_root.join(".gatkeeper").join("gatkeeper.db");
        std::fs::create_dir_all(db_path.parent().unwrap())?;
        let conn = Connection::open(&db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<(), StorageError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS projects (
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
                agents_used TEXT,
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
            CREATE INDEX IF NOT EXISTS idx_scans_project ON scans(project_id);",
        )?;
        Ok(())
    }

    pub fn ensure_project(&self, name: &str, root_path: &str) -> Result<i64, StorageError> {
        let existing: Option<i64> = self
            .conn
            .query_row(
                "SELECT id FROM projects WHERE root_path = ?1",
                params![root_path],
                |row| row.get(0),
            )
            .ok();

        if let Some(id) = existing {
            self.conn.execute(
                "UPDATE projects SET updated_at = datetime('now'), name = ?1 WHERE id = ?2",
                params![name, id],
            )?;
            return Ok(id);
        }

        self.conn.execute(
            "INSERT INTO projects (name, root_path) VALUES (?1, ?2)",
            params![name, root_path],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn save_scan(&self, project_id: i64, scan: &ScanResult) -> Result<i64, StorageError> {
        let agents_json = serde_json::to_string(&scan.agents_used).unwrap_or_default();

        self.conn.execute(
            "INSERT INTO scans (project_id, duration_ms, files_analyzed, lines_analyzed, agents_used,
             risk_score_global, risk_score_security, risk_score_performance, risk_score_compliance)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                project_id,
                scan.duration_ms,
                scan.files_analyzed,
                scan.lines_analyzed as i64,
                agents_json,
                scan.risk_score.global,
                scan.risk_score.security,
                scan.risk_score.performance,
                scan.risk_score.compliance,
            ],
        )?;
        let scan_id = self.conn.last_insert_rowid();

        for finding in &scan.findings {
            self.conn.execute(
                "INSERT INTO findings (scan_id, finding_id, agent, severity, file, line_start, line_end,
                 finding_type, cvss_score, description, correction, effort_correction)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![
                    scan_id,
                    finding.id,
                    finding.agent,
                    finding.severity.label(),
                    finding.file,
                    finding.line_start,
                    finding.line_end,
                    finding.finding_type,
                    finding.cvss_score,
                    finding.description,
                    finding.correction,
                    finding.effort_correction,
                ],
            )?;
        }

        Ok(scan_id)
    }

    pub fn get_last_scan(&self, project_id: i64) -> Result<Option<ScanResult>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, duration_ms, files_analyzed, lines_analyzed, agents_used,
             risk_score_global, risk_score_security, risk_score_performance, risk_score_compliance
             FROM scans WHERE project_id = ?1 ORDER BY id DESC LIMIT 1",
        )?;

        let row = stmt
            .query_row(params![project_id], |row| {
                let scan_id: i64 = row.get(0)?;
                let timestamp: String = row.get(1)?;
                let duration_ms: u64 = row.get(2)?;
                let files_analyzed: u32 = row.get(3)?;
                let lines_analyzed: i64 = row.get(4)?;
                let agents_json: String = row.get(5)?;
                let global: u32 = row.get(6)?;
                let security: u32 = row.get(7)?;
                let performance: u32 = row.get(8)?;
                let compliance: u32 = row.get(9)?;

                let agents_used: Vec<String> =
                    serde_json::from_str(&agents_json).unwrap_or_default();

                let mut fstmt = self.conn.prepare(
                    "SELECT finding_id, agent, severity, file, line_start, line_end, finding_type,
                 cvss_score, description, correction, effort_correction
                 FROM findings WHERE scan_id = ?1",
                )?;

                let findings = fstmt
                    .query_map(params![scan_id], |frow| {
                        let sev_str: String = frow.get(2)?;
                        let severity = match sev_str.as_str() {
                            "CRITICAL" => Severity::Critical,
                            "HIGH" => Severity::High,
                            "MEDIUM" => Severity::Medium,
                            "LOW" => Severity::Low,
                            _ => Severity::Info,
                        };
                        Ok(Finding {
                            id: frow.get(0)?,
                            agent: frow.get(1)?,
                            severity,
                            file: frow.get(3)?,
                            line_start: frow.get(4)?,
                            line_end: frow.get(5)?,
                            finding_type: frow.get(6)?,
                            cvss_score: frow.get(7)?,
                            description: frow.get(8)?,
                            correction: frow.get(9)?,
                            effort_correction: frow.get(10)?,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ScanResult {
                    project_name: String::new(),
                    timestamp,
                    duration_ms,
                    files_analyzed,
                    lines_analyzed: lines_analyzed as u64,
                    agents_used,
                    findings,
                    risk_score: RiskScore {
                        global,
                        security,
                        performance,
                        compliance,
                        tendency: "stable".to_string(),
                    },
                })
            })
            .ok();

        Ok(row)
    }

    pub fn get_scan_history(
        &self,
        project_id: i64,
        limit: u32,
    ) -> Result<Vec<(i64, String, u32)>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, risk_score_global FROM scans WHERE project_id = ?1 ORDER BY id DESC LIMIT ?2",
        )?;

        let rows = stmt
            .query_map(params![project_id, limit], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    pub fn get_findings_by_severity(
        &self,
        scan_id: i64,
    ) -> Result<Vec<(String, u32)>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT severity, COUNT(*) FROM findings WHERE scan_id = ?1 GROUP BY severity ORDER BY COUNT(*) DESC",
        )?;

        let rows = stmt
            .query_map(params![scan_id], |row| {
                Ok((row.get(0)?, row.get::<_, u32>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
