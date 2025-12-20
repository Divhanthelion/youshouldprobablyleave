//! Database Management Module
//! 
//! Provides SQLite database connection management with SQLCipher encryption
//! and schema migration support.

use std::path::Path;
use std::sync::Mutex;
use rusqlite::{Connection, params};
use tracing::{info, debug};
use crate::error::{WmsError, Result};

/// Database wrapper providing thread-safe access to SQLite with encryption
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Create a new database connection with encryption
    pub fn new(path: &Path, encryption_key: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Enable SQLCipher encryption
        conn.pragma_update(None, "key", encryption_key)?;
        
        // Enable foreign keys
        conn.pragma_update(None, "foreign_keys", "ON")?;
        
        // Enable WAL mode for better concurrent access
        conn.pragma_update(None, "journal_mode", "WAL")?;
        
        info!("Database connection established");
        
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
    
    /// Run all database migrations
    pub fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| WmsError::LockError)?;
        
        // Create migrations tracking table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS _migrations (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        // Apply migrations
        let migrations = get_migrations();
        for (name, sql) in migrations {
            if !self.migration_applied(&conn, name)? {
                info!("Applying migration: {}", name);
                conn.execute_batch(sql)?;
                conn.execute(
                    "INSERT INTO _migrations (name) VALUES (?)",
                    params![name],
                )?;
            } else {
                debug!("Migration already applied: {}", name);
            }
        }
        
        Ok(())
    }
    
    fn migration_applied(&self, conn: &Connection, name: &str) -> Result<bool> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM _migrations WHERE name = ?",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
    
    /// Execute a query with parameters
    pub fn execute<P>(&self, sql: &str, params: P) -> Result<usize>
    where
        P: rusqlite::Params,
    {
        let conn = self.conn.lock().map_err(|_| WmsError::LockError)?;
        let rows = conn.execute(sql, params)?;
        Ok(rows)
    }
    
    /// Query and map results
    pub fn query_map<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<Vec<T>>
    where
        P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let conn = self.conn.lock().map_err(|_| WmsError::LockError)?;
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params, f)?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
    
    /// Query a single row
    pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<Option<T>>
    where
        P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let conn = self.conn.lock().map_err(|_| WmsError::LockError)?;
        match conn.query_row(sql, params, f) {
            Ok(result) => Ok(Some(result)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    /// Begin a transaction
    pub fn transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let mut conn = self.conn.lock().map_err(|_| WmsError::LockError)?;
        let tx = conn.transaction()?;
        let result = f(&tx)?;
        tx.commit()?;
        Ok(result)
    }
}

/// Get all database migrations in order
fn get_migrations() -> Vec<(&'static str, &'static str)> {
    vec![
        ("001_initial_schema", include_str!("migrations/001_initial_schema.sql")),
        ("002_inventory_tables", include_str!("migrations/002_inventory_tables.sql")),
        ("003_shipping_tables", include_str!("migrations/003_shipping_tables.sql")),
        ("004_delivery_tables", include_str!("migrations/004_delivery_tables.sql")),
        ("005_crm_tables", include_str!("migrations/005_crm_tables.sql")),
        ("006_timesheet_tables", include_str!("migrations/006_timesheet_tables.sql")),
        ("007_sync_tables", include_str!("migrations/007_sync_tables.sql")),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_database_creation() {
        let path = PathBuf::from(":memory:");
        let db = Database::new(&path, "test-key").unwrap();
        db.run_migrations().unwrap();
    }
}

