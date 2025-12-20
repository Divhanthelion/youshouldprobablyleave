//! CRDT Document Management
//! 
//! Wrapper around Automerge for managing conflict-free replicated data types.

use automerge::{AutoCommit, ObjType, Prop, ReadDoc, transaction::Transactable};
use serde::{Deserialize, Serialize};
use wms_core::error::{WmsError, Result};

/// A CRDT document backed by Automerge
pub struct CrdtDocument {
    doc: AutoCommit,
}

impl CrdtDocument {
    /// Create a new empty CRDT document
    pub fn new() -> Self {
        Self {
            doc: AutoCommit::new(),
        }
    }
    
    /// Create a document from saved changes
    pub fn from_changes(bytes: &[u8]) -> Result<Self> {
        let doc = AutoCommit::load(bytes)
            .map_err(|e| WmsError::SyncError(format!("Failed to load CRDT: {}", e)))?;
        Ok(Self { doc })
    }
    
    /// Save the document to bytes
    pub fn save(&self) -> Result<Vec<u8>> {
        Ok(self.doc.save())
    }
    
    /// Merge changes from another document
    pub fn merge(&mut self, other_changes: &[u8]) -> Result<()> {
        let other = AutoCommit::load(other_changes)
            .map_err(|e| WmsError::SyncError(format!("Failed to load other CRDT: {}", e)))?;
        
        self.doc.merge(&mut other.clone())
            .map_err(|e| WmsError::SyncError(format!("Failed to merge CRDTs: {}", e)))?;
        
        Ok(())
    }
    
    /// Get the document heads as JSON for storage
    pub fn get_heads_json(&self) -> Result<String> {
        let heads: Vec<String> = self.doc.get_heads()
            .iter()
            .map(|h| h.to_string())
            .collect();
        
        serde_json::to_string(&heads)
            .map_err(|e| WmsError::Serialization(e))
    }
    
    /// Convert document to JSON
    pub fn to_json(&self) -> Result<String> {
        let json_value = automerge::AutoSerde::from(&self.doc);
        serde_json::to_string(&json_value)
            .map_err(|e| WmsError::Serialization(e))
    }
    
    /// Set a value in the document
    pub fn set(&mut self, key: &str, value: CrdtValue) -> Result<()> {
        match value {
            CrdtValue::String(s) => {
                self.doc.put(automerge::ROOT, key, s)
                    .map_err(|e| WmsError::SyncError(e.to_string()))?;
            }
            CrdtValue::Int(i) => {
                self.doc.put(automerge::ROOT, key, i)
                    .map_err(|e| WmsError::SyncError(e.to_string()))?;
            }
            CrdtValue::Float(f) => {
                self.doc.put(automerge::ROOT, key, f)
                    .map_err(|e| WmsError::SyncError(e.to_string()))?;
            }
            CrdtValue::Bool(b) => {
                self.doc.put(automerge::ROOT, key, b)
                    .map_err(|e| WmsError::SyncError(e.to_string()))?;
            }
            CrdtValue::Null => {
                self.doc.put(automerge::ROOT, key, ())
                    .map_err(|e| WmsError::SyncError(e.to_string()))?;
            }
        }
        Ok(())
    }
    
