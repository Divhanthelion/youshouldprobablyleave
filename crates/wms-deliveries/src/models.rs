//! Delivery Data Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use wms_core::types::Address;

/// Geographic point (latitude/longitude)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lat: f64,
    pub lng: f64,
}

impl GeoPoint {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
    
    /// Calculate distance to another point in kilometers (Haversine formula)
    pub fn distance_to(&self, other: &GeoPoint) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        
        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();
        let dlat = (other.lat - self.lat).to_radians();
        let dlng = (other.lng - self.lng).to_radians();
        
        let a = (dlat / 2.0).sin().powi(2) 
            + lat1.cos() * lat2.cos() * (dlng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        
        EARTH_RADIUS_KM * c
    }
}

/// Individual delivery/stop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delivery {
    pub id: String,
    pub delivery_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_id: Option<String>,
    pub status: DeliveryStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    pub delivery_address: DeliveryAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeoPoint>,
    #[serde(default = "default_geofence_radius")]
    pub geofence_radius_meters: f64,
    pub scheduled_date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_arrival: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_arrival: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_departure: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_instructions: Option<String>,
    #[serde(default)]
    pub signature_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

fn default_geofence_radius() -> f64 {
    100.0
}

/// Delivery address with coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAddress {
    pub name: String,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Delivery status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    Pending,
    Assigned,
    EnRoute,
    Arrived,
    Delivered,
    Failed,
    Returned,
    Cancelled,
}

impl Default for DeliveryStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Delivery route (collection of stops)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRoute {
    pub id: String,
    pub route_name: String,
    pub route_date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_id: Option<String>,
    pub status: RouteStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_location: Option<GeoPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_location: Option<GeoPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_distance_km: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    /// Deliveries in this route
    #[serde(default)]
    pub deliveries: Vec<Delivery>,
}

/// Route status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RouteStatus {
    Planning,
    Assigned,
    InProgress,
    Completed,
    Cancelled,
}

impl Default for RouteStatus {
    fn default() -> Self {
        Self::Planning
    }
}

/// Vehicle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub vehicle_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub vehicle_type: VehicleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_plate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_m3: Option<f64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Vehicle types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VehicleType {
    Car,
    Van,
    Truck,
    Motorcycle,
}

/// Driver location update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverLocation {
    pub id: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    pub location: GeoPoint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_kmh: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<f64>,
    pub recorded_at: DateTime<Utc>,
}

/// Delivery status history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryStatusEntry {
    pub id: String,
    pub delivery_id: String,
    pub status: DeliveryStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeoPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_by: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

