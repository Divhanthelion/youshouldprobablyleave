//! WMS Inventory Module
//! 
//! Provides inventory management functionality including:
//! - Item master management
//! - Stock level tracking
//! - Inventory adjustments with CRDT support
//! - Demand forecasting using time series analysis
//! - ABC classification

mod models;
mod service;
mod forecast;

pub use models::*;
pub use service::InventoryService;
pub use forecast::{ForecastEngine, ForecastResult};

