//! Synchronization Engine
//! 
//! Manages bidirectional sync between local SQLite and remote server
//! using CRDTs for conflict-free merging.

use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, debug};
use wms_core::db::Database;
use wms_core::error::{WmsError, Result};
use crate::crdt::CrdtDocument;

/// Synchronization status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub is_syncing: bool,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub pending_changes: u64,
    pub sync_errors: u64,
    pub last_error: Option<String>,
    pub connection_status: ConnectionStatus,
}

/// Network connection status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Online,
    Offline,
    Slow,
    Unknown,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// The main synchronization engine
pub struct SyncEngine {
    db: Arc<Database>,
    status: SyncStatus,
    server_url: Option<String>,
    device_id: String,
}

impl SyncEngine {
    /// Create a new sync engine
    pub fn new(db: Arc<Database>) -> Result<Self> {
        let device_id = Self::get_or_create_device_id(&db)?;
        
        Ok(Self {
            db,
            status: SyncStatus {
                is_syncing: false,
                last_sync_at: None,
                pending_changes: 0,
                sync_errors: 0,
                last_error: None,
                connection_status: ConnectionStatus::Unknown,
            },
            server_url: std::env::var("WMS_SERVER_URL").ok(),
            device_id,
        })
    }
    
    /// Get or create a unique device ID
    fn get_or_create_device_id(db: &Database) -> Result<String> {
        let existing: Option<String> = db.query_row(
            "SELECT value FROM settings WHERE key = 'device_id'",
            [],
            |row| row.get(0),
        )?;
        
        if let Some(id) = existing {
            return Ok(id);
        }
        
        let new_id = uuid::Uuid::new_v4().to_string();
        db.execute(
            "INSERT INTO settings (key, value, description) VALUES ('device_id', ?, 'Unique device identifier')",
            rusqlite::params![&new_id],
        )?;
        
        Ok(new_id)
    }
    
    /// Get current sync status
    pub fn get_status(&self) -> SyncStatus {
        self.status.clone()
    }
    
    /// Perform synchronization
    pub async fn sync_now(&mut self) -> Result<SyncStatus> {
        if self.status.is_syncing {
            return Err(WmsError::SyncError("Sync already in progress".to_string()));
        }
        
        let server_url = self.server_url.clone()
            .ok_or_else(|| WmsError::SyncError("No server URL configured".to_string()))?;
        
        self.status.is_syncing = true;
        info!("Starting synchronization with server: {}", server_url);
        
        match self.perform_sync(&server_url).await {
            Ok(_) => {
                self.status.last_sync_at = Some(Utc::now());
                self.status.sync_errors = 0;
                self.status.last_error = None;
                info!("Synchronization completed successfully");
            }
            Err(e) => {
                self.status.sync_errors += 1;
                self.status.last_error = Some(e.to_string());
                error!("Synchronization failed: {}", e);
            }
        }
        
        self.status.is_syncing = false;
        self.update_pending_count()?;
        
        Ok(self.status.clone())
    }
    
    /// Internal sync logic
    async fn perform_sync(&self, server_url: &str) -> Result<()> {
        // Step 1: Get pending outbox items
        let pending = self.get_pending_changes()?;
        debug!("Found {} pending changes to sync", pending.len());
        
        // Step 2: Send local changes to server
        for change in &pending {
            self.send_change(server_url, change).await?;
        }
        
        // Step 3: Get server changes since last sync
        let server_changes = self.fetch_server_changes(server_url).await?;
        debug!("Received {} changes from server", server_changes.len());
        
        // Step 4: Apply server changes using CRDT merge
        for change in server_changes {
            self.apply_server_change(&change)?;
        }
        
        // Step 5: Mark sent changes as acknowledged
        for change in &pending {
            self.mark_change_acknowledged(&change.id)?;
        }
        
        Ok(())
    }
    
