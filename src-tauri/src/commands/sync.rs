//! Sync Command Handlers

use tauri::State;
use crate::AppState;
use wms_sync::SyncStatus;

/// Trigger a manual synchronization with the server
#[tauri::command]
pub async fn sync_now(
    state: State<'_, AppState>,
) -> Result<SyncStatus, String> {
    let offline = *state.offline_mode.read().await;
    if offline {
        return Err("Cannot sync while in offline mode".to_string());
    }
    
    let mut sync_engine = state.sync_engine.write().await;
    sync_engine
        .sync_now()
        .await
        .map_err(|e| e.to_string())
}

/// Get the current synchronization status
#[tauri::command]
pub async fn get_sync_status(
    state: State<'_, AppState>,
) -> Result<SyncStatus, String> {
    let sync_engine = state.sync_engine.read().await;
    Ok(sync_engine.get_status())
}

/// Enable or disable offline mode
#[tauri::command]
pub async fn set_offline_mode(
    state: State<'_, AppState>,
    offline: bool,
) -> Result<bool, String> {
    let mut mode = state.offline_mode.write().await;
    *mode = offline;
    Ok(*mode)
}

