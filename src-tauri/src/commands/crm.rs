//! CRM Command Handlers

use tauri::State;
use crate::AppState;
use wms_crm::{Customer, CustomerSearchQuery};

/// Get all customers with pagination
#[tauri::command]
pub async fn get_customers(
    state: State<'_, AppState>,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Result<Vec<Customer>, String> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(50);
    
    state.crm
        .get_customers(page, page_size)
        .await
        .map_err(|e| e.to_string())
}

/// Get a single customer by ID
#[tauri::command]
pub async fn get_customer(
    state: State<'_, AppState>,
    customer_id: String,
) -> Result<Option<Customer>, String> {
    state.crm
        .get_customer(&customer_id)
        .await
        .map_err(|e| e.to_string())
}

/// Create a new customer
#[tauri::command]
pub async fn create_customer(
    state: State<'_, AppState>,
    customer: Customer,
) -> Result<Customer, String> {
    state.crm
        .create_customer(customer)
        .await
        .map_err(|e| e.to_string())
}

/// Update an existing customer
#[tauri::command]
pub async fn update_customer(
    state: State<'_, AppState>,
    customer: Customer,
) -> Result<Customer, String> {
    state.crm
        .update_customer(customer)
        .await
        .map_err(|e| e.to_string())
}

/// Search customers by various criteria
#[tauri::command]
pub async fn search_customers(
    state: State<'_, AppState>,
    query: CustomerSearchQuery,
) -> Result<Vec<Customer>, String> {
    state.crm
        .search_customers(query)
        .await
        .map_err(|e| e.to_string())
}

