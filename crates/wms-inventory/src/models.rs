//! Inventory Data Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use wms_core::types::UnitOfMeasure;

/// Inventory item (product/SKU)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: String,
    pub sku: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    #[serde(default)]
    pub unit_of_measure: UnitOfMeasure,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ItemDimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode_type: Option<BarcodeType>,
    #[serde(default)]
    pub min_stock_level: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stock_level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reorder_point: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reorder_quantity: Option<f64>,
    #[serde(default)]
    pub lead_time_days: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abc_class: Option<AbcClass>,
    #[serde(default = "default_true")]
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// Computed field: total quantity across all locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<f64>,
}

fn default_true() -> bool {
    true
}

/// Item physical dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDimensions {
    pub length_cm: f64,
    pub width_cm: f64,
    pub height_cm: f64,
}

impl ItemDimensions {
    pub fn volume_m3(&self) -> f64 {
        (self.length_cm * self.width_cm * self.height_cm) / 1_000_000.0
    }
}

/// Barcode format types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BarcodeType {
    Ean13,
    Ean8,
    Upc,
    Code128,
    Code39,
    Qr,
    Pdf417,
    DataMatrix,
}

/// ABC inventory classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbcClass {
    A, // High value, tight control
    B, // Medium value, moderate control  
    C, // Low value, loose control
}

/// Warehouse location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub code: String,
    pub zone: LocationZone,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aisle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<f64>,
    #[serde(default)]
    pub current_units: f64,
    pub created_at: DateTime<Utc>,
}

impl Location {
    /// Check if location has available capacity
    pub fn has_capacity(&self, units: f64) -> bool {
        match self.capacity_units {
            Some(cap) => self.current_units + units <= cap,
            None => true, // No limit
        }
    }
}

/// Location zone types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum LocationZone {
    Receiving,
    Storage,
    Picking,
    Shipping,
    Staging,
    Quarantine,
    Returns,
}

/// Inventory stock level at a specific location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryStock {
    pub id: String,
    pub item_id: String,
    pub location_id: String,
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_unit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_count_date: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    /// Computed: item details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<InventoryItem>>,
    /// Computed: location details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
}

/// Inventory adjustment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAdjustment {
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    pub adjustment_type: AdjustmentType,
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub user_id: String,
}

/// Types of inventory adjustments
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AdjustmentType {
    Receive,   // Incoming goods
    Pick,      // Outbound pick (negative)
    Adjust,    // Manual adjustment
    Transfer,  // Move between locations
    Count,     // Cycle count correction
    Damage,    // Damaged goods (negative)
    Return,    // Customer return (positive)
    Scrap,     // Scrap/dispose (negative)
}

impl AdjustmentType {
    /// Get the sign multiplier for this adjustment type
    pub fn sign(&self) -> f64 {
        match self {
            Self::Receive | Self::Return => 1.0,
            Self::Pick | Self::Damage | Self::Scrap => -1.0,
            Self::Adjust | Self::Transfer | Self::Count => 1.0, // Uses actual delta
        }
    }
}

/// Inventory transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryTransaction {
    pub id: String,
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    pub transaction_type: AdjustmentType,
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

/// Stock level summary for an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSummary {
    pub item_id: String,
    pub sku: String,
    pub name: String,
    pub total_quantity: f64,
    pub available_quantity: f64,
    pub reserved_quantity: f64,
    pub reorder_point: Option<f64>,
    pub reorder_quantity: Option<f64>,
    pub is_below_minimum: bool,
    pub is_below_reorder: bool,
    pub locations: Vec<LocationStock>,
}

/// Stock at a single location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationStock {
    pub location_id: String,
    pub location_code: String,
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
}

