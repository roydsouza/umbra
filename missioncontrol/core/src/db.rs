//! SQLite database for persistent state
//!
//! Stores metrics history, configuration state, and session data.

use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, params};
use tracing::info;

/// Database wrapper for SQLite operations
/// 
/// Uses Mutex for thread-safe access in async context.
#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Open or create the database
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
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

            -- Leak events from Guardian
            CREATE TABLE IF NOT EXISTS leak_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL DEFAULT (datetime('now')),
                process_name TEXT,
                process_pid INTEGER,
                process_path TEXT,
                dest_ip TEXT,
                dest_port INTEGER,
                severity TEXT NOT NULL,
                event_type TEXT,
                binary_info TEXT
            );
            
            CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics(timestamp);
            CREATE INDEX IF NOT EXISTS idx_metrics_type ON metrics(metric_type);
            CREATE INDEX IF NOT EXISTS idx_leak_events_timestamp ON leak_events(timestamp);
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

    /// Insert a leak event into the database
    pub fn insert_leak_event(&self, event: &LeakEvent) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"INSERT INTO leak_events (
                process_name, process_pid, process_path, 
                dest_ip, dest_port, severity, event_type, binary_info
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
            params![
                event.process_name,
                event.process_pid,
                event.process_path,
                event.dest_ip,
                event.dest_port,
                event.severity,
                event.event_type,
                event.binary_info,
            ],
        )?;
        Ok(())
    }

    /// Get recent leak history
    pub fn get_leak_history(&self, limit: usize) -> anyhow::Result<Vec<LeakEvent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            r#"SELECT 
                id, timestamp, process_name, process_pid, process_path, 
                dest_ip, dest_port, severity, event_type, binary_info 
            FROM leak_events ORDER BY timestamp DESC LIMIT ?1"#
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            Ok(LeakEvent {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                process_name: row.get(2)?,
                process_pid: row.get(3)?,
                process_path: row.get(4)?,
                dest_ip: row.get(5)?,
                dest_port: row.get(6)?,
                severity: row.get(7)?,
                event_type: row.get(8)?,
                binary_info: row.get(9)?,
            })
        })?;

        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }
        Ok(events)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricPoint {
    pub timestamp: String,
    pub value: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LeakEvent {
    pub id: Option<i64>,
    pub timestamp: Option<String>,
    pub process_name: Option<String>,
    pub process_pid: Option<i32>,
    pub process_path: Option<String>,
    pub dest_ip: Option<String>,
    pub dest_port: Option<u16>,
    pub severity: String,
    pub event_type: Option<String>,
    pub binary_info: Option<String>,
}
