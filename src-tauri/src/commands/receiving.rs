//! Receiving Command Handlers

use tauri::State;
use crate::AppState;
use wms_shipping::{Receipt, ReceiptItem, ReceiptStatus};

/// Create a new receipt for incoming goods
#[tauri::command]
pub async fn create_receipt(
    state: State<'_, AppState>,
    receipt: Receipt,
) -> Result<Receipt, String> {
    state.shipping
        .create_receipt(receipt)
        .await
        .map_err(|e| e.to_string())
}

/// Process a single item in a receipt (scan and verify)
#[tauri::command]
pub async fn process_receipt_item(
    state: State<'_, AppState>,
    receipt_id: String,
    item: ReceiptItem,
) -> Result<Receipt, String> {
    state.shipping
        .process_receipt_item(&receipt_id, item)
        .await
        .map_err(|e| e.to_string())
}

/// Complete a receipt and update inventory
#[tauri::command]
pub async fn complete_receipt(
    state: State<'_, AppState>,
    receipt_id: String,
) -> Result<Receipt, String> {
    state.shipping
        .complete_receipt(&receipt_id)
        .await
        .map_err(|e| e.to_string())
}

