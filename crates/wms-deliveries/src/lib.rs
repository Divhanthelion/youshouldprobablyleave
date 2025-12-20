//! WMS Deliveries Module
//! 
//! Provides delivery and logistics functionality including:
//! - Delivery route management
//! - Vehicle routing problem (VRP) optimization
//! - Geofencing and location tracking
//! - Driver management

mod models;
mod service;
mod routing;
mod geofence;

pub use models::*;
pub use service::DeliveryService;
pub use routing::{RouteOptimizer, OptimizedRoute};
pub use geofence::{GeofenceChecker, GeofenceResult};

