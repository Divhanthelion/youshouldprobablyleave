//! Tauri API Bindings
//! 
//! Provides type-safe bindings to Tauri backend commands.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Invoke a Tauri command
pub async fn tauri_invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let args_js = serde_wasm_bindgen::to_value(args)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    let result = invoke(cmd, args_js).await;
    
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Deserialization error: {}", e))
}

// ============ Inventory API ============

#[derive(Serialize)]
pub struct GetItemsArgs {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Deserialize)]
pub struct InventoryItem {
    pub id: String,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub total_quantity: Option<f64>,
}

pub async fn get_all_items(page: u32, page_size: u32) -> Result<Vec<InventoryItem>, String> {
    tauri_invoke("get_all_items", &GetItemsArgs {
        page: Some(page),
        page_size: Some(page_size),
    }).await
}

// ============ Sync API ============

#[derive(Deserialize)]
pub struct SyncStatus {
    pub is_syncing: bool,
    pub last_sync_at: Option<String>,
    pub pending_changes: u64,
    pub sync_errors: u64,
}

pub async fn get_sync_status() -> Result<SyncStatus, String> {
    tauri_invoke("get_sync_status", &()).await
}

pub async fn sync_now() -> Result<SyncStatus, String> {
    tauri_invoke("sync_now", &()).await
}

// ============ Barcode API ============

#[derive(Serialize)]
pub struct ScanBarcodeArgs {
    pub image_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
pub struct BarcodeResult {
    pub text: String,
    pub format: String,
}

pub async fn scan_barcode(image_data: Vec<u8>, width: u32, height: u32) -> Result<BarcodeResult, String> {
    tauri_invoke("scan_barcode", &ScanBarcodeArgs {
        image_data,
        width,
        height,
    }).await
}

// ============ Timesheet API ============

#[derive(Serialize)]
pub struct ClockArgs {
    pub user_id: String,
    pub biometric_verified: bool,
}

#[derive(Deserialize)]
pub struct TimeEntry {
    pub id: String,
    pub clock_in_time: String,
    pub clock_out_time: Option<String>,
    pub total_hours: Option<f64>,
}

pub async fn clock_in(user_id: &str, biometric_verified: bool) -> Result<TimeEntry, String> {
    tauri_invoke("clock_in", &ClockArgs {
        user_id: user_id.to_string(),
        biometric_verified,
    }).await
}

pub async fn clock_out(user_id: &str, biometric_verified: bool) -> Result<TimeEntry, String> {
    tauri_invoke("clock_out", &ClockArgs {
        user_id: user_id.to_string(),
        biometric_verified,
    }).await
}

