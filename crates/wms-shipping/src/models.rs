//! Shipping Data Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use wms_core::types::Address;

/// Outbound shipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipment {
    pub id: String,
    pub shipment_number: String,
    pub status: ShipmentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_delivery_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_delivery_date: Option<DateTime<Utc>>,
    pub ship_to: ShipToAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight_kg: Option<f64>,
    #[serde(default = "default_one")]
    pub total_packages: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_instructions: Option<String>,
    #[serde(default)]
    pub label_printed: bool,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// Line items in this shipment
    #[serde(default)]
    pub items: Vec<ShipmentItem>,
    /// Packages/cartons
    #[serde(default)]
    pub packages: Vec<ShipmentPackage>,
}

fn default_one() -> u32 {
    1
}

/// Shipment status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ShipmentStatus {
    Draft,
    Confirmed,
    Picking,
    Packed,
    Shipped,
    Delivered,
    Cancelled,
}

impl Default for ShipmentStatus {
    fn default() -> Self {
        Self::Draft
    }
}

/// Ship-to address with contact info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipToAddress {
    pub name: String,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Shipment line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentItem {
    pub id: String,
    pub shipment_id: String,
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    pub quantity_ordered: f64,
    #[serde(default)]
    pub quantity_picked: f64,
    #[serde(default)]
    pub quantity_shipped: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    pub status: ShipmentItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picked_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picked_at: Option<DateTime<Utc>>,
    /// Item details (populated on read)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
}

/// Shipment item status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ShipmentItemStatus {
    Pending,
    Picking,
    Picked,
    Packed,
    Shipped,
}

impl Default for ShipmentItemStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Package/carton in a shipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentPackage {
    pub id: String,
    pub shipment_id: String,
    pub package_number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_cm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Shipping carrier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Carrier {
    pub id: String,
    pub code: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url_template: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Inbound receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub id: String,
    pub receipt_number: String,
    pub status: ReceiptStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub po_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dock_door: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by: Option<String>,
    /// Line items
    #[serde(default)]
    pub items: Vec<ReceiptItem>,
}

/// Receipt status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReceiptStatus {
    Pending,
    Receiving,
    Completed,
    Cancelled,
}

impl Default for ReceiptStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Receipt line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptItem {
    pub id: String,
    pub receipt_id: String,
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    pub quantity_expected: f64,
    #[serde(default)]
    pub quantity_received: f64,
    #[serde(default)]
    pub quantity_damaged: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: ReceiptItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Item details (populated on read)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
}

/// Receipt item status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReceiptItemStatus {
    Pending,
    Partial,
    Complete,
    Damaged,
}

impl Default for ReceiptItemStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Generated shipping label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingLabel {
    pub id: String,
    pub shipment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    pub label_type: LabelType,
    pub format: LabelFormat,
    /// Base64 encoded label data
    pub data: String,
    pub created_at: DateTime<Utc>,
}

/// Label types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LabelType {
    Shipping,
    Pallet,
    Carton,
    Return,
}

/// Label output formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LabelFormat {
    Zpl,
    Pdf,
    Png,
}

