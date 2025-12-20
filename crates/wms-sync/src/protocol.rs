//! Sync Protocol Messages
//! 
//! Defines the message format for synchronization between client and server.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A sync message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMessage {
    pub id: String,
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub payload: SyncPayload,
}

/// Sync message payload types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SyncPayload {
    Request(SyncRequest),
    Response(SyncResponse),
    Push(SyncPush),
    Ack(SyncAck),
}

/// Request for changes from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    /// Tables to sync
    pub tables: Vec<String>,
    /// Last known version for each table
    pub versions: Vec<TableVersion>,
    /// Maximum number of changes to receive
    pub limit: Option<u32>,
}

/// Response from server with changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    pub changes: Vec<ChangeRecord>,
    pub has_more: bool,
    pub server_time: DateTime<Utc>,
}

/// Push local changes to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPush {
    pub changes: Vec<ChangeRecord>,
}

/// Acknowledgment of received changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAck {
    pub change_ids: Vec<String>,
    pub success: bool,
    pub errors: Vec<SyncError>,
}

/// Version info for a table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableVersion {
    pub table_name: String,
    pub version: i64,
    pub last_sync_at: Option<DateTime<Utc>>,
}

/// A single change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRecord {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub operation: ChangeOperation,
    pub version: i64,
    pub timestamp: DateTime<Utc>,
    pub actor_id: String,
    /// JSON payload for non-CRDT data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_payload: Option<String>,
    /// Binary CRDT changes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "base64_serde")]
    pub crdt_changes: Option<Vec<u8>>,
}

/// Type of change operation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChangeOperation {
    Insert,
    Update,
    Delete,
    Merge, // CRDT merge
}

/// Sync error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncError {
    pub change_id: String,
    pub error_code: String,
    pub message: String,
}

/// Base64 serialization for binary data
mod base64_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match bytes {
            Some(b) => serializer.serialize_str(&STANDARD.encode(b)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => STANDARD
                .decode(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

impl SyncMessage {
    /// Create a new sync request message
    pub fn request(device_id: &str, tables: Vec<String>, versions: Vec<TableVersion>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            device_id: device_id.to_string(),
            timestamp: Utc::now(),
            payload: SyncPayload::Request(SyncRequest {
                tables,
                versions,
                limit: Some(100),
            }),
        }
    }
    
    /// Create a new sync push message
    pub fn push(device_id: &str, changes: Vec<ChangeRecord>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            device_id: device_id.to_string(),
            timestamp: Utc::now(),
            payload: SyncPayload::Push(SyncPush { changes }),
        }
    }
}

impl ChangeRecord {
    /// Create a new change record for JSON data
    pub fn json(
        table_name: &str,
        record_id: &str,
        operation: ChangeOperation,
        actor_id: &str,
        payload: &str,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            table_name: table_name.to_string(),
            record_id: record_id.to_string(),
            operation,
            version: 1,
            timestamp: Utc::now(),
            actor_id: actor_id.to_string(),
            json_payload: Some(payload.to_string()),
            crdt_changes: None,
        }
    }
    
    /// Create a new change record for CRDT data
    pub fn crdt(
        table_name: &str,
        record_id: &str,
        actor_id: &str,
        changes: Vec<u8>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            table_name: table_name.to_string(),
            record_id: record_id.to_string(),
            operation: ChangeOperation::Merge,
            version: 1,
            timestamp: Utc::now(),
            actor_id: actor_id.to_string(),
            json_payload: None,
            crdt_changes: Some(changes),
        }
    }
}