    /// Get pending changes from outbox
    fn get_pending_changes(&self) -> Result<Vec<OutboxItem>> {
        let items = self.db.query_map(
            "SELECT id, table_name, record_id, operation, payload, version, created_at 
             FROM sync_outbox 
             WHERE sent_at IS NULL 
             ORDER BY created_at ASC 
             LIMIT 100",
            [],
            |row| {
                Ok(OutboxItem {
                    id: row.get(0)?,
                    table_name: row.get(1)?,
                    record_id: row.get(2)?,
                    operation: row.get(3)?,
                    payload: row.get(4)?,
                    version: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )?;
        
        Ok(items)
    }
    
    /// Send a change to the server
    async fn send_change(&self, _server_url: &str, change: &OutboxItem) -> Result<()> {
        // TODO: Implement actual gRPC/HTTP call
        debug!("Sending change: {} {} {}", change.table_name, change.operation, change.record_id);
        
        // Mark as sent
        self.db.execute(
            "UPDATE sync_outbox SET sent_at = datetime('now') WHERE id = ?",
            rusqlite::params![&change.id],
        )?;
        
        Ok(())
    }
    
    /// Fetch changes from server
    async fn fetch_server_changes(&self, _server_url: &str) -> Result<Vec<ServerChange>> {
        // TODO: Implement actual gRPC/HTTP call
        // For now, return empty list
        Ok(Vec::new())
    }
    
    /// Apply a server change using CRDT merge
    fn apply_server_change(&self, change: &ServerChange) -> Result<()> {
        debug!("Applying server change: {} {} {}", 
               change.table_name, change.operation, change.record_id);
        
        // Load existing CRDT document
        let existing_doc = self.load_crdt_document(&change.table_name, &change.record_id)?;
        
        // Parse incoming changes
        let incoming_changes: Vec<u8> = change.crdt_changes.clone();
        
        // Merge using Automerge
        let mut merged_doc = if let Some(mut doc) = existing_doc {
            doc.merge(&incoming_changes)?;
            doc
        } else {
            CrdtDocument::from_changes(&incoming_changes)?
        };
        
        // Save merged document
        self.save_crdt_document(&change.table_name, &change.record_id, &mut merged_doc)?;
        
        // Apply to SQL table
        self.apply_to_sql_table(&change.table_name, &change.record_id, &merged_doc)?;
        
        Ok(())
    }
    
    /// Load a CRDT document from storage
    fn load_crdt_document(&self, doc_type: &str, record_id: &str) -> Result<Option<CrdtDocument>> {
        let data: Option<Vec<u8>> = self.db.query_row(
            "SELECT compressed_changes FROM crdt_documents 
             WHERE document_type = ? AND record_id = ?",
            rusqlite::params![doc_type, record_id],
            |row| row.get(0),
        )?;
        
        match data {
            Some(bytes) => Ok(Some(CrdtDocument::from_changes(&bytes)?)),
            None => Ok(None),
        }
    }
    
    /// Save a CRDT document to storage
    fn save_crdt_document(&self, doc_type: &str, record_id: &str, doc: &mut CrdtDocument) -> Result<()> {
        let changes = doc.save()?;
        let heads = doc.get_heads_json()?;
        
        self.db.execute(
            "INSERT INTO crdt_documents (id, document_type, record_id, actor_id, heads, compressed_changes, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, datetime('now'))
             ON CONFLICT(document_type, record_id) DO UPDATE SET
                heads = excluded.heads,
                compressed_changes = excluded.compressed_changes,
                version = version + 1,
                updated_at = datetime('now')",
            rusqlite::params![
                uuid::Uuid::new_v4().to_string(),
                doc_type,
                record_id,
                &self.device_id,
                &heads,
                &changes,
            ],
        )?;
        
        Ok(())
    }
    
    /// Apply CRDT document state to SQL table
    fn apply_to_sql_table(&self, table_name: &str, record_id: &str, doc: &CrdtDocument) -> Result<()> {
        let data = doc.to_json()?;
        
        // Insert into inbox for later processing
        self.db.execute(
            "INSERT INTO sync_inbox (id, table_name, record_id, operation, payload, server_version, received_at)
             VALUES (?, ?, ?, 'MERGE', ?, 0, datetime('now'))",
            rusqlite::params![
                uuid::Uuid::new_v4().to_string(),
                table_name,
                record_id,
                &data,
            ],
        )?;
        
        Ok(())
    }
    
    /// Mark a change as acknowledged
    fn mark_change_acknowledged(&self, change_id: &str) -> Result<()> {
        self.db.execute(
            "UPDATE sync_outbox SET acknowledged_at = datetime('now') WHERE id = ?",
            rusqlite::params![change_id],
        )?;
        Ok(())
    }
    
    /// Update pending change count in status
    fn update_pending_count(&mut self) -> Result<()> {
        let count: i64 = self.db.query_row(
            "SELECT COUNT(*) FROM sync_outbox WHERE acknowledged_at IS NULL",
            [],
            |row| row.get(0),
        )?.unwrap_or(0);
        
        self.status.pending_changes = count as u64;
        Ok(())
    }
    
    /// Queue a local change for sync
    pub fn queue_change(&self, table_name: &str, record_id: &str, operation: &str, payload: &str) -> Result<()> {
        self.db.execute(
            "INSERT INTO sync_outbox (id, table_name, record_id, operation, payload, version, created_at)
             VALUES (?, ?, ?, ?, ?, 1, datetime('now'))",
            rusqlite::params![
                uuid::Uuid::new_v4().to_string(),
                table_name,
                record_id,
                operation,
                payload,
            ],
        )?;
        Ok(())
    }
    
    /// Update connection status
    pub fn set_connection_status(&mut self, status: ConnectionStatus) {
        self.status.connection_status = status;
    }
}

/// Outbox item representing a pending local change
#[derive(Debug)]
struct OutboxItem {
    id: String,
    table_name: String,
    record_id: String,
    operation: String,
    payload: String,
    version: i64,
    created_at: String,
}

/// Server change to be applied locally
#[derive(Debug)]
struct ServerChange {
    table_name: String,
    record_id: String,
    operation: String,
    crdt_changes: Vec<u8>,
}

