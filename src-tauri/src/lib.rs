//! WMS Application - Tauri Backend
//! 
//! This module provides the Tauri command handlers and plugin initialization
//! for the Warehouse Management System.

use tauri::Manager;
use tracing::info;

mod commands;
mod state;

pub use state::AppState;

/// Initialize and run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting Warehouse Management System");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Initialize application state
            let app_state = AppState::new(app.handle())?;
            app.manage(app_state);
            
            info!("Application state initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Inventory commands
            commands::inventory::get_all_items,
            commands::inventory::get_item_by_sku,
            commands::inventory::create_item,
            commands::inventory::update_item,
            commands::inventory::adjust_quantity,
            commands::inventory::get_low_stock_items,
            commands::inventory::run_forecast,
            // Shipping commands
            commands::shipping::create_shipment,
            commands::shipping::get_shipment,
            commands::shipping::update_shipment_status,
            commands::shipping::generate_shipping_label,
            commands::shipping::scan_barcode,
            // Receiving commands
            commands::receiving::create_receipt,
            commands::receiving::process_receipt_item,
            commands::receiving::complete_receipt,
            // Delivery commands
            commands::deliveries::get_deliveries,
            commands::deliveries::create_delivery,
            commands::deliveries::update_delivery_status,
            commands::deliveries::optimize_route,
            commands::deliveries::check_geofence,
            // CRM commands
            commands::crm::get_customers,
            commands::crm::get_customer,
            commands::crm::create_customer,
            commands::crm::update_customer,
            commands::crm::search_customers,
            // Timesheet commands
            commands::timesheets::clock_in,
            commands::timesheets::clock_out,
            commands::timesheets::get_timesheet,
            commands::timesheets::export_timesheet,
            // Sync commands
            commands::sync::sync_now,
            commands::sync::get_sync_status,
            commands::sync::set_offline_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

