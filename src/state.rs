//! Application State Management

use leptos::*;
use serde::{Deserialize, Serialize};

/// Global application state
#[derive(Clone, Debug)]
pub struct AppState {
    /// Current user
    pub user: RwSignal<Option<User>>,
    /// Sync status
    pub sync_status: RwSignal<SyncStatus>,
    /// Current module/page
    pub current_module: RwSignal<Module>,
    /// Toast notifications
    pub toasts: RwSignal<Vec<Toast>>,
    /// Theme
    pub theme: RwSignal<Theme>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            user: create_rw_signal(None),
            sync_status: create_rw_signal(SyncStatus::default()),
            current_module: create_rw_signal(Module::Dashboard),
            toasts: create_rw_signal(Vec::new()),
            theme: create_rw_signal(Theme::Dark),
        }
    }
    
    /// Add a toast notification
    pub fn toast(&self, message: &str, toast_type: ToastType) {
        let mut toasts = self.toasts.get();
        toasts.push(Toast {
            id: uuid::Uuid::new_v4().to_string(),
            message: message.to_string(),
            toast_type,
            timestamp: chrono::Utc::now().timestamp() as u64,
        });
        self.toasts.set(toasts);
    }
    
    /// Remove a toast
    pub fn dismiss_toast(&self, id: &str) {
        let toasts: Vec<Toast> = self.toasts.get()
            .into_iter()
            .filter(|t| t.id != id)
            .collect();
        self.toasts.set(toasts);
    }
}

/// Current user information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub full_name: String,
    pub role: String,
}

/// Synchronization status
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SyncStatus {
    pub is_syncing: bool,
    pub is_online: bool,
    pub pending_changes: u32,
    pub last_sync: Option<String>,
}

/// Application modules
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Module {
    #[default]
    Dashboard,
    Inventory,
    Shipping,
    Receiving,
    Deliveries,
    Customers,
    Timesheets,
    Settings,
}

impl Module {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Inventory => "Inventory",
            Self::Shipping => "Shipping",
            Self::Receiving => "Receiving",
            Self::Deliveries => "Deliveries",
            Self::Customers => "Customers",
            Self::Timesheets => "Timesheets",
            Self::Settings => "Settings",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Dashboard => "📊",
            Self::Inventory => "📦",
            Self::Shipping => "🚚",
            Self::Receiving => "📥",
            Self::Deliveries => "🗺️",
            Self::Customers => "👥",
            Self::Timesheets => "⏰",
            Self::Settings => "⚙️",
        }
    }
    
    pub fn path(&self) -> &'static str {
        match self {
            Self::Dashboard => "/",
            Self::Inventory => "/inventory",
            Self::Shipping => "/shipping",
            Self::Receiving => "/receiving",
            Self::Deliveries => "/deliveries",
            Self::Customers => "/customers",
            Self::Timesheets => "/timesheets",
            Self::Settings => "/settings",
        }
    }
}

/// Toast notification
#[derive(Clone, Debug)]
pub struct Toast {
    pub id: String,
    pub message: String,
    pub toast_type: ToastType,
    pub timestamp: u64,
}

/// Toast types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

/// Application theme
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

