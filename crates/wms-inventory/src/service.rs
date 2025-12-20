//! Inventory Service
//! 
//! Core business logic for inventory management operations.

use std::sync::Arc;
use chrono::Utc;
use rusqlite::params;
use tracing::{info, debug};
use wms_core::db::Database;
use wms_core::error::{WmsError, Result};
use wms_core::types::new_id;
use crate::models::*;
use crate::forecast::{ForecastEngine, ForecastResult};

/// Inventory management service
pub struct InventoryService {
    db: Arc<Database>,
    forecast_engine: ForecastEngine,
}

impl InventoryService {
    /// Create a new inventory service
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            forecast_engine: ForecastEngine::new(),
        }
    }
    
    /// Get all inventory items with pagination
    pub async fn get_all_items(&self, page: u32, page_size: u32) -> Result<Vec<InventoryItem>> {
        let offset = (page.saturating_sub(1)) * page_size;
        
        let items = self.db.query_map(
            "SELECT i.*, COALESCE(SUM(s.quantity), 0) as total_qty
             FROM inventory_items i
             LEFT JOIN inventory_stock s ON i.id = s.item_id
             WHERE i.is_active = 1
             GROUP BY i.id
             ORDER BY i.sku
             LIMIT ? OFFSET ?",
            params![page_size, offset],
            |row| Self::row_to_item(row),
        )?;
        
        Ok(items)
    }
    
    /// Get item by SKU
    pub async fn get_item_by_sku(&self, sku: &str) -> Result<Option<InventoryItem>> {
        self.db.query_row(
            "SELECT i.*, COALESCE(SUM(s.quantity), 0) as total_qty
             FROM inventory_items i
             LEFT JOIN inventory_stock s ON i.id = s.item_id
             WHERE i.sku = ?
             GROUP BY i.id",
            params![sku],
            |row| Self::row_to_item(row),
        )
    }
    
    /// Get item by ID
    pub async fn get_item_by_id(&self, id: &str) -> Result<Option<InventoryItem>> {
        self.db.query_row(
            "SELECT i.*, COALESCE(SUM(s.quantity), 0) as total_qty
             FROM inventory_items i
             LEFT JOIN inventory_stock s ON i.id = s.item_id
             WHERE i.id = ?
             GROUP BY i.id",
            params![id],
            |row| Self::row_to_item(row),
        )
    }
    
    /// Create a new inventory item
    pub async fn create_item(&self, mut item: InventoryItem) -> Result<InventoryItem> {
        // Validate SKU uniqueness
        let existing = self.get_item_by_sku(&item.sku).await?;
        if existing.is_some() {
            return Err(WmsError::conflict(format!("SKU {} already exists", item.sku)));
        }
        
        item.id = new_id();
        item.created_at = Utc::now();
        
        self.db.execute(
            "INSERT INTO inventory_items (
                id, sku, name, description, category, subcategory,
                unit_of_measure, weight_kg, length_cm, width_cm, height_cm,
                barcode, barcode_type, min_stock_level, max_stock_level,
                reorder_point, reorder_quantity, lead_time_days, abc_class,
                is_active, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &item.id,
                &item.sku,
                &item.name,
                &item.description,
                &item.category,
                &item.subcategory,
                format!("{:?}", item.unit_of_measure).to_lowercase(),
                &item.weight_kg,
                item.dimensions.as_ref().map(|d| d.length_cm),
                item.dimensions.as_ref().map(|d| d.width_cm),
                item.dimensions.as_ref().map(|d| d.height_cm),
                &item.barcode,
                item.barcode_type.map(|t| format!("{:?}", t)),
                &item.min_stock_level,
                &item.max_stock_level,
                &item.reorder_point,
                &item.reorder_quantity,
                &item.lead_time_days,
                item.abc_class.map(|c| format!("{:?}", c)),
                &item.is_active,
                item.created_at.to_rfc3339(),
            ],
        )?;
        
        info!("Created inventory item: {} - {}", item.sku, item.name);
        Ok(item)
    }
    
    /// Update an existing inventory item
    pub async fn update_item(&self, mut item: InventoryItem) -> Result<InventoryItem> {
        item.updated_at = Some(Utc::now());
        
        let rows = self.db.execute(
            "UPDATE inventory_items SET
                sku = ?, name = ?, description = ?, category = ?, subcategory = ?,
                unit_of_measure = ?, weight_kg = ?, length_cm = ?, width_cm = ?, height_cm = ?,
                barcode = ?, barcode_type = ?, min_stock_level = ?, max_stock_level = ?,
                reorder_point = ?, reorder_quantity = ?, lead_time_days = ?, abc_class = ?,
                is_active = ?, updated_at = ?
             WHERE id = ?",
            params![
                &item.sku,
                &item.name,
                &item.description,
                &item.category,
                &item.subcategory,
                format!("{:?}", item.unit_of_measure).to_lowercase(),
                &item.weight_kg,
                item.dimensions.as_ref().map(|d| d.length_cm),
                item.dimensions.as_ref().map(|d| d.width_cm),
                item.dimensions.as_ref().map(|d| d.height_cm),
                &item.barcode,
                item.barcode_type.map(|t| format!("{:?}", t)),
                &item.min_stock_level,
                &item.max_stock_level,
                &item.reorder_point,
                &item.reorder_quantity,
                &item.lead_time_days,
                item.abc_class.map(|c| format!("{:?}", c)),
                &item.is_active,
                item.updated_at.map(|t| t.to_rfc3339()),
                &item.id,
            ],
        )?;
        
        if rows == 0 {
            return Err(WmsError::not_found(format!("Item {} not found", item.id)));
        }
        
        debug!("Updated inventory item: {}", item.sku);
        Ok(item)
    }
    
    /// Adjust inventory quantity
    pub async fn adjust_quantity(&self, adjustment: InventoryAdjustment) -> Result<InventoryItem> {
        // Get current stock level
        let item = self.get_item_by_id(&adjustment.item_id).await?
            .ok_or_else(|| WmsError::not_found("Item not found"))?;
        
        let current_qty = item.total_quantity.unwrap_or(0.0);
        let delta = adjustment.quantity * adjustment.adjustment_type.sign();
        let new_qty = current_qty + delta;
        
        // Check for negative inventory (unless allowed)
        if new_qty < 0.0 {
            return Err(WmsError::validation(format!(
                "Insufficient inventory. Current: {}, Requested: {}",
                current_qty, adjustment.quantity
            )));
        }
        
        // Create transaction record
        let tx_id = new_id();
        self.db.execute(
            "INSERT INTO inventory_transactions (
                id, item_id, location_id, transaction_type, quantity,
                previous_quantity, new_quantity, lot_number, reason_code,
                notes, user_id, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))",
            params![
                &tx_id,
                &adjustment.item_id,
                &adjustment.location_id,
                format!("{:?}", adjustment.adjustment_type).to_uppercase(),
                &adjustment.quantity,
                &current_qty,
                &new_qty,
                &adjustment.lot_number,
                &adjustment.reason_code,
                &adjustment.notes,
                &adjustment.user_id,
            ],
        )?;
        
        // Update or insert stock record
        if let Some(location_id) = &adjustment.location_id {
            self.db.execute(
                "INSERT INTO inventory_stock (id, item_id, location_id, quantity, lot_number, updated_at)
                 VALUES (?, ?, ?, ?, ?, datetime('now'))
                 ON CONFLICT(item_id, location_id, lot_number) DO UPDATE SET
                    quantity = quantity + ?,
                    updated_at = datetime('now')",
                params![
                    new_id(),
                    &adjustment.item_id,
                    location_id,
                    delta,
                    &adjustment.lot_number.clone().unwrap_or_default(),
                    delta,
                ],
            )?;
        }
        
        info!(
            "Adjusted inventory: {} {} {} units (user: {})",
            item.sku,
            format!("{:?}", adjustment.adjustment_type),
            adjustment.quantity,
            adjustment.user_id
        );
        
        // Return updated item
        self.get_item_by_id(&adjustment.item_id).await?
            .ok_or_else(|| WmsError::not_found("Item not found"))
    }
    
    /// Get items below their reorder point
    pub async fn get_low_stock_items(&self) -> Result<Vec<InventoryItem>> {
        let items = self.db.query_map(
            "SELECT i.*, COALESCE(SUM(s.quantity), 0) as total_qty
             FROM inventory_items i
             LEFT JOIN inventory_stock s ON i.id = s.item_id
             WHERE i.is_active = 1
               AND i.reorder_point IS NOT NULL
             GROUP BY i.id
             HAVING total_qty <= i.reorder_point
             ORDER BY (i.reorder_point - total_qty) DESC",
            [],
            |row| Self::row_to_item(row),
        )?;
        
        Ok(items)
    }
    
    /// Run demand forecast for an item
    pub async fn run_forecast(&self, sku: &str, days_ahead: u32) -> Result<ForecastResult> {
        // Get historical transaction data
        let history = self.get_transaction_history(sku, 365).await?;
        
        if history.len() < 30 {
            return Err(WmsError::Forecast(
                "Insufficient history for forecasting (need at least 30 data points)".to_string()
            ));
        }
        
        // Run forecast
        self.forecast_engine.forecast(&history, days_ahead)
    }
    
    /// Get transaction history for forecasting
    async fn get_transaction_history(&self, sku: &str, days: u32) -> Result<Vec<f64>> {
        let transactions: Vec<f64> = self.db.query_map(
            "SELECT ABS(t.quantity) as qty
             FROM inventory_transactions t
             JOIN inventory_items i ON t.item_id = i.id
             WHERE i.sku = ?
               AND t.transaction_type IN ('PICK', 'RECEIVE')
               AND t.created_at >= date('now', '-' || ? || ' days')
             ORDER BY t.created_at ASC",
            params![sku, days],
            |row| row.get(0),
        )?;
        
        Ok(transactions)
    }
    
    /// Convert database row to InventoryItem
    fn row_to_item(row: &rusqlite::Row) -> rusqlite::Result<InventoryItem> {
        let dimensions = match (
            row.get::<_, Option<f64>>("length_cm")?,
            row.get::<_, Option<f64>>("width_cm")?,
            row.get::<_, Option<f64>>("height_cm")?,
        ) {
            (Some(l), Some(w), Some(h)) => Some(ItemDimensions {
                length_cm: l,
                width_cm: w,
                height_cm: h,
            }),
            _ => None,
        };
        
        Ok(InventoryItem {
            id: row.get("id")?,
            sku: row.get("sku")?,
            name: row.get("name")?,
            description: row.get("description")?,
            category: row.get("category")?,
            subcategory: row.get("subcategory")?,
            unit_of_measure: wms_core::types::UnitOfMeasure::Each, // Parse from string if needed
            weight_kg: row.get("weight_kg")?,
            dimensions,
            barcode: row.get("barcode")?,
            barcode_type: None, // Parse from string if needed
            min_stock_level: row.get::<_, f64>("min_stock_level").unwrap_or(0.0),
            max_stock_level: row.get("max_stock_level")?,
            reorder_point: row.get("reorder_point")?,
            reorder_quantity: row.get("reorder_quantity")?,
            lead_time_days: row.get::<_, u32>("lead_time_days").unwrap_or(0),
            abc_class: None, // Parse from string if needed
            is_active: row.get::<_, i32>("is_active")? == 1,
            created_at: chrono::Utc::now(), // Parse from string
            updated_at: None,
            total_quantity: row.get("total_qty").ok(),
        })
    }
}