    /// Get a string value from the document
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.doc.get(automerge::ROOT, key)
            .ok()
            .flatten()
            .and_then(|(val, _)| val.to_str().map(|s| s.to_string()))
    }
    
    /// Get an integer value from the document
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.doc.get(automerge::ROOT, key)
            .ok()
            .flatten()
            .and_then(|(val, _)| val.to_i64())
    }
    
    /// Get a float value from the document
    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.doc.get(automerge::ROOT, key)
            .ok()
            .flatten()
            .and_then(|(val, _)| val.to_f64())
    }
    
    /// Create or get a list in the document
    pub fn create_list(&mut self, key: &str) -> Result<CrdtList> {
        let obj_id = self.doc.put_object(automerge::ROOT, key, ObjType::List)
            .map_err(|e| WmsError::SyncError(e.to_string()))?;
        
        Ok(CrdtList {
            obj_id,
        })
    }
    
    /// Add an operation to a list (for inventory adjustments)
    pub fn push_operation(&mut self, list_key: &str, operation: &CrdtOperation) -> Result<()> {
        // Get or create the list
        let list_id = match self.doc.get(automerge::ROOT, list_key)? {
            Some((_, id)) => id,
            None => self.doc.put_object(automerge::ROOT, list_key, ObjType::List)
                .map_err(|e| WmsError::SyncError(e.to_string()))?,
        };
        
        // Create operation object
        let op_id = self.doc.insert_object(&list_id, 0, ObjType::Map)
            .map_err(|e| WmsError::SyncError(e.to_string()))?;
        
        self.doc.put(&op_id, "type", &operation.op_type)
            .map_err(|e| WmsError::SyncError(e.to_string()))?;
        self.doc.put(&op_id, "delta", operation.delta)
            .map_err(|e| WmsError::SyncError(e.to_string()))?;
        self.doc.put(&op_id, "user", &operation.user_id)
            .map_err(|e| WmsError::SyncError(e.to_string()))?;
        self.doc.put(&op_id, "id", &operation.id)
            .map_err(|e| WmsError::SyncError(e.to_string()))?;
        self.doc.put(&op_id, "timestamp", &operation.timestamp)
            .map_err(|e| WmsError::SyncError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Calculate final value by summing operations
    pub fn calculate_sum(&self, list_key: &str) -> Result<f64> {
        let list_id = match self.doc.get(automerge::ROOT, list_key)? {
            Some((_, id)) => id,
            None => return Ok(0.0),
        };
        
        let len = self.doc.length(&list_id);
        let mut sum = 0.0;
        
        for i in 0..len {
            if let Some((_, op_id)) = self.doc.get(&list_id, i)? {
                if let Some((val, _)) = self.doc.get(&op_id, "delta")? {
                    if let Some(delta) = val.to_f64() {
                        sum += delta;
                    }
                }
            }
        }
        
        Ok(sum)
    }
}

impl Default for CrdtDocument {
    fn default() -> Self {
        Self::new()
    }
}

/// Supported CRDT value types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrdtValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

/// A list within a CRDT document
pub struct CrdtList {
    obj_id: automerge::ObjId,
}

/// An operation record for CRDT lists (used for inventory adjustments)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdtOperation {
    pub id: String,
    pub op_type: String, // "pick", "receive", "adjust", "count"
    pub delta: f64,
    pub user_id: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl CrdtOperation {
    /// Create a new inventory operation
    pub fn new(op_type: &str, delta: f64, user_id: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            op_type: op_type.to_string(),
            delta,
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            notes: None,
        }
    }
    
    /// Add notes to the operation
    pub fn with_notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crdt_basic_operations() {
        let mut doc = CrdtDocument::new();
        doc.set("name", CrdtValue::String("Test Item".to_string())).unwrap();
        doc.set("quantity", CrdtValue::Int(100)).unwrap();
        
        assert_eq!(doc.get_string("name"), Some("Test Item".to_string()));
        assert_eq!(doc.get_int("quantity"), Some(100));
    }
    
    #[test]
    fn test_crdt_merge() {
        // Create two documents
        let mut doc1 = CrdtDocument::new();
        let mut doc2 = CrdtDocument::new();
        
        // Different users make changes
        doc1.set("user1_field", CrdtValue::String("value1".to_string())).unwrap();
        doc2.set("user2_field", CrdtValue::String("value2".to_string())).unwrap();
        
        // Merge doc2 into doc1
        let doc2_changes = doc2.save().unwrap();
        doc1.merge(&doc2_changes).unwrap();
        
        // Both values should be present
        assert_eq!(doc1.get_string("user1_field"), Some("value1".to_string()));
        assert_eq!(doc1.get_string("user2_field"), Some("value2".to_string()));
    }
    
    #[test]
    fn test_inventory_operations() {
        let mut doc = CrdtDocument::new();
        
        // Simulate inventory adjustments
        let op1 = CrdtOperation::new("receive", 100.0, "user1");
        let op2 = CrdtOperation::new("pick", -25.0, "user2");
        let op3 = CrdtOperation::new("pick", -10.0, "user1");
        
        doc.push_operation("adjustments", &op1).unwrap();
        doc.push_operation("adjustments", &op2).unwrap();
        doc.push_operation("adjustments", &op3).unwrap();
        
        // Calculate final quantity
        let total = doc.calculate_sum("adjustments").unwrap();
        assert_eq!(total, 65.0); // 100 - 25 - 10
    }
}

