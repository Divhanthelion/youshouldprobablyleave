//! Application State Management
//! 
//! Manages the global application state including database connections,
//! sync engine, and module services.

use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::info;

use wms_core::db::Database;
use wms_sync::SyncEngine;
use wms_inventory::InventoryService;
use wms_shipping::ShippingService;
use wms_deliveries::DeliveryService;
use wms_crm::CrmService;
use wms_timesheets::TimesheetService;

/// Global application state shared across all Tauri commands
pub struct AppState {
    /// Database connection pool
    pub db: Arc<Database>,
    /// Synchronization engine
    pub sync_engine: Arc<RwLock<SyncEngine>>,
    /// Inventory management service
    pub inventory: Arc<InventoryService>,
    /// Shipping management service
    pub shipping: Arc<ShippingService>,
    /// Delivery management service
    pub deliveries: Arc<DeliveryService>,
    /// CRM service
    pub crm: Arc<CrmService>,
    /// Timesheet service
    pub timesheets: Arc<TimesheetService>,
    /// Offline mode flag
    pub offline_mode: Arc<RwLock<bool>>,
}

impl AppState {
    /// Initialize the application state with all services
    pub fn new(app: AppHandle) -> Result<Self> {
        // Get app data directory for database storage
        let app_dir = app.path().app_data_dir()?;
        std::fs::create_dir_all(&app_dir)?;
        
        let db_path = app_dir.join("wms.db");
        info!("Initializing database at {:?}", db_path);
        
        // Initialize encrypted database
        let encryption_key = std::env::var("WMS_DB_KEY")
            .unwrap_or_else(|_| "default-dev-key-change-in-production".to_string());
        
        let db = Arc::new(Database::new(&db_path, &encryption_key)?);
        
        // Run migrations
        db.run_migrations()?;
        info!("Database migrations completed");
        
        // Initialize sync engine
        let sync_engine = Arc::new(RwLock::new(SyncEngine::new(db.clone())?));
        
        // Initialize services
        let inventory = Arc::new(InventoryService::new(db.clone()));
        let shipping = Arc::new(ShippingService::new(db.clone()));
        let deliveries = Arc::new(DeliveryService::new(db.clone()));
        let crm = Arc::new(CrmService::new(db.clone()));
        let timesheets = Arc::new(TimesheetService::new(db.clone()));
        
        info!("All services initialized successfully");
        
        Ok(Self {
            db,
            sync_engine,
            inventory,
            shipping,
            deliveries,
            crm,
            timesheets,
            offline_mode: Arc::new(RwLock::new(false)),
        })
    }
}

