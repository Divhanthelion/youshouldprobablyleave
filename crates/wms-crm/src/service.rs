//! CRM Service
//! 
//! Core business logic for customer relationship management.

use std::sync::Arc;
use chrono::Utc;
use rusqlite::params;
use tracing::{info, debug};
use validator::Validate;
use wms_core::db::Database;
use wms_core::error::{WmsError, Result};
use wms_core::types::new_id;
use crate::models::*;
use crate::validation::validate_phone_number;

/// CRM service
pub struct CrmService {
    db: Arc<Database>,
}

impl CrmService {
    /// Create a new CRM service
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
    
    /// Get all customers with pagination
    pub async fn get_customers(&self, page: u32, page_size: u32) -> Result<Vec<Customer>> {
        let offset = (page.saturating_sub(1)) * page_size;
        
        let customers = self.db.query_map(
            "SELECT * FROM customers WHERE is_active = 1
             ORDER BY company_name, last_name, first_name
             LIMIT ? OFFSET ?",
            params![page_size, offset],
            |row| Self::row_to_customer(row),
        )?;
        
        Ok(customers)
    }
    
    /// Get customer by ID
    pub async fn get_customer(&self, id: &str) -> Result<Option<Customer>> {
        let customer = self.db.query_row(
            "SELECT * FROM customers WHERE id = ?",
            params![id],
            |row| Self::row_to_customer(row),
        )?;
        
        if let Some(mut c) = customer {
            c.addresses = self.get_customer_addresses(&c.id)?;
            c.contacts = self.get_customer_contacts(&c.id)?;
            Ok(Some(c))
        } else {
            Ok(None)
        }
    }
    
    /// Create a new customer
    pub async fn create_customer(&self, mut customer: Customer) -> Result<Customer> {
        // Validate
        customer.validate()
            .map_err(|e| WmsError::validation(format!("Invalid customer data: {}", e)))?;
        
        // Validate phone numbers
        if let Some(ref phone) = customer.phone {
            validate_phone_number(phone)?;
        }
        if let Some(ref mobile) = customer.mobile {
            validate_phone_number(mobile)?;
        }
        
        customer.id = new_id();
        customer.customer_number = self.generate_customer_number()?;
        customer.created_at = Utc::now();
        
        let tags_json = serde_json::to_string(&customer.tags).unwrap_or_default();
        
        self.db.execute(
            "INSERT INTO customers (
                id, customer_number, company_name, first_name, last_name,
                email, phone, mobile, fax, website, tax_id, customer_type,
                credit_limit, payment_terms, currency_code, notes, tags,
                is_active, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &customer.id,
                &customer.customer_number,
                &customer.company_name,
                &customer.first_name,
                &customer.last_name,
                &customer.email,
                &customer.phone,
                &customer.mobile,
                &customer.fax,
                &customer.website,
                &customer.tax_id,
                format!("{:?}", customer.customer_type).to_lowercase(),
                &customer.credit_limit,
                &customer.payment_terms,
                &customer.currency_code,
                &customer.notes,
                &tags_json,
                customer.is_active,
                customer.created_at.to_rfc3339(),
            ],
        )?;
        
        // Insert addresses
        for mut addr in &mut customer.addresses {
            addr.id = new_id();
            addr.customer_id = customer.id.clone();
            self.insert_address(&addr)?;
        }
        
        // Insert contacts
        for mut contact in &mut customer.contacts {
            contact.id = new_id();
            contact.customer_id = customer.id.clone();
            self.insert_contact(&contact)?;
        }
        
        info!("Created customer: {} - {:?}", 
              customer.customer_number, 
              customer.company_name.as_ref().or(customer.last_name.as_ref()));
        
