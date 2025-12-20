//! WMS Synchronization Engine
//! 
//! This crate provides offline-first synchronization using CRDTs (Automerge)
//! and a custom sync protocol for the Warehouse Management System.

mod engine;
mod crdt;
mod protocol;

pub use engine::{SyncEngine, SyncStatus};
pub use crdt::{CrdtDocument, CrdtOperation};
pub use protocol::{SyncMessage, SyncRequest, SyncResponse};

