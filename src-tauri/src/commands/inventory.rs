//! Inventory Command Handlers

use tauri::State;
use crate::AppState;
use wms_inventory::{InventoryItem, InventoryAdjustment, ForecastResult};

/// Get all inventory items with optional pagination
#[tauri::command]
pub async fn get_all_items(
    state: State<'_, AppState>,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<InventoryItem>, String> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(50);
    
    state.inventory
        .get_all_items(page, page_size)
        .await
        .map_err(|e| e.to_string())
}

/// Get a single inventory item by SKU
#[tauri::command]
pub async fn get_item_by_sku(
    state: State<'_, AppState>,
    sku: String,
) -> Result<Option<InventoryItem>, String> {
    state.inventory
        .get_item_by_sku(&sku)
        .await
        .map_err(|e| e.to_string())
}

/// Create a new inventory item
#[tauri::command]
pub async fn create_item(
    state: State<'_, AppState>,
    item: InventoryItem,
) -> Result<InventoryItem, String> {
    state.inventory
        .create_item(item)
        .await
        .map_err(|e| e.to_string())
}

/// Update an existing inventory item
#[tauri::command]
pub async fn update_item(
    state: State<'_, AppState>,
    item: InventoryItem,
) -> Result<InventoryItem, String> {
    state.inventory
        .update_item(item)
        .await
        .map_err(|e| e.to_string())
}

/// Adjust inventory quantity (pick, receive, count, etc.)
#[tauri::command]
pub async fn adjust_quantity(
    state: State<'_, AppState>,
    adjustment: InventoryAdjustment,
) -> Result<InventoryItem, String> {
    state.inventory
        .adjust_quantity(adjustment)
        .await
        .map_err(|e| e.to_string())
}

/// Get items below their reorder point
#[tauri::command]
pub async fn get_low_stock_items(
    state: State<'_, AppState>,
) -> Result<Vec<InventoryItem>, String> {
    state.inventory
        .get_low_stock_items()
        .await
        .map_err(|e| e.to_string())
}

/// Run demand forecasting for an item
#[tauri::command]
pub async fn run_forecast(
    state: State<'_, AppState>,
    sku: String,
    days_ahead: u32,
) -> Result<ForecastResult, String> {
    state.inventory
        .run_forecast(&sku, days_ahead)
        .await
        .map_err(|e| e.to_string())
}

