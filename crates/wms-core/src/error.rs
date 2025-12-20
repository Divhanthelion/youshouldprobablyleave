//! Error Types for WMS

use thiserror::Error;

/// Result type alias for WMS operations
pub type Result<T> = std::result::Result<T, WmsError>;

/// WMS Error types
#[derive(Error, Debug)]
pub enum WmsError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Lock acquisition failed")]
    LockError,
    
    #[error("Sync error: {0}")]
    SyncError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Barcode error: {0}")]
    Barcode(String),
    
    #[error("Route optimization error: {0}")]
    RouteOptimization(String),
    
    #[error("Forecast error: {0}")]
    Forecast(String),
    
    #[error("Export error: {0}")]
    Export(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl WmsError {
    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
    
    /// Create a not found error
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
    
    /// Create a conflict error
    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }
}

