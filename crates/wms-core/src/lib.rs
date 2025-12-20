//! WMS Core Library
//! 
//! This crate provides core utilities shared across all WMS modules:
//! - Database connection and migration management
//! - Common types and traits
//! - Error handling utilities

pub mod db;
pub mod error;
pub mod types;

pub use db::Database;
pub use error::{WmsError, Result};
pub use types::*;

