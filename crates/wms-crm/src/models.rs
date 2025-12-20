//! CRM Data Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use wms_core::types::Address;

/// Customer record
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Customer {
    pub id: String,
    pub customer_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 100))]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 100))]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(email)]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(default)]
    pub customer_type: CustomerType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0.0))]
    pub credit_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<String>,
    #[serde(default = "default_currency")]
    pub currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_true")]
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// Associated addresses
    #[serde(default)]
    pub addresses: Vec<CustomerAddress>,
    /// Associated contacts
    #[serde(default)]
    pub contacts: Vec<CustomerContact>,
}

fn default_currency() -> String {
    "USD".to_string()
}

fn default_true() -> bool {
    true
}

/// Customer types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum CustomerType {
    #[default]
    Retail,
    Wholesale,
    Distributor,
    Government,
    Internal,
}

/// Customer address
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CustomerAddress {
    pub id: String,
    pub customer_id: String,
    #[serde(default)]
    pub address_type: AddressType,
    #[serde(default)]
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[validate(nested)]
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    pub created_at: DateTime<Utc>,
}

/// Address types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AddressType {
    #[default]
    Shipping,
    Billing,
    Both,
}

/// Customer contact person
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CustomerContact {
    pub id: String,
    pub customer_id: String,
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(email)]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default)]
    pub is_primary: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl CustomerContact {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

/// Customer interaction/activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInteraction {
    pub id: String,
    pub customer_id: String,
    pub interaction_type: InteractionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_notes: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

/// Interaction types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InteractionType {
    Call,
    Email,
    Meeting,
    Note,
    Order,
    Complaint,
    Support,
}

/// Customer search query
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerSearchQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<CustomerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    50
}

