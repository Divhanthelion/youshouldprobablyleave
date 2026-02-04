//! Shipping Service
//! 
//! Core business logic for shipping and receiving operations.

use std::sync::Arc;
use base64::Engine;
use chrono::Utc;
use rusqlite::params;
use tracing::{info, debug};
use wms_core::db::Database;
use wms_core::error::{WmsError, Result};
use wms_core::types::new_id;
use crate::models::*;
use crate::barcode::{BarcodeDecoder, BarcodeResult};
use crate::labels::ZplLabel;

/// Shipping management service
pub struct ShippingService {
    db: Arc<Database>,
    barcode_decoder: BarcodeDecoder,
}

impl ShippingService {
    /// Create a new shipping service
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            barcode_decoder: BarcodeDecoder::new(),
        }
    }
    
    // ============ Shipment Operations ============
    
    /// Create a new shipment
    pub async fn create_shipment(&self, mut shipment: Shipment) -> Result<Shipment> {
        shipment.id = new_id();
        shipment.shipment_number = self.generate_shipment_number()?;
        shipment.status = ShipmentStatus::Draft;
        shipment.created_at = Utc::now();
        
        self.db.execute(
            "INSERT INTO shipments (
                id, shipment_number, status, order_reference, customer_id,
                carrier_id, service_type, ship_to_name, ship_to_address_line1,
                ship_to_address_line2, ship_to_city, ship_to_state,
                ship_to_postal_code, ship_to_country, ship_to_phone, ship_to_email,
                special_instructions, created_by, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &shipment.id,
                &shipment.shipment_number,
                "draft",
                &shipment.order_reference,
                &shipment.customer_id,
                &shipment.carrier_id,
                &shipment.service_type,
                &shipment.ship_to.name,
                &shipment.ship_to.address.line1,
                &shipment.ship_to.address.line2,
                &shipment.ship_to.address.city,
                &shipment.ship_to.address.state,
                &shipment.ship_to.address.postal_code,
                &shipment.ship_to.address.country,
                &shipment.ship_to.phone,
                &shipment.ship_to.email,
                &shipment.special_instructions,
                &shipment.created_by,
                shipment.created_at.to_rfc3339(),
            ],
        )?;
        
        // Insert line items
        for mut item in &mut shipment.items {
            item.id = new_id();
            item.shipment_id = shipment.id.clone();
            self.insert_shipment_item(&item)?;
        }
        
        info!("Created shipment: {}", shipment.shipment_number);
        Ok(shipment)
    }
    
    /// Get shipment by ID
    pub async fn get_shipment(&self, id: &str) -> Result<Option<Shipment>> {
        // Get shipment header
        let shipment = self.db.query_row(
            "SELECT * FROM shipments WHERE id = ?",
            params![id],
            |row| Self::row_to_shipment(row),
        )?;
        
        if let Some(mut s) = shipment {
            // Load items
            s.items = self.get_shipment_items(&s.id)?;
            // Load packages
            s.packages = self.get_shipment_packages(&s.id)?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }
    
    /// Update shipment status
    pub async fn update_status(&self, id: &str, status: ShipmentStatus) -> Result<Shipment> {
        let status_str = format!("{:?}", status).to_lowercase();
        
        let rows = self.db.execute(
            "UPDATE shipments SET status = ?, updated_at = datetime('now') WHERE id = ?",
            params![&status_str, id],
        )?;
        
        if rows == 0 {
            return Err(WmsError::not_found("Shipment not found"));
        }
        
        // Handle status-specific actions
        match status {
            ShipmentStatus::Shipped => {
                self.db.execute(
                    "UPDATE shipments SET ship_date = datetime('now') WHERE id = ?",
                    params![id],
                )?;
            }
            ShipmentStatus::Delivered => {
                self.db.execute(
                    "UPDATE shipments SET actual_delivery_date = datetime('now') WHERE id = ?",
                    params![id],
                )?;
            }
            _ => {}
        }
        
        debug!("Updated shipment {} status to {:?}", id, status);
        self.get_shipment(id).await?
            .ok_or_else(|| WmsError::not_found("Shipment not found"))
    }
    
    /// Generate shipping label
    pub async fn generate_label(&self, shipment_id: &str) -> Result<ShippingLabel> {
        let shipment = self.get_shipment(shipment_id).await?
            .ok_or_else(|| WmsError::not_found("Shipment not found"))?;
        
        // Generate ZPL label
        let zpl = ZplLabel::new()
            .set_size(4, 6) // 4" x 6" label
            .add_text(50, 50, &shipment.ship_to.name, 'A', 40)
            .add_text(50, 100, &shipment.ship_to.address.line1, 'A', 30)
            .add_text(50, 140, &format!(
                "{}, {} {}",
                shipment.ship_to.address.city,
                shipment.ship_to.address.state,
                shipment.ship_to.address.postal_code
            ), 'A', 30)
            .add_barcode_128(50, 200, &shipment.shipment_number, 80)
            .add_text(50, 300, &format!("Ship #: {}", shipment.shipment_number), 'A', 25);
        
        let zpl_data = zpl.build();
        
        // Store label
        let label = ShippingLabel {
            id: new_id(),
            shipment_id: shipment_id.to_string(),
            package_id: None,
            label_type: LabelType::Shipping,
            format: LabelFormat::Zpl,
            data: base64::engine::general_purpose::STANDARD.encode(&zpl_data),
            created_at: Utc::now(),
        };
        
        self.db.execute(
            "INSERT INTO shipping_labels (id, shipment_id, package_id, label_type, label_format, label_data, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                &label.id,
                &label.shipment_id,
                &label.package_id,
                "shipping",
                "zpl",
                zpl_data.as_bytes(),
                label.created_at.to_rfc3339(),
            ],
        )?;
        
        // Mark label as printed
        self.db.execute(
            "UPDATE shipments SET label_printed = 1 WHERE id = ?",
            params![shipment_id],
        )?;
        
        info!("Generated shipping label for {}", shipment.shipment_number);
        Ok(label)
    }
    
    // ============ Receipt Operations ============
    
    /// Create a new receipt
    pub async fn create_receipt(&self, mut receipt: Receipt) -> Result<Receipt> {
        receipt.id = new_id();
        receipt.receipt_number = self.generate_receipt_number()?;
        receipt.status = ReceiptStatus::Pending;
        receipt.created_at = Utc::now();
        
        self.db.execute(
            "INSERT INTO receipts (
                id, receipt_number, status, po_number, supplier_name,
                supplier_reference, expected_date, dock_door, notes,
                created_by, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &receipt.id,
                &receipt.receipt_number,
                "pending",
                &receipt.po_number,
                &receipt.supplier_name,
                &receipt.supplier_reference,
                receipt.expected_date.map(|d| d.to_rfc3339()),
                &receipt.dock_door,
                &receipt.notes,
                &receipt.created_by,
                receipt.created_at.to_rfc3339(),
            ],
        )?;
        
        // Insert line items
        for mut item in &mut receipt.items {
            item.id = new_id();
            item.receipt_id = receipt.id.clone();
            self.insert_receipt_item(&item)?;
        }
        
        info!("Created receipt: {}", receipt.receipt_number);
        Ok(receipt)
    }
    
    /// Process a receipt item (scan and receive)
    pub async fn process_receipt_item(&self, receipt_id: &str, mut item: ReceiptItem) -> Result<Receipt> {
        // Update receipt status to receiving
        self.db.execute(
            "UPDATE receipts SET status = 'receiving', received_date = COALESCE(received_date, datetime('now'))
             WHERE id = ?",
            params![receipt_id],
        )?;
        
        // Update item quantities
        item.status = if item.quantity_received >= item.quantity_expected {
            ReceiptItemStatus::Complete
        } else if item.quantity_received > 0.0 {
            ReceiptItemStatus::Partial
        } else {
            ReceiptItemStatus::Pending
        };
        
        item.received_at = Some(Utc::now());
        
        self.db.execute(
            "UPDATE receipt_items SET
                quantity_received = ?, quantity_damaged = ?, lot_number = ?,
                expiry_date = ?, status = ?, received_by = ?, received_at = ?, notes = ?
             WHERE id = ?",
            params![
                item.quantity_received,
                item.quantity_damaged,
                &item.lot_number,
                item.expiry_date.map(|d| d.to_rfc3339()),
                format!("{:?}", item.status).to_lowercase(),
                &item.received_by,
                item.received_at.map(|d| d.to_rfc3339()),
                &item.notes,
                &item.id,
            ],
        )?;
        
        debug!("Processed receipt item: {} received {}", item.id, item.quantity_received);
        
        // Return updated receipt
        self.get_receipt(receipt_id).await?
            .ok_or_else(|| WmsError::not_found("Receipt not found"))
    }
    
    /// Complete a receipt and update inventory
    pub async fn complete_receipt(&self, receipt_id: &str) -> Result<Receipt> {
        let receipt = self.get_receipt(receipt_id).await?
            .ok_or_else(|| WmsError::not_found("Receipt not found"))?;
        
        // Verify all items are received
        for item in &receipt.items {
            if item.status == ReceiptItemStatus::Pending {
                return Err(WmsError::validation("Not all items have been received"));
            }
        }
        
        // Update receipt status
        self.db.execute(
            "UPDATE receipts SET status = 'completed', completed_at = datetime('now')
             WHERE id = ?",
            params![receipt_id],
        )?;
        
        info!("Completed receipt: {}", receipt.receipt_number);
        self.get_receipt(receipt_id).await?
            .ok_or_else(|| WmsError::not_found("Receipt not found"))
    }
    
    /// Get receipt by ID
    async fn get_receipt(&self, id: &str) -> Result<Option<Receipt>> {
        let receipt = self.db.query_row(
            "SELECT * FROM receipts WHERE id = ?",
            params![id],
            |row| Self::row_to_receipt(row),
        )?;
        
        if let Some(mut r) = receipt {
            r.items = self.get_receipt_items(&r.id)?;
            Ok(Some(r))
        } else {
            Ok(None)
        }
    }
    
    // ============ Barcode Operations ============
    
    /// Decode a barcode from image data
    pub async fn decode_barcode(&self, image_data: &[u8], width: u32, height: u32) -> Result<BarcodeResult> {
        self.barcode_decoder.decode(image_data, width, height)
    }
    
    // ============ Helper Methods ============
    
    fn generate_shipment_number(&self) -> Result<String> {
        let count: i64 = self.db.query_row(
            "SELECT COUNT(*) + 1 FROM shipments",
            [],
            |row| row.get(0),
        )?.unwrap_or(1);
        
        Ok(format!("SHP-{:08}", count))
    }
    
    fn generate_receipt_number(&self) -> Result<String> {
        let count: i64 = self.db.query_row(
            "SELECT COUNT(*) + 1 FROM receipts",
            [],
            |row| row.get(0),
        )?.unwrap_or(1);
        
        Ok(format!("RCV-{:08}", count))
    }
    
    fn insert_shipment_item(&self, item: &ShipmentItem) -> Result<()> {
        self.db.execute(
            "INSERT INTO shipment_items (
                id, shipment_id, item_id, location_id, quantity_ordered,
                quantity_picked, quantity_shipped, lot_number, serial_number, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &item.id,
                &item.shipment_id,
                &item.item_id,
                &item.location_id,
                item.quantity_ordered,
                item.quantity_picked,
                item.quantity_shipped,
                &item.lot_number,
                &item.serial_number,
                "pending",
            ],
        )?;
        Ok(())
    }
    
    fn insert_receipt_item(&self, item: &ReceiptItem) -> Result<()> {
        self.db.execute(
            "INSERT INTO receipt_items (
                id, receipt_id, item_id, location_id, quantity_expected,
                quantity_received, quantity_damaged, lot_number, expiry_date, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &item.id,
                &item.receipt_id,
                &item.item_id,
                &item.location_id,
                item.quantity_expected,
                item.quantity_received,
                item.quantity_damaged,
                &item.lot_number,
                item.expiry_date.map(|d| d.to_rfc3339()),
                "pending",
            ],
        )?;
        Ok(())
    }
    
    fn get_shipment_items(&self, shipment_id: &str) -> Result<Vec<ShipmentItem>> {
        self.db.query_map(
            "SELECT si.*, i.sku, i.name 
             FROM shipment_items si
             LEFT JOIN inventory_items i ON si.item_id = i.id
             WHERE si.shipment_id = ?",
            params![shipment_id],
            |row| {
                Ok(ShipmentItem {
                    id: row.get("id")?,
                    shipment_id: row.get("shipment_id")?,
                    item_id: row.get("item_id")?,
                    location_id: row.get("location_id")?,
                    quantity_ordered: row.get("quantity_ordered")?,
                    quantity_picked: row.get("quantity_picked")?,
                    quantity_shipped: row.get("quantity_shipped")?,
                    lot_number: row.get("lot_number")?,
                    serial_number: row.get("serial_number")?,
                    status: ShipmentItemStatus::Pending,
                    picked_by: row.get("picked_by")?,
                    picked_at: None,
                    item_sku: row.get("sku")?,
                    item_name: row.get("name")?,
                })
            },
        )
    }
    
    fn get_shipment_packages(&self, shipment_id: &str) -> Result<Vec<ShipmentPackage>> {
        self.db.query_map(
            "SELECT * FROM shipment_packages WHERE shipment_id = ? ORDER BY package_number",
            params![shipment_id],
            |row| {
                Ok(ShipmentPackage {
                    id: row.get("id")?,
                    shipment_id: row.get("shipment_id")?,
                    package_number: row.get("package_number")?,
                    tracking_number: row.get("tracking_number")?,
                    weight_kg: row.get("weight_kg")?,
                    length_cm: row.get("length_cm")?,
                    width_cm: row.get("width_cm")?,
                    height_cm: row.get("height_cm")?,
                    package_type: row.get("package_type")?,
                    created_at: Utc::now(),
                })
            },
        )
    }
    
    fn get_receipt_items(&self, receipt_id: &str) -> Result<Vec<ReceiptItem>> {
        self.db.query_map(
            "SELECT ri.*, i.sku, i.name
             FROM receipt_items ri
             LEFT JOIN inventory_items i ON ri.item_id = i.id
             WHERE ri.receipt_id = ?",
            params![receipt_id],
            |row| {
                Ok(ReceiptItem {
                    id: row.get("id")?,
                    receipt_id: row.get("receipt_id")?,
                    item_id: row.get("item_id")?,
                    location_id: row.get("location_id")?,
                    quantity_expected: row.get("quantity_expected")?,
                    quantity_received: row.get("quantity_received")?,
                    quantity_damaged: row.get("quantity_damaged")?,
                    lot_number: row.get("lot_number")?,
                    expiry_date: None,
                    status: ReceiptItemStatus::Pending,
                    received_by: row.get("received_by")?,
                    received_at: None,
                    notes: row.get("notes")?,
                    item_sku: row.get("sku")?,
                    item_name: row.get("name")?,
                })
            },
        )
    }
    
    fn row_to_shipment(row: &rusqlite::Row) -> rusqlite::Result<Shipment> {
        Ok(Shipment {
            id: row.get("id")?,
            shipment_number: row.get("shipment_number")?,
            status: ShipmentStatus::Draft,
            order_reference: row.get("order_reference")?,
            customer_id: row.get("customer_id")?,
            carrier_id: row.get("carrier_id")?,
            service_type: row.get("service_type")?,
            tracking_number: row.get("tracking_number")?,
            ship_date: None,
            expected_delivery_date: None,
            actual_delivery_date: None,
            ship_to: ShipToAddress {
                name: row.get("ship_to_name")?,
                address: wms_core::types::Address {
                    line1: row.get("ship_to_address_line1")?,
                    line2: row.get("ship_to_address_line2")?,
                    city: row.get("ship_to_city")?,
                    state: row.get("ship_to_state")?,
                    postal_code: row.get("ship_to_postal_code")?,
                    country: row.get("ship_to_country")?,
                },
                phone: row.get("ship_to_phone")?,
                email: row.get("ship_to_email")?,
            },
            total_weight_kg: row.get("total_weight_kg")?,
            total_packages: row.get::<_, u32>("total_packages").unwrap_or(1),
            shipping_cost: row.get("shipping_cost")?,
            insurance_value: row.get("insurance_value")?,
            special_instructions: row.get("special_instructions")?,
            label_printed: row.get::<_, i32>("label_printed")? == 1,
            created_by: row.get("created_by")?,
            created_at: Utc::now(),
            updated_at: None,
            items: Vec::new(),
            packages: Vec::new(),
        })
    }
    
    fn row_to_receipt(row: &rusqlite::Row) -> rusqlite::Result<Receipt> {
        Ok(Receipt {
            id: row.get("id")?,
            receipt_number: row.get("receipt_number")?,
            status: ReceiptStatus::Pending,
            po_number: row.get("po_number")?,
            supplier_name: row.get("supplier_name")?,
            supplier_reference: row.get("supplier_reference")?,
            expected_date: None,
            received_date: None,
            dock_door: row.get("dock_door")?,
            notes: row.get("notes")?,
            created_by: row.get("created_by")?,
            created_at: Utc::now(),
            completed_at: None,
            completed_by: row.get("completed_by")?,
            items: Vec::new(),
        })
    }
}

