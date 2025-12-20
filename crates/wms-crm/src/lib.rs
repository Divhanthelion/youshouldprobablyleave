//! WMS CRM Module
//! 
//! Provides customer relationship management functionality including:
//! - Customer master data management
//! - Contact management
//! - Address management
//! - Data validation (email, phone)
//! - Customer search and filtering

mod models;
mod service;
mod validation;

pub use models::*;
pub use service::CrmService;
pub use validation::*;