        Ok(customer)
    }
    
    /// Update an existing customer
    pub async fn update_customer(&self, mut customer: Customer) -> Result<Customer> {
        // Validate
        customer.validate()
            .map_err(|e| WmsError::validation(format!("Invalid customer data: {}", e)))?;
        
        customer.updated_at = Some(Utc::now());
        let tags_json = serde_json::to_string(&customer.tags).unwrap_or_default();
        
        let rows = self.db.execute(
            "UPDATE customers SET
                company_name = ?, first_name = ?, last_name = ?,
                email = ?, phone = ?, mobile = ?, fax = ?, website = ?,
                tax_id = ?, customer_type = ?, credit_limit = ?,
                payment_terms = ?, currency_code = ?, notes = ?, tags = ?,
                is_active = ?, updated_at = ?
             WHERE id = ?",
            params![
                &customer.company_name,
                &customer.first_name,
                &customer.last_name,
                &customer.email,
                &customer.phone,
                &customer.mobile,
                &customer.fax,
                &customer.website,
                &customer.tax_id,
                format!("{:?}", customer.customer_type).to_lowercase(),
                &customer.credit_limit,
                &customer.payment_terms,
                &customer.currency_code,
                &customer.notes,
                &tags_json,
                customer.is_active,
                customer.updated_at.map(|t| t.to_rfc3339()),
                &customer.id,
            ],
        )?;
        
        if rows == 0 {
            return Err(WmsError::not_found("Customer not found"));
        }
        
        debug!("Updated customer: {}", customer.customer_number);
        Ok(customer)
    }
    
    /// Search customers by various criteria
    pub async fn search_customers(&self, query: CustomerSearchQuery) -> Result<Vec<Customer>> {
        let offset = (query.page.saturating_sub(1)) * query.page_size;
        
        // Build dynamic search query
        let mut sql = String::from(
            "SELECT * FROM customers WHERE 1=1"
        );
        
        if query.query.is_some() {
            sql.push_str(" AND (
                company_name LIKE '%' || ? || '%' OR
                first_name LIKE '%' || ? || '%' OR
                last_name LIKE '%' || ? || '%' OR
                email LIKE '%' || ? || '%' OR
                customer_number LIKE '%' || ? || '%'
            )");
        }
        
        if query.is_active.is_some() {
            sql.push_str(" AND is_active = ?");
        }
        
        sql.push_str(" ORDER BY company_name, last_name LIMIT ? OFFSET ?");
        
        // For simplicity, execute basic search
        let customers = if let Some(q) = &query.query {
            self.db.query_map(
                "SELECT * FROM customers 
                 WHERE is_active = 1 AND (
                    company_name LIKE '%' || ? || '%' OR
                    first_name LIKE '%' || ? || '%' OR
                    last_name LIKE '%' || ? || '%' OR
                    email LIKE '%' || ? || '%'
                 )
                 ORDER BY company_name, last_name
                 LIMIT ? OFFSET ?",
                params![q, q, q, q, query.page_size, offset],
                |row| Self::row_to_customer(row),
            )?
        } else {
            self.get_customers(query.page, query.page_size).await?
        };
        
        Ok(customers)
    }
    
    /// Add an interaction/activity
    pub async fn add_interaction(&self, interaction: CustomerInteraction) -> Result<CustomerInteraction> {
        let mut interaction = interaction;
        interaction.id = new_id();
        interaction.created_at = Utc::now();
        
        self.db.execute(
            "INSERT INTO customer_interactions (
                id, customer_id, interaction_type, subject, description,
                outcome, follow_up_date, follow_up_notes, created_by, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &interaction.id,
                &interaction.customer_id,
                format!("{:?}", interaction.interaction_type).to_lowercase(),
                &interaction.subject,
                &interaction.description,
                &interaction.outcome,
                interaction.follow_up_date.map(|d| d.to_rfc3339()),
                &interaction.follow_up_notes,
                &interaction.created_by,
                interaction.created_at.to_rfc3339(),
            ],
        )?;
        
        Ok(interaction)
    }
    
    // Helper methods
    
    fn generate_customer_number(&self) -> Result<String> {
        let count: i64 = self.db.query_row(
            "SELECT COUNT(*) + 1 FROM customers",
            [],
            |row| row.get(0),
        )?.unwrap_or(1);
        
        Ok(format!("CUS-{:06}", count))
    }
    
    fn insert_address(&self, addr: &CustomerAddress) -> Result<()> {
        self.db.execute(
            "INSERT INTO customer_addresses (
                id, customer_id, address_type, is_default, contact_name,
                address_line1, address_line2, city, state, postal_code, country,
                phone, delivery_instructions, latitude, longitude, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &addr.id,
                &addr.customer_id,
                format!("{:?}", addr.address_type).to_lowercase(),
                addr.is_default,
                &addr.contact_name,
                &addr.address.line1,
                &addr.address.line2,
                &addr.address.city,
                &addr.address.state,
                &addr.address.postal_code,
                &addr.address.country,
                &addr.phone,
                &addr.delivery_instructions,
                &addr.latitude,
                &addr.longitude,
                addr.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }
    
    fn insert_contact(&self, contact: &CustomerContact) -> Result<()> {
        self.db.execute(
            "INSERT INTO customer_contacts (
                id, customer_id, first_name, last_name, title, department,
                email, phone, mobile, is_primary, notes, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &contact.id,
                &contact.customer_id,
                &contact.first_name,
                &contact.last_name,
                &contact.title,
                &contact.department,
                &contact.email,
                &contact.phone,
                &contact.mobile,
                contact.is_primary,
                &contact.notes,
                contact.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }
    
    fn get_customer_addresses(&self, customer_id: &str) -> Result<Vec<CustomerAddress>> {
        self.db.query_map(
            "SELECT * FROM customer_addresses WHERE customer_id = ? ORDER BY is_default DESC",
            params![customer_id],
            |row| {
                Ok(CustomerAddress {
                    id: row.get("id")?,
                    customer_id: row.get("customer_id")?,
                    address_type: AddressType::Shipping,
                    is_default: row.get::<_, i32>("is_default")? == 1,
                    contact_name: row.get("contact_name")?,
                    address: wms_core::types::Address {
                        line1: row.get("address_line1")?,
                        line2: row.get("address_line2")?,
                        city: row.get("city")?,
                        state: row.get("state")?,
                        postal_code: row.get("postal_code")?,
                        country: row.get("country")?,
                    },
                    phone: row.get("phone")?,
                    delivery_instructions: row.get("delivery_instructions")?,
                    latitude: row.get("latitude")?,
                    longitude: row.get("longitude")?,
                    created_at: Utc::now(),
                })
            },
        )
    }
    
    fn get_customer_contacts(&self, customer_id: &str) -> Result<Vec<CustomerContact>> {
        self.db.query_map(
            "SELECT * FROM customer_contacts WHERE customer_id = ? ORDER BY is_primary DESC",
            params![customer_id],
            |row| {
                Ok(CustomerContact {
                    id: row.get("id")?,
                    customer_id: row.get("customer_id")?,
                    first_name: row.get("first_name")?,
                    last_name: row.get("last_name")?,
                    title: row.get("title")?,
                    department: row.get("department")?,
                    email: row.get("email")?,
                    phone: row.get("phone")?,
                    mobile: row.get("mobile")?,
                    is_primary: row.get::<_, i32>("is_primary")? == 1,
                    notes: row.get("notes")?,
                    created_at: Utc::now(),
                })
            },
        )
    }
    
    fn row_to_customer(row: &rusqlite::Row) -> rusqlite::Result<Customer> {
        let tags_str: String = row.get("tags").unwrap_or_default();
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
        
        Ok(Customer {
            id: row.get("id")?,
            customer_number: row.get("customer_number")?,
            company_name: row.get("company_name")?,
            first_name: row.get("first_name")?,
            last_name: row.get("last_name")?,
            email: row.get("email")?,
            phone: row.get("phone")?,
            mobile: row.get("mobile")?,
            fax: row.get("fax")?,
            website: row.get("website")?,
            tax_id: row.get("tax_id")?,
            customer_type: CustomerType::Retail,
            credit_limit: row.get("credit_limit")?,
            payment_terms: row.get("payment_terms")?,
            currency_code: row.get("currency_code").unwrap_or_else(|_| "USD".to_string()),
            notes: row.get("notes")?,
            tags,
            is_active: row.get::<_, i32>("is_active")? == 1,
            created_at: Utc::now(),
            updated_at: None,
            addresses: Vec::new(),
            contacts: Vec::new(),
        })
    }
}

