//! WMS Application Entry Point
//! 
//! This is the main entry point for the desktop application.
//! Mobile platforms use the lib.rs mobile_entry_point instead.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    wms_app_lib::run()
}

