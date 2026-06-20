use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;

pub struct DnaStore {
    conn: Connection,
}

impl DnaStore {
    pub fn open(db_path: &Path) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS fingerprints (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hash TEXT NOT NULL,
                file_count INTEGER,
                total_lines INTEGER,
                timestamp TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS findings_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                finding_id TEXT NOT NULL,
                agent TEXT NOT NULL,
                severity TEXT NOT NULL,
                file TEXT NOT NULL,
                line INTEGER,
                description TEXT,
                timestamp TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS baselines (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                root_hash TEXT NOT NULL,
                established TEXT NOT NULL,
                data TEXT NOT NULL
            );",
        )?;

        Ok(Self { conn })
    }

    pub fn save_fingerprint(&self, hash: &str, file_count: usize, total_lines: usize) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO fingerprints (hash, file_count, total_lines, timestamp) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![hash, file_count, total_lines, chrono::Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn save_finding(&self, finding_id: &str, agent: &str, severity: &str, file: &str, line: usize, description: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO findings_history (finding_id, agent, severity, file, line, description, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![finding_id, agent, severity, file, line, description, chrono::Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }
}
