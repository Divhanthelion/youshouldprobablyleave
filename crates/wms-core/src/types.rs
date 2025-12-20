//! Common Types for WMS

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Generate a new UUID v4
pub fn new_id() -> String {
    Uuid::new_v4().to_string()
}

/// Get current timestamp
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Format timestamp for database storage
pub fn format_timestamp(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

/// Parse timestamp from database
pub fn parse_timestamp(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

impl Pagination {
    pub fn new(page: u32, page_size: u32) -> Self {
        Self { page, page_size }
    }
    
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.page_size
    }
    
    pub fn limit(&self) -> u32 {
        self.page_size
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 50,
        }
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, page_size: u32, total_count: u64) -> Self {
        let total_pages = ((total_count as f64) / (page_size as f64)).ceil() as u32;
        Self {
            data,
            page,
            page_size,
            total_count,
            total_pages,
        }
    }
}

/// Sort direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Asc
    }
}

/// Address structure used across modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

impl Address {
    pub fn full_address(&self) -> String {
        let mut parts = vec![self.line1.clone()];
        if let Some(line2) = &self.line2 {
            parts.push(line2.clone());
        }
        parts.push(format!("{}, {} {}", self.city, self.state, self.postal_code));
        parts.push(self.country.clone());
        parts.join("\n")
    }
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}

/// Audit fields for tracking changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditInfo {
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

impl AuditInfo {
    pub fn new(user_id: &str) -> Self {
        Self {
            created_at: now(),
            created_by: user_id.to_string(),
            updated_at: None,
            updated_by: None,
        }
    }
    
    pub fn update(&mut self, user_id: &str) {
        self.updated_at = Some(now());
        self.updated_by = Some(user_id.to_string());
    }
}

/// Unit of measure for inventory
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UnitOfMeasure {
    Each,
    Case,
    Pallet,
    Kilogram,
    Pound,
    Liter,
    Gallon,
    Meter,
    Foot,
}

impl Default for UnitOfMeasure {
    fn default() -> Self {
        Self::Each
    }
}

impl std::fmt::Display for UnitOfMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Each => write!(f, "EA"),
            Self::Case => write!(f, "CS"),
            Self::Pallet => write!(f, "PL"),
            Self::Kilogram => write!(f, "KG"),
            Self::Pound => write!(f, "LB"),
            Self::Liter => write!(f, "L"),
            Self::Gallon => write!(f, "GAL"),
            Self::Meter => write!(f, "M"),
            Self::Foot => write!(f, "FT"),
        }
    }
}

