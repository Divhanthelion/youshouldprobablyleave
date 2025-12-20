//! Shipping Command Handlers

use tauri::State;
use crate::AppState;
use wms_shipping::{Shipment, ShipmentStatus, ShippingLabel, BarcodeResult};

/// Create a new shipment
#[tauri::command]
pub async fn create_shipment(
    state: State<'_, AppState>,
    shipment: Shipment,
) -> Result<Shipment, String> {
    state.shipping
        .create_shipment(shipment)
        .await
        .map_err(|e| e.to_string())
}

/// Get a shipment by ID
#[tauri::command]
pub async fn get_shipment(
    state: State<'_, AppState>,
    shipment_id: String,
) -> Result<Option<Shipment>, String> {
    state.shipping
        .get_shipment(&shipment_id)
        .await
        .map_err(|e| e.to_string())
}

/// Update shipment status
#[tauri::command]
pub async fn update_shipment_status(
    state: State<'_, AppState>,
    shipment_id: String,
    status: ShipmentStatus,
) -> Result<Shipment, String> {
    state.shipping
        .update_status(&shipment_id, status)
        .await
        .map_err(|e| e.to_string())
}

/// Generate a shipping label (ZPL format for thermal printers)
#[tauri::command]
pub async fn generate_shipping_label(
    state: State<'_, AppState>,
    shipment_id: String,
) -> Result<ShippingLabel, String> {
    state.shipping
        .generate_label(&shipment_id)
        .await
        .map_err(|e| e.to_string())
}

/// Decode a barcode from image data
#[tauri::command]
pub async fn scan_barcode(
    state: State<'_, AppState>,
    image_data: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<BarcodeResult, String> {
    state.shipping
        .decode_barcode(&image_data, width, height)
        .await
        .map_err(|e| e.to_string())
}

