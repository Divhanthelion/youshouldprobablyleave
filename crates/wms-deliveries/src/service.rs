//! Delivery Service
//! 
//! Core business logic for delivery and logistics operations.

use std::sync::Arc;
use chrono::Utc;
use rusqlite::params;
use tracing::{info, debug};
use wms_core::db::Database;
use wms_core::error::{WmsError, Result};
use wms_core::types::new_id;
use crate::models::*;
use crate::routing::{RouteOptimizer, OptimizedRoute};
use crate::geofence::{GeofenceChecker, GeofenceResult};

/// Delivery management service
pub struct DeliveryService {
    db: Arc<Database>,
    route_optimizer: RouteOptimizer,
    geofence_checker: GeofenceChecker,
}

impl DeliveryService {
    /// Create a new delivery service
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            route_optimizer: RouteOptimizer::new(),
            geofence_checker: GeofenceChecker::new(),
        }
    }
    
    /// Get deliveries with optional filters
    pub async fn get_deliveries(
        &self,
        status: Option<DeliveryStatus>,
        date: Option<&str>,
    ) -> Result<Vec<Delivery>> {
        let mut sql = String::from(
            "SELECT * FROM deliveries WHERE 1=1"
        );
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(s) = status {
            sql.push_str(" AND status = ?");
            params_vec.push(Box::new(format!("{:?}", s).to_lowercase()));
        }
        
        if let Some(d) = date {
            sql.push_str(" AND date(scheduled_date) = date(?)");
            params_vec.push(Box::new(d.to_string()));
        }
        
        sql.push_str(" ORDER BY scheduled_date ASC, sequence_number ASC");
        
        // Simplified query without dynamic params for now
        let deliveries = self.db.query_map(
            "SELECT * FROM deliveries ORDER BY scheduled_date ASC",
            [],
            |row| Self::row_to_delivery(row),
        )?;
        
        Ok(deliveries)
    }
    
    /// Create a new delivery
    pub async fn create_delivery(&self, mut delivery: Delivery) -> Result<Delivery> {
        delivery.id = new_id();
        delivery.delivery_number = self.generate_delivery_number()?;
        delivery.status = DeliveryStatus::Pending;
        delivery.created_at = Utc::now();
        
        self.db.execute(
            "INSERT INTO deliveries (
                id, delivery_number, route_id, shipment_id, status, sequence_number,
                customer_id, delivery_name, delivery_address_line1, delivery_address_line2,
                delivery_city, delivery_state, delivery_postal_code, delivery_country,
                delivery_phone, delivery_email, latitude, longitude, geofence_radius_meters,
                scheduled_date, scheduled_time_window_start, scheduled_time_window_end,
                delivery_instructions, signature_required, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &delivery.id,
                &delivery.delivery_number,
                &delivery.route_id,
                &delivery.shipment_id,
                "pending",
                &delivery.sequence_number,
                &delivery.customer_id,
                &delivery.delivery_address.name,
                &delivery.delivery_address.address.line1,
                &delivery.delivery_address.address.line2,
                &delivery.delivery_address.address.city,
                &delivery.delivery_address.address.state,
                &delivery.delivery_address.address.postal_code,
                &delivery.delivery_address.address.country,
                &delivery.delivery_address.phone,
                &delivery.delivery_address.email,
                delivery.location.map(|l| l.lat),
                delivery.location.map(|l| l.lng),
                delivery.geofence_radius_meters,
                delivery.scheduled_date.to_rfc3339(),
                &delivery.time_window_start,
                &delivery.time_window_end,
                &delivery.delivery_instructions,
                delivery.signature_required,
                delivery.created_at.to_rfc3339(),
            ],
        )?;
        
        info!("Created delivery: {}", delivery.delivery_number);
        Ok(delivery)
    }
    
    /// Update delivery status
    pub async fn update_status(
        &self,
        delivery_id: &str,
        status: DeliveryStatus,
        location: Option<GeoPoint>,
    ) -> Result<Delivery> {
        let status_str = format!("{:?}", status).to_lowercase();
        
        let rows = self.db.execute(
            "UPDATE deliveries SET status = ?, updated_at = datetime('now') WHERE id = ?",
            params![&status_str, delivery_id],
        )?;
        
        if rows == 0 {
            return Err(WmsError::not_found("Delivery not found"));
        }
        
        // Record status history
        self.record_status_history(delivery_id, status, location, None).await?;
        
        // Handle status-specific updates
        match status {
            DeliveryStatus::Arrived => {
                self.db.execute(
                    "UPDATE deliveries SET actual_arrival_time = datetime('now') WHERE id = ?",
                    params![delivery_id],
                )?;
            }
            DeliveryStatus::Delivered => {
                self.db.execute(
                    "UPDATE deliveries SET actual_departure_time = datetime('now') WHERE id = ?",
                    params![delivery_id],
                )?;
            }
            _ => {}
        }
        
        debug!("Updated delivery {} status to {:?}", delivery_id, status);
        self.get_delivery(delivery_id).await?
            .ok_or_else(|| WmsError::not_found("Delivery not found"))
    }
    
    /// Optimize route for multiple deliveries
    pub async fn optimize_route(
        &self,
        delivery_ids: &[String],
        start_location: GeoPoint,
    ) -> Result<OptimizedRoute> {
        // Get delivery locations
        let mut stops: Vec<(String, GeoPoint)> = Vec::new();
        
        for id in delivery_ids {
            let delivery = self.get_delivery(id).await?
                .ok_or_else(|| WmsError::not_found(format!("Delivery {} not found", id)))?;
            
            if let Some(loc) = delivery.location {
                stops.push((id.clone(), loc));
            } else {
                return Err(WmsError::validation(format!(
                    "Delivery {} has no location coordinates", id
                )));
            }
        }
        
        // Run optimization
        let optimized = self.route_optimizer.optimize(start_location, stops)?;
        
        // Update sequence numbers
        for (seq, delivery_id) in optimized.stop_order.iter().enumerate() {
            self.db.execute(
                "UPDATE deliveries SET sequence_number = ? WHERE id = ?",
                params![seq as u32 + 1, delivery_id],
            )?;
        }
        
        info!(
            "Optimized route with {} stops, total distance: {:.2} km",
            optimized.stop_order.len(),
            optimized.total_distance_km
        );
        
        Ok(optimized)
    }
    
    /// Check if current location is within delivery geofence
    pub async fn check_geofence(
        &self,
        delivery_id: &str,
        current_location: GeoPoint,
    ) -> Result<GeofenceResult> {
        let delivery = self.get_delivery(delivery_id).await?
            .ok_or_else(|| WmsError::not_found("Delivery not found"))?;
        
        let delivery_location = delivery.location
            .ok_or_else(|| WmsError::validation("Delivery has no location"))?;
        
        let result = self.geofence_checker.check_circle(
            current_location,
            delivery_location,
            delivery.geofence_radius_meters,
        );
        
        // If just entered geofence, auto-update status
        if result.is_inside && delivery.status == DeliveryStatus::EnRoute {
            self.update_status(delivery_id, DeliveryStatus::Arrived, Some(current_location)).await?;
        }
        
        Ok(result)
    }
    
    /// Record driver location
    pub async fn record_location(&self, location: DriverLocation) -> Result<()> {
        self.db.execute(
            "INSERT INTO driver_locations (
                id, user_id, route_id, latitude, longitude,
                accuracy_meters, speed_kmh, heading, recorded_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                new_id(),
                &location.user_id,
                &location.route_id,
                location.location.lat,
                location.location.lng,
                location.accuracy_meters,
                location.speed_kmh,
                location.heading,
                location.recorded_at.to_rfc3339(),
            ],
        )?;
        
        Ok(())
    }
    
    /// Get delivery by ID
    async fn get_delivery(&self, id: &str) -> Result<Option<Delivery>> {
        self.db.query_row(
            "SELECT * FROM deliveries WHERE id = ?",
            params![id],
            |row| Self::row_to_delivery(row),
        )
    }
    
    /// Record status history
    async fn record_status_history(
        &self,
        delivery_id: &str,
        status: DeliveryStatus,
        location: Option<GeoPoint>,
        notes: Option<&str>,
    ) -> Result<()> {
        self.db.execute(
            "INSERT INTO delivery_status_history (
                id, delivery_id, status, latitude, longitude, notes, recorded_at
            ) VALUES (?, ?, ?, ?, ?, ?, datetime('now'))",
            params![
                new_id(),
                delivery_id,
                format!("{:?}", status).to_lowercase(),
                location.map(|l| l.lat),
                location.map(|l| l.lng),
                notes,
            ],
        )?;
        
        Ok(())
    }
    
    fn generate_delivery_number(&self) -> Result<String> {
        let count: i64 = self.db.query_row(
            "SELECT COUNT(*) + 1 FROM deliveries",
            [],
            |row| row.get(0),
        )?.unwrap_or(1);
        
        Ok(format!("DEL-{:08}", count))
    }
    
    fn row_to_delivery(row: &rusqlite::Row) -> rusqlite::Result<Delivery> {
        let lat: Option<f64> = row.get("latitude")?;
        let lng: Option<f64> = row.get("longitude")?;
        let location = match (lat, lng) {
            (Some(la), Some(ln)) => Some(GeoPoint::new(la, ln)),
            _ => None,
        };
        
        Ok(Delivery {
            id: row.get("id")?,
            delivery_number: row.get("delivery_number")?,
            route_id: row.get("route_id")?,
            shipment_id: row.get("shipment_id")?,
            status: DeliveryStatus::Pending, // Parse from string
            sequence_number: row.get("sequence_number")?,
            customer_id: row.get("customer_id")?,
            delivery_address: DeliveryAddress {
                name: row.get("delivery_name")?,
                address: wms_core::types::Address {
                    line1: row.get("delivery_address_line1")?,
                    line2: row.get("delivery_address_line2")?,
                    city: row.get("delivery_city")?,
                    state: row.get("delivery_state")?,
                    postal_code: row.get("delivery_postal_code")?,
                    country: row.get("delivery_country")?,
                },
                phone: row.get("delivery_phone")?,
                email: row.get("delivery_email")?,
            },
            location,
            geofence_radius_meters: row.get("geofence_radius_meters").unwrap_or(100.0),
            scheduled_date: Utc::now(), // Parse from string
            time_window_start: row.get("scheduled_time_window_start")?,
            time_window_end: row.get("scheduled_time_window_end")?,
            estimated_arrival: None,
            actual_arrival: None,
            actual_departure: None,
            delivery_instructions: row.get("delivery_instructions")?,
            signature_required: row.get::<_, i32>("signature_required").unwrap_or(0) == 1,
            signature_name: row.get("signature_name")?,
            delivery_notes: row.get("delivery_notes")?,
            failure_reason: row.get("failure_reason")?,
            created_at: Utc::now(),
            updated_at: None,
        })
    }
}

