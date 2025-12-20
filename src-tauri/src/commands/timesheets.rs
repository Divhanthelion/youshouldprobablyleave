//! Timesheet Command Handlers

use tauri::State;
use crate::AppState;
use wms_timesheets::{TimeEntry, Timesheet, TimesheetExport};

/// Clock in for the current user
#[tauri::command]
pub async fn clock_in(
    state: State<'_, AppState>,
    user_id: String,
    biometric_verified: bool,
) -> Result<TimeEntry, String> {
    if !biometric_verified {
        return Err("Biometric verification required for clock in".to_string());
    }
    
    state.timesheets
        .clock_in(&user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Clock out for the current user
#[tauri::command]
pub async fn clock_out(
    state: State<'_, AppState>,
    user_id: String,
    biometric_verified: bool,
) -> Result<TimeEntry, String> {
    if !biometric_verified {
        return Err("Biometric verification required for clock out".to_string());
    }
    
    state.timesheets
        .clock_out(&user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get timesheet for a user within a date range
#[tauri::command]
pub async fn get_timesheet(
    state: State<'_, AppState>,
    user_id: String,
    start_date: String,
    end_date: String,
) -> Result<Timesheet, String> {
    state.timesheets
        .get_timesheet(&user_id, &start_date, &end_date)
        .await
        .map_err(|e| e.to_string())
}

/// Export timesheet data to Excel/CSV format
#[tauri::command]
pub async fn export_timesheet(
    state: State<'_, AppState>,
    user_id: String,
    start_date: String,
    end_date: String,
    format: String,
) -> Result<TimesheetExport, String> {
    state.timesheets
        .export_timesheet(&user_id, &start_date, &end_date, &format)
        .await
        .map_err(|e| e.to_string())
}

