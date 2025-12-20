//! Delivery Command Handlers

use tauri::State;
use crate::AppState;
use wms_deliveries::{Delivery, DeliveryStatus, OptimizedRoute, GeoPoint, GeofenceResult};

/// Get all deliveries with optional filters
#[tauri::command]
pub async fn get_deliveries(
    state: State<'_, AppState>,
    status: Option<DeliveryStatus>,
    date: Option<String>,
) -> Result<Vec<Delivery>, String> {
    state.deliveries
        .get_deliveries(status, date.as_deref())
        .await
        .map_err(|e| e.to_string())
}

/// Create a new delivery
#[tauri::command]
pub async fn create_delivery(
    state: State<'_, AppState>,
    delivery: Delivery,
) -> Result<Delivery, String> {
    state.deliveries
        .create_delivery(delivery)
        .await
        .map_err(|e| e.to_string())
}

/// Update delivery status
#[tauri::command]
pub async fn update_delivery_status(
    state: State<'_, AppState>,
    delivery_id: String,
    status: DeliveryStatus,
    location: Option<GeoPoint>,
) -> Result<Delivery, String> {
    state.deliveries
        .update_status(&delivery_id, status, location)
        .await
        .map_err(|e| e.to_string())
}

/// Optimize route for multiple delivery stops
#[tauri::command]
pub async fn optimize_route(
    state: State<'_, AppState>,
    delivery_ids: Vec<String>,
    start_location: GeoPoint,
) -> Result<OptimizedRoute, String> {
    state.deliveries
        .optimize_route(&delivery_ids, start_location)
        .await
        .map_err(|e| e.to_string())
}

/// Check if current location is within a delivery geofence
#[tauri::command]
pub async fn check_geofence(
    state: State<'_, AppState>,
    delivery_id: String,
    current_location: GeoPoint,
) -> Result<GeofenceResult, String> {
    state.deliveries
        .check_geofence(&delivery_id, current_location)
        .await
        .map_err(|e| e.to_string())
}

