//! SQLite database for persistent state
//!
//! Stores metrics history, configuration state, and session data.

use std::path::Path;
use std::sync::Mutex;
use rusqlite::{Connection, params};
use tracing::info;

/// Database wrapper for SQLite operations
/// 
/// Uses Mutex for thread-safe access in async context.
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Open or create the database
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let conn = Connection::open(path)?;
        let db = Self { conn: Mutex::new(conn) };
        db.init_schema()?;
        
        info!("Database initialized at {:?}", path);
        Ok(db)
    }
    
    /// Initialize database schema
    fn init_schema(&self) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(r#"
            -- Metrics history (for charts)
            CREATE TABLE IF NOT EXISTS metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL DEFAULT (datetime('now')),
                metric_type TEXT NOT NULL,
                value REAL NOT NULL,
                labels TEXT
            );
            
            -- Circuit history
            CREATE TABLE IF NOT EXISTS circuits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                circuit_id TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                status TEXT NOT NULL,
                guard_relay TEXT,
                middle_relay TEXT,
                exit_relay TEXT
            );
            
            -- Onion service state
            CREATE TABLE IF NOT EXISTS onion_services (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                nickname TEXT UNIQUE NOT NULL,
                onion_address TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_seen TEXT,
                status TEXT NOT NULL DEFAULT 'stopped'
            );
            
            -- Integration status
            CREATE TABLE IF NOT EXISTS integrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                last_request TEXT,
                request_count INTEGER NOT NULL DEFAULT 0
            );
            
            CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics(timestamp);
            CREATE INDEX IF NOT EXISTS idx_metrics_type ON metrics(metric_type);
        "#)?;
        
        Ok(())
    }
    
    /// Record a metric value
    pub fn record_metric(&self, metric_type: &str, value: f64, labels: Option<&str>) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO metrics (metric_type, value, labels) VALUES (?1, ?2, ?3)",
            params![metric_type, value, labels],
        )?;
        Ok(())
    }
    
    /// Get recent metrics of a given type
    pub fn get_recent_metrics(&self, metric_type: &str, limit: usize) -> anyhow::Result<Vec<MetricPoint>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT timestamp, value FROM metrics WHERE metric_type = ?1 ORDER BY timestamp DESC LIMIT ?2"
        )?;
        
        let rows = stmt.query_map(params![metric_type, limit], |row| {
            Ok(MetricPoint {
                timestamp: row.get(0)?,
                value: row.get(1)?,
            })
        })?;
        
        let mut metrics = Vec::new();
        for row in rows {
            metrics.push(row?);
        }
        Ok(metrics)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricPoint {
    pub timestamp: String,
    pub value: f64,
}
