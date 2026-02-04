//! =============================================================================
//! WMS Frontend - Leptos 0.7 Research File
//! =============================================================================
//!
//! This file concatenates all frontend source code for research purposes.
//! The frontend is a WebAssembly (WASM) application built with Leptos 0.7
//! for a Warehouse Management System running on Tauri v2.
//!
//! ## Current Status
//!
//! The frontend has **92 compilation errors** related to Leptos 0.7 API changes.
//! The backend crates and Tauri app compile successfully.
//!
//! ## Error Categories (92 total errors)
//!
//! ### 1. ElementChild Trait Issues (~31 errors)
//! The `.child()` method requires the `ElementChild` trait in scope.
//! - Affected: Various component children rendering
//! - Fix: Add `use leptos::prelude::ElementChild;` or update view patterns
//!
//! ### 2. Type Mismatches (~27 errors)
//! Components expecting `String` but receiving `Option<String>`:
//! - Card `title` prop: expects `String`, gets `Option<String>`
//! - Similar issues with other optional props
//! - Fix: Update component signatures or call sites
//!
//! ### 3. Router Component Imports (~8 errors)
//! `A` component from leptos_router not in scope:
//! - Currently: `use leptos_router::components::A;`
//! - Leptos 0.7 may have changed the export path
//!
//! ### 4. Signal API Changes
//! - `signal()` function signature may have changed
//! - `RwSignal::new()` vs other constructors
//!
//! ### 5. View Macro Patterns
//! - Children rendering: `{children()}` vs newer patterns
//! - Optional rendering: `.map()` patterns in views
//! - `.into_any()` conversions
//!
//! ## Architecture Overview
//!
//! ```
//! frontend/src/
//! ├── lib.rs         - Module exports
//! ├── main.rs        - Entry point (mount_to_body)
//! ├── app.rs         - Router setup, main App component
//! ├── api.rs         - Tauri command bindings
//! ├── state.rs       - AppState, signals, enums
//! ├── components/    - Reusable UI components
//! │   ├── mod.rs
//! │   ├── sidebar.rs
//! │   ├── header.rs
//! │   ├── data_table.rs
//! │   ├── card.rs
//! │   ├── button.rs
//! │   ├── input.rs
//! │   ├── modal.rs
//! │   ├── toast.rs
//! │   ├── loading.rs
//! │   ├── badge.rs
//! │   └── chart.rs
//! └── pages/         - Route pages
//!     ├── mod.rs
//!     ├── dashboard.rs
//!     ├── inventory.rs
//!     ├── shipping.rs
//!     ├── receiving.rs
//!     ├── deliveries.rs
//!     ├── customers.rs
//!     ├── timesheets.rs
//!     ├── settings.rs
//!     └── not_found.rs
//! ```
//!
//! ## Key Leptos 0.7 Migration Points
//!
//! 1. **Imports**: `leptos::prelude::*` is the main import
//! 2. **Context**: `use leptos::context::{provide_context, use_context}`
//! 3. **Router**: `leptos_router::{components::*, hooks::*, path}`
//! 4. **Signals**: `RwSignal`, `Signal`, `ReadSignal`, `WriteSignal`
//! 5. **Mount**: `leptos::mount::mount_to_body`
//!
//! ## Dependencies (from frontend/Cargo.toml)
//!
//! - leptos = "0.7"
//! - leptos_router = "0.7"
//! - wasm-bindgen
//! - web-sys
//! - serde / serde_json
//! - uuid, chrono
//! - console_error_panic_hook
//!
//! =============================================================================

// =============================================================================
// FILE: frontend/src/lib.rs
// =============================================================================

//! WMS Frontend - Leptos Application
//!
//! Modern WebAssembly frontend for the Warehouse Management System.
//! Built with Leptos for reactive UI and compiled to WebAssembly.

mod app;
mod components;
mod pages;
mod api;
mod state;

pub use app::App;


// =============================================================================
// FILE: frontend/src/main.rs
// =============================================================================

//! WMS Frontend Entry Point

use leptos::prelude::*;
use leptos::mount::mount_to_body;
use wms_frontend::App;

fn main() {
    // Initialize console logging for debugging
    console_error_panic_hook::set_once();

    // Mount the app to the document body
    mount_to_body(App);
}


// =============================================================================
// FILE: frontend/src/app.rs
// =============================================================================

//! Main Application Component

use leptos::prelude::*;
use leptos::context::provide_context;
use leptos_router::components::{Router, Route, Routes};
use leptos_router::path;
use crate::components::*;
use crate::pages::*;
use crate::state::AppState;

/// Main application component with routing
#[component]
pub fn App() -> impl IntoView {
    // Provide global state
    provide_context(AppState::new());

    view! {
        <Router>
            <div class="app-container">
                <Sidebar/>
                <main class="main-content">
                    <Header/>
                    <div class="page-content">
                        <Routes fallback=|| NotFoundPage>
                            <Route path=path!("/") view=Dashboard/>
                            <Route path=path!("/inventory") view=InventoryPage/>
                            <Route path=path!("/inventory/:id") view=InventoryDetailPage/>
                            <Route path=path!("/shipping") view=ShippingPage/>
                            <Route path=path!("/shipping/new") view=NewShipmentPage/>
                            <Route path=path!("/receiving") view=ReceivingPage/>
                            <Route path=path!("/deliveries") view=DeliveriesPage/>
                            <Route path=path!("/deliveries/:id") view=DeliveryDetailPage/>
                            <Route path=path!("/customers") view=CustomersPage/>
                            <Route path=path!("/customers/:id") view=CustomerDetailPage/>
                            <Route path=path!("/timesheets") view=TimesheetsPage/>
                            <Route path=path!("/settings") view=SettingsPage/>
                        </Routes>
                    </div>
                </main>
            </div>
        </Router>
    }
}


// =============================================================================
// FILE: frontend/src/api.rs
// =============================================================================

//! Tauri API Bindings
//!
//! Provides type-safe bindings to Tauri backend commands.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Invoke a Tauri command
pub async fn tauri_invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let args_js = serde_wasm_bindgen::to_value(args)
        .map_err(|e| format!("Serialization error: {}", e))?;

    let result = invoke(cmd, args_js).await;

    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Deserialization error: {}", e))
}

// ============ Inventory API ============

#[derive(Serialize)]
pub struct GetItemsArgs {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Deserialize)]
pub struct InventoryItem {
    pub id: String,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub total_quantity: Option<f64>,
}

pub async fn get_all_items(page: u32, page_size: u32) -> Result<Vec<InventoryItem>, String> {
    tauri_invoke("get_all_items", &GetItemsArgs {
        page: Some(page),
        page_size: Some(page_size),
    }).await
}

// ============ Sync API ============

#[derive(Deserialize)]
pub struct SyncStatus {
    pub is_syncing: bool,
    pub last_sync_at: Option<String>,
    pub pending_changes: u64,
    pub sync_errors: u64,
}

pub async fn get_sync_status() -> Result<SyncStatus, String> {
    tauri_invoke("get_sync_status", &()).await
}

pub async fn sync_now() -> Result<SyncStatus, String> {
    tauri_invoke("sync_now", &()).await
}

// ============ Barcode API ============

#[derive(Serialize)]
pub struct ScanBarcodeArgs {
    pub image_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
pub struct BarcodeResult {
    pub text: String,
    pub format: String,
}

pub async fn scan_barcode(image_data: Vec<u8>, width: u32, height: u32) -> Result<BarcodeResult, String> {
    tauri_invoke("scan_barcode", &ScanBarcodeArgs {
        image_data,
        width,
        height,
    }).await
}

// ============ Timesheet API ============

#[derive(Serialize)]
pub struct ClockArgs {
    pub user_id: String,
    pub biometric_verified: bool,
}

#[derive(Deserialize)]
pub struct TimeEntry {
    pub id: String,
    pub clock_in_time: String,
    pub clock_out_time: Option<String>,
    pub total_hours: Option<f64>,
}

pub async fn clock_in(user_id: &str, biometric_verified: bool) -> Result<TimeEntry, String> {
    tauri_invoke("clock_in", &ClockArgs {
        user_id: user_id.to_string(),
        biometric_verified,
    }).await
}

pub async fn clock_out(user_id: &str, biometric_verified: bool) -> Result<TimeEntry, String> {
    tauri_invoke("clock_out", &ClockArgs {
        user_id: user_id.to_string(),
        biometric_verified,
    }).await
}


// =============================================================================
// FILE: frontend/src/state.rs
// =============================================================================

//! Application State Management

use leptos::prelude::*;
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
            user: RwSignal::new(None),
            sync_status: RwSignal::new(SyncStatus::default()),
            current_module: RwSignal::new(Module::Dashboard),
            toasts: RwSignal::new(Vec::new()),
            theme: RwSignal::new(Theme::Dark),
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


// =============================================================================
// FILE: frontend/src/components/mod.rs
// =============================================================================

//! UI Components

mod sidebar;
mod header;
mod data_table;
mod card;
mod button;
mod input;
mod modal;
mod toast;
mod loading;
mod badge;
mod chart;

pub use sidebar::Sidebar;
pub use header::Header;
pub use data_table::DataTable;
pub use card::{Card, StatCard};
pub use button::*;
pub use input::*;
pub use modal::Modal;
pub use toast::ToastContainer;
pub use loading::Loading;
pub use badge::Badge;
pub use chart::{Chart, DataPoint, Sparkline};


// =============================================================================
// FILE: frontend/src/components/sidebar.rs
// =============================================================================

//! Sidebar Navigation Component

use leptos::prelude::*;
use leptos::context::use_context;
use leptos_router::components::A;
use crate::state::{AppState, Module};

/// Sidebar navigation component
#[component]
pub fn Sidebar() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let collapsed = RwSignal::new(false);

    let modules = vec![
        Module::Dashboard,
        Module::Inventory,
        Module::Shipping,
        Module::Receiving,
        Module::Deliveries,
        Module::Customers,
        Module::Timesheets,
        Module::Settings,
    ];

    view! {
        <aside class=move || format!("sidebar {}", if collapsed.get() { "collapsed" } else { "" })>
            <div class="sidebar-header">
                <div class="logo">
                    <span class="logo-icon">"📦"</span>
                    <Show when=move || !collapsed.get()>
                        <span class="logo-text">"WMS"</span>
                    </Show>
                </div>
                <button
                    class="collapse-btn"
                    on:click=move |_| collapsed.update(|c| *c = !*c)
                >
                    {move || if collapsed.get() { "→" } else { "←" }}
                </button>
            </div>

            <nav class="sidebar-nav">
                <ul>
                    {modules.into_iter().map(|module| {
                        view! {
                            <li>
                                <A
                                    href=module.path()
                                    attr:class="nav-link"
                                >
                                    <span class="nav-icon">{module.icon()}</span>
                                    <Show when=move || !collapsed.get()>
                                        <span class="nav-text">{module.title()}</span>
                                    </Show>
                                </A>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </nav>

            <div class="sidebar-footer">
                <div class="sync-status">
                    <span class=move || {
                        let status = state.sync_status.get();
                        format!("status-dot {}", if status.is_online { "online" } else { "offline" })
                    }></span>
                    <Show when=move || !collapsed.get()>
                        <span class="status-text">
                            {move || {
                                let status = state.sync_status.get();
                                if status.is_syncing {
                                    "Syncing...".to_string()
                                } else if status.is_online {
                                    format!("{} pending", status.pending_changes)
                                } else {
                                    "Offline".to_string()
                                }
                            }}
                        </span>
                    </Show>
                </div>
            </div>
        </aside>
    }
}

// =============================================================================
// FILE: frontend/src/components/header.rs
// =============================================================================

//! Header Component

use leptos::prelude::*;
use leptos::context::use_context;
use crate::state::AppState;

/// Application header
#[component]
pub fn Header() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    let toggle_theme = move |_| {
        state.theme.update(|t| {
            *t = match t {
                crate::state::Theme::Light => crate::state::Theme::Dark,
                crate::state::Theme::Dark => crate::state::Theme::Light,
            }
        });
    };

    view! {
        <header class="app-header">
            <div class="header-left">
                <div class="search-box">
                    <span class="search-icon">"🔍"</span>
                    <input
                        type="text"
                        placeholder="Search inventory, shipments, customers..."
                        class="search-input"
                    />
                    <kbd class="search-shortcut">"⌘K"</kbd>
                </div>
            </div>

            <div class="header-right">
                <button class="header-btn" title="Scan barcode">
                    <span>"📷"</span>
                </button>

                <button class="header-btn" title="Notifications">
                    <span>"🔔"</span>
                    <span class="notification-badge">"3"</span>
                </button>

                <button class="header-btn" on:click=toggle_theme title="Toggle theme">
                    {move || if matches!(state.theme.get(), crate::state::Theme::Dark) { "☀️" } else { "🌙" }}
                </button>

                <div class="user-menu">
                    <button class="user-btn">
                        <span class="user-avatar">"👤"</span>
                        <span class="user-name">
                            {move || state.user.get().map(|u| u.full_name).unwrap_or_else(|| "Guest".to_string())}
                        </span>
                    </button>
                </div>
            </div>
        </header>
    }
}


// =============================================================================
// FILE: frontend/src/components/data_table.rs
// =============================================================================

//! Data Table Component
//!
//! Virtualized data table for displaying large datasets efficiently.

use leptos::prelude::*;

/// Column definition for data table
#[derive(Clone)]
pub struct Column {
    pub key: String,
    pub label: String,
    pub sortable: bool,
    pub width: Option<String>,
}

impl Column {
    pub fn new(key: &str, label: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            sortable: true,
            width: None,
        }
    }

    pub fn with_width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn not_sortable(mut self) -> Self {
        self.sortable = false;
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Basic data table component - placeholder for future generic table
#[component]
pub fn DataTable(
    columns: Vec<Column>,
    #[prop(default = false)] loading: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let sort_column = RwSignal::new(None::<String>);
    let sort_direction = RwSignal::new(SortDirection::Asc);

    let toggle_sort = move |col: String| {
        if sort_column.get().as_ref() == Some(&col) {
            sort_direction.update(|d| {
                *d = match d {
                    SortDirection::Asc => SortDirection::Desc,
                    SortDirection::Desc => SortDirection::Asc,
                }
            });
        } else {
            sort_column.set(Some(col));
            sort_direction.set(SortDirection::Asc);
        }
    };

    view! {
        <div class="data-table-container">
            <table class="data-table">
                <thead>
                    <tr>
                        {columns.iter().map(|col| {
                            let col_key = col.key.clone();
                            let col_key2 = col_key.clone();
                            let col_label = col.label.clone();
                            let sortable = col.sortable;
                            let width = col.width.clone();

                            view! {
                                <th
                                    class=move || format!("table-header {}", if sortable { "sortable" } else { "" })
                                    style=move || width.clone().map(|w| format!("width: {}", w)).unwrap_or_default()
                                    on:click=move |_| {
                                        if sortable {
                                            toggle_sort(col_key.clone())
                                        }
                                    }
                                >
                                    <span>{col_label.clone()}</span>
                                    {move || {
                                        if sortable && sort_column.get().as_ref() == Some(&col_key2) {
                                            Some(view! {
                                                <span class="sort-indicator">
                                                    {match sort_direction.get() {
                                                        SortDirection::Asc => "↑",
                                                        SortDirection::Desc => "↓",
                                                    }}
                                                </span>
                                            })
                                        } else {
                                            None
                                        }
                                    }}
                                </th>
                            }
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody>
                    <Show
                        when=move || !loading
                        fallback=|| view! {
                            <tr>
                                <td colspan="100" class="loading-row">
                                    <div class="loading-spinner"></div>
                                    "Loading..."
                                </td>
                            </tr>
                        }
                    >
                        {children()}
                    </Show>
                </tbody>
            </table>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/components/card.rs
// =============================================================================

//! Card Component

use leptos::prelude::*;

/// Card container component
#[component]
pub fn Card(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("card {}", class.unwrap_or_default())>
            {title.map(|t| view! {
                <div class="card-header">
                    <h3 class="card-title">{t}</h3>
                </div>
            })}
            <div class="card-body">
                {children()}
            </div>
        </div>
    }
}

/// Stat card for dashboard metrics
#[component]
pub fn StatCard(
    title: &'static str,
    value: Signal<String>,
    #[prop(optional)] trend: Option<f64>,
    #[prop(optional)] icon: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="stat-card">
            {icon.map(|i| view! { <span class="stat-icon">{i}</span> })}
            <div class="stat-content">
                <span class="stat-title">{title}</span>
                <span class="stat-value">{move || value.get()}</span>
                {trend.map(|t| {
                    let trend_class = if t >= 0.0 { "positive" } else { "negative" };
                    let trend_icon = if t >= 0.0 { "↑" } else { "↓" };
                    view! {
                        <span class=format!("stat-trend {}", trend_class)>
                            {trend_icon} {format!("{:.1}%", t.abs())}
                        </span>
                    }
                })}
            </div>
        </div>
    }
}


// =============================================================================
// FILE: frontend/src/components/button.rs
// =============================================================================

//! Button Components

use leptos::prelude::*;

/// Button variant
#[derive(Clone, Copy, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
}

impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Primary => "btn-primary",
            Self::Secondary => "btn-secondary",
            Self::Danger => "btn-danger",
            Self::Ghost => "btn-ghost",
        }
    }
}

/// Button size
#[derive(Clone, Copy, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    fn class(&self) -> &'static str {
        match self {
            Self::Small => "btn-sm",
            Self::Medium => "btn-md",
            Self::Large => "btn-lg",
        }
    }
}

/// Button component
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,
    #[prop(optional)] icon: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=format!("btn {} {}", variant.class(), size.class())
            disabled=disabled || loading
        >
            {if loading {
                view! { <span class="btn-spinner"></span> }.into_any()
            } else if let Some(i) = icon {
                view! { <span class="btn-icon">{i}</span> }.into_any()
            } else {
                view! {}.into_any()
            }}
            <span class="btn-text">{children()}</span>
        </button>
    }
}

/// Icon button
#[component]
pub fn IconButton(
    icon: &'static str,
    #[prop(optional)] title: Option<&'static str>,
    #[prop(optional)] variant: ButtonVariant,
) -> impl IntoView {
    view! {
        <button
            class=format!("icon-btn {}", variant.class())
            title=title
        >
            {icon}
        </button>
    }
}

// =============================================================================
// FILE: frontend/src/components/input.rs
// =============================================================================

//! Input Components

use leptos::prelude::*;
use leptos::ev::Event;
use wasm_bindgen::JsCast;

/// Get target value from event
fn event_target_value(ev: &Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|t| t.value())
        .unwrap_or_default()
}

/// Text input component
#[component]
pub fn TextInput(
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] input_type: Option<&'static str>,
) -> impl IntoView {
    let input_value = value.unwrap_or_else(|| RwSignal::new(String::new()));
    let has_error = error.is_some();
    let error_msg = error;

    view! {
        <div class=format!("form-group {}", if has_error { "has-error" } else { "" })>
            {label.map(|l| view! { <label class="form-label">{l}</label> })}
            <input
                type=input_type.unwrap_or("text")
                class="form-input"
                placeholder=placeholder
                disabled=disabled
                prop:value=move || input_value.get()
                on:input=move |ev| {
                    input_value.set(event_target_value(&ev));
                }
            />
            {error_msg.map(|e| view! { <span class="form-error">{e}</span> })}
        </div>
    }
}

/// Select dropdown
#[component]
pub fn Select(
    #[prop(optional)] label: Option<&'static str>,
    options: Vec<(String, String)>, // (value, label)
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] placeholder: Option<&'static str>,
) -> impl IntoView {
    let selected = value.unwrap_or_else(|| RwSignal::new(String::new()));

    view! {
        <div class="form-group">
            {label.map(|l| view! { <label class="form-label">{l}</label> })}
            <select
                class="form-select"
                on:change=move |ev| {
                    selected.set(event_target_value(&ev));
                }
            >
                {placeholder.map(|p| view! {
                    <option value="" disabled selected>{p}</option>
                })}
                {options.into_iter().map(|(val, label)| {
                    view! {
                        <option value=val.clone()>{label}</option>
                    }
                }).collect::<Vec<_>>()}
            </select>
        </div>
    }
}

/// Search input with debounce
#[component]
pub fn SearchInput(
    #[prop(optional)] placeholder: Option<&'static str>,
) -> impl IntoView {
    let query = RwSignal::new(String::new());

    view! {
        <div class="search-input-wrapper">
            <span class="search-icon">"🔍"</span>
            <input
                type="search"
                class="search-input"
                placeholder=placeholder.unwrap_or("Search...")
                prop:value=move || query.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    query.set(value);
                }
            />
            <Show when=move || !query.get().is_empty()>
                <button
                    class="search-clear"
                    on:click=move |_| query.set(String::new())
                >
                    "✕"
                </button>
            </Show>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/components/modal.rs
// =============================================================================

//! Modal Component

use leptos::prelude::*;

/// Modal dialog component
#[component]
pub fn Modal(
    #[prop(into)] open: Signal<bool>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] on_close: Option<Box<dyn Fn() + 'static>>,
    children: Children,
) -> impl IntoView {
    let close = move |_| {
        if let Some(ref handler) = on_close {
            handler();
        }
    };

    view! {
        <Show when=move || open.get()>
            <div class="modal-overlay" on:click=close>
                <div class="modal" on:click=|e| e.stop_propagation()>
                    <div class="modal-header">
                        {title.map(|t| view! { <h2 class="modal-title">{t}</h2> })}
                        <button class="modal-close" on:click=close>"✕"</button>
                    </div>
                    <div class="modal-body">
                        {children()}
                    </div>
                </div>
            </div>
        </Show>
    }
}


// =============================================================================
// FILE: frontend/src/components/toast.rs
// =============================================================================

//! Toast Notification Component

use leptos::prelude::*;
use leptos::context::use_context;
use crate::state::{AppState, ToastType};

/// Toast container that shows notifications
#[component]
pub fn ToastContainer() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    view! {
        <div class="toast-container">
            <For
                each=move || state.toasts.get()
                key=|toast| toast.id.clone()
                children=move |toast| {
                    let toast_class = match toast.toast_type {
                        ToastType::Success => "toast-success",
                        ToastType::Error => "toast-error",
                        ToastType::Warning => "toast-warning",
                        ToastType::Info => "toast-info",
                    };
                    let toast_id = toast.id.clone();

                    view! {
                        <div class=format!("toast {}", toast_class)>
                            <span class="toast-message">{toast.message.clone()}</span>
                            <button
                                class="toast-close"
                                on:click=move |_| state.dismiss_toast(&toast_id)
                            >"✕"</button>
                        </div>
                    }
                }
            />
        </div>
    }
}


// =============================================================================
// FILE: frontend/src/components/loading.rs
// =============================================================================

//! Loading Component

use leptos::prelude::*;

/// Loading spinner component
#[component]
pub fn Loading(
    #[prop(optional)] message: Option<&'static str>,
    #[prop(optional)] full_page: bool,
) -> impl IntoView {
    view! {
        <div class=if full_page { "loading-fullpage" } else { "loading" }>
            <div class="loading-spinner"></div>
            {message.map(|m| view! { <p class="loading-message">{m}</p> })}
        </div>
    }
}

/// Skeleton loader for content placeholders
#[component]
pub fn Skeleton(
    #[prop(optional)] width: Option<&'static str>,
    #[prop(optional)] height: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div
            class="skeleton"
            style=format!(
                "width: {}; height: {};",
                width.unwrap_or("100%"),
                height.unwrap_or("1rem")
            )
        ></div>
    }
}


// =============================================================================
// FILE: frontend/src/components/badge.rs
// =============================================================================

//! Badge Component

use leptos::prelude::*;

/// Badge variant
#[derive(Clone, Copy, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Danger,
    Info,
}

impl BadgeVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "badge-default",
            Self::Success => "badge-success",
            Self::Warning => "badge-warning",
            Self::Danger => "badge-danger",
            Self::Info => "badge-info",
        }
    }
}

/// Badge component for status indicators
#[component]
pub fn Badge(
    children: Children,
    #[prop(optional)] variant: BadgeVariant,
) -> impl IntoView {
    view! {
        <span class=format!("badge {}", variant.class())>
            {children()}
        </span>
    }
}

/// Status badge with predefined states
#[component]
pub fn StatusBadge(
    status: &'static str,
) -> impl IntoView {
    let (variant, label) = match status.to_lowercase().as_str() {
        "active" | "delivered" | "completed" | "approved" => (BadgeVariant::Success, status),
        "pending" | "draft" | "processing" => (BadgeVariant::Default, status),
        "warning" | "low_stock" | "partial" => (BadgeVariant::Warning, status),
        "error" | "failed" | "cancelled" | "rejected" => (BadgeVariant::Danger, status),
        _ => (BadgeVariant::Info, status),
    };

    view! {
        <Badge variant=variant>
            {label}
        </Badge>
    }
}


// =============================================================================
// FILE: frontend/src/components/chart.rs
// =============================================================================

//! Chart Component
//!
//! Simple SVG-based charts for dashboard visualizations.

use leptos::prelude::*;

/// Chart data point
#[derive(Clone)]
pub struct DataPoint {
    pub label: String,
    pub value: f64,
}

/// Simple bar chart
#[component]
pub fn Chart(
    data: Vec<DataPoint>,
    #[prop(optional)] height: Option<u32>,
    #[prop(optional)] show_labels: bool,
) -> impl IntoView {
    let height = height.unwrap_or(200);
    let max_value = data.iter().map(|d| d.value).fold(0.0_f64, f64::max);
    let bar_width = 100.0 / (data.len() as f64 * 1.5);

    view! {
        <div class="chart-container">
            <svg
                class="chart"
                viewBox=format!("0 0 100 {}", height)
                preserveAspectRatio="none"
            >
                {data.iter().enumerate().map(|(i, point)| {
                    let bar_height = if max_value > 0.0 {
                        (point.value / max_value) * (height as f64 - 20.0)
                    } else {
                        0.0
                    };
                    let x = (i as f64) * bar_width * 1.5 + bar_width * 0.25;
                    let y = height as f64 - bar_height - 10.0;

                    view! {
                        <g class="chart-bar">
                            <rect
                                x=x
                                y=y
                                width=bar_width
                                height=bar_height
                                class="bar"
                                rx="2"
                            />
                            {if show_labels {
                                Some(view! {
                                    <text
                                        x=x + bar_width / 2.0
                                        y=height as f64 - 2.0
                                        class="bar-label"
                                        text-anchor="middle"
                                    >
                                        {point.label.clone()}
                                    </text>
                                })
                            } else {
                                None
                            }}
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </svg>
        </div>
    }
}

/// Sparkline mini chart
#[component]
pub fn Sparkline(
    data: Vec<f64>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let width = 100.0;
    let height = 30.0;
    let max_val = data.iter().fold(0.0_f64, |a, &b| a.max(b));
    let min_val = data.iter().fold(f64::MAX, |a, &b| a.min(b));
    let range = max_val - min_val;

    let points: String = data.iter().enumerate().map(|(i, &val)| {
        let x = (i as f64 / (data.len() - 1) as f64) * width;
        let y = if range > 0.0 {
            height - ((val - min_val) / range * height)
        } else {
            height / 2.0
        };
        format!("{:.1},{:.1}", x, y)
    }).collect::<Vec<_>>().join(" ");

    view! {
        <svg class="sparkline" viewBox=format!("0 0 {} {}", width, height)>
            <polyline
                points=points
                fill="none"
                stroke=color.unwrap_or("currentColor")
                stroke-width="2"
            />
        </svg>
    }
}


// =============================================================================
// FILE: frontend/src/pages/mod.rs
// =============================================================================

//! Page Components

mod dashboard;
mod inventory;
mod shipping;
mod receiving;
mod deliveries;
mod customers;
mod timesheets;
mod settings;
mod not_found;

pub use dashboard::Dashboard;
pub use inventory::{InventoryPage, InventoryDetailPage};
pub use shipping::{ShippingPage, NewShipmentPage};
pub use receiving::ReceivingPage;
pub use deliveries::{DeliveriesPage, DeliveryDetailPage};
pub use customers::{CustomersPage, CustomerDetailPage};
pub use timesheets::TimesheetsPage;
pub use settings::SettingsPage;
pub use not_found::NotFoundPage;


// =============================================================================
// FILE: frontend/src/pages/dashboard.rs
// =============================================================================

//! Dashboard Page

use leptos::prelude::*;
use crate::components::{Card, StatCard, Chart, DataPoint};

/// Main dashboard with key metrics
#[component]
pub fn Dashboard() -> impl IntoView {
    // Mock data - would come from Tauri commands
    let (inventory_count, _) = signal("12,456".to_string());
    let (pending_shipments, _) = signal("47".to_string());
    let (deliveries_today, _) = signal("23".to_string());
    let (low_stock_items, _) = signal("8".to_string());

    let chart_data = vec![
        DataPoint { label: "Mon".to_string(), value: 120.0 },
        DataPoint { label: "Tue".to_string(), value: 150.0 },
        DataPoint { label: "Wed".to_string(), value: 180.0 },
        DataPoint { label: "Thu".to_string(), value: 140.0 },
        DataPoint { label: "Fri".to_string(), value: 200.0 },
        DataPoint { label: "Sat".to_string(), value: 90.0 },
        DataPoint { label: "Sun".to_string(), value: 60.0 },
    ];

    view! {
        <div class="dashboard">
            <div class="page-header">
                <h1>"Dashboard"</h1>
                <p class="subtitle">"Welcome back! Here's your warehouse overview."</p>
            </div>

            <div class="stats-grid">
                <StatCard
                    title="Total Inventory"
                    value=inventory_count.into()
                    icon=Some("📦")
                    trend=Some(5.2)
                />
                <StatCard
                    title="Pending Shipments"
                    value=pending_shipments.into()
                    icon=Some("🚚")
                    trend=Some(-2.1)
                />
                <StatCard
                    title="Deliveries Today"
                    value=deliveries_today.into()
                    icon=Some("📍")
                    trend=Some(12.5)
                />
                <StatCard
                    title="Low Stock Alerts"
                    value=low_stock_items.into()
                    icon=Some("⚠️")
                    trend=Some(-3.0)
                />
            </div>

            <div class="dashboard-grid">
                <Card title=Some("Shipments This Week".to_string())>
                    <Chart data=chart_data.clone() show_labels=true />
                </Card>

                <Card title=Some("Recent Activity".to_string())>
                    <div class="activity-list">
                        <ActivityItem
                            icon="📥"
                            title="Receipt RCV-00000123 completed"
                            time="5 minutes ago"
                        />
                        <ActivityItem
                            icon="🚚"
                            title="Shipment SHP-00000456 dispatched"
                            time="12 minutes ago"
                        />
                        <ActivityItem
                            icon="📍"
                            title="Delivery DEL-00000789 delivered"
                            time="25 minutes ago"
                        />
                        <ActivityItem
                            icon="⚠️"
                            title="Low stock alert: SKU-12345"
                            time="1 hour ago"
                        />
                    </div>
                </Card>

                <Card title=Some("Quick Actions".to_string())>
                    <div class="quick-actions">
                        <QuickActionButton icon="📷" label="Scan Barcode" />
                        <QuickActionButton icon="📦" label="New Shipment" />
                        <QuickActionButton icon="📥" label="Receive Goods" />
                        <QuickActionButton icon="🗺️" label="Route Planner" />
                    </div>
                </Card>

                <Card title=Some("Top Products".to_string())>
                    <table class="mini-table">
                        <thead>
                            <tr>
                                <th>"SKU"</th>
                                <th>"Name"</th>
                                <th>"Stock"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td>"SKU-001"</td><td>"Widget Pro"</td><td>"1,234"</td></tr>
                            <tr><td>"SKU-002"</td><td>"Gadget Plus"</td><td>"956"</td></tr>
                            <tr><td>"SKU-003"</td><td>"Tool Master"</td><td>"723"</td></tr>
                            <tr><td>"SKU-004"</td><td>"Part Essential"</td><td>"512"</td></tr>
                        </tbody>
                    </table>
                </Card>
            </div>
        </div>
    }
}

#[component]
fn ActivityItem(
    icon: &'static str,
    title: &'static str,
    time: &'static str,
) -> impl IntoView {
    view! {
        <div class="activity-item">
            <span class="activity-icon">{icon}</span>
            <div class="activity-content">
                <span class="activity-title">{title}</span>
                <span class="activity-time">{time}</span>
            </div>
        </div>
    }
}

#[component]
fn QuickActionButton(
    icon: &'static str,
    label: &'static str,
) -> impl IntoView {
    view! {
        <button class="quick-action-btn">
            <span class="qa-icon">{icon}</span>
            <span class="qa-label">{label}</span>
        </button>
    }
}

// =============================================================================
// FILE: frontend/src/pages/inventory.rs
// =============================================================================

//! Inventory Page

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::components::{Card, SearchInput};

/// Inventory list page
#[component]
pub fn InventoryPage() -> impl IntoView {
    let items = RwSignal::new(vec![
        InventoryRow { sku: "SKU-001".into(), name: "Widget Pro".into(), qty: 1234, reorder: 100, status: "In Stock".into() },
        InventoryRow { sku: "SKU-002".into(), name: "Gadget Plus".into(), qty: 956, reorder: 200, status: "In Stock".into() },
        InventoryRow { sku: "SKU-003".into(), name: "Tool Master".into(), qty: 45, reorder: 100, status: "Low Stock".into() },
        InventoryRow { sku: "SKU-004".into(), name: "Part Essential".into(), qty: 512, reorder: 50, status: "In Stock".into() },
    ]);

    view! {
        <div class="page inventory-page">
            <div class="page-header">
                <div>
                    <h1>"Inventory"</h1>
                    <p class="subtitle">"Manage your warehouse inventory"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-secondary">"Export"</button>
                    <button class="btn btn-primary">"+ Add Item"</button>
                </div>
            </div>

            <Card>
                <div class="table-toolbar">
                    <SearchInput placeholder=Some("Search by SKU or name...") />
                    <select class="form-select">
                        <option>"All Categories"</option>
                        <option>"Electronics"</option>
                        <option>"Hardware"</option>
                    </select>
                </div>

                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"SKU"</th>
                            <th>"Name"</th>
                            <th>"Quantity"</th>
                            <th>"Reorder Point"</th>
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || items.get()
                            key=|item| item.sku.clone()
                            children=move |item| {
                                let status_class = if item.status == "Low Stock" { "badge-warning" } else { "badge-success" };
                                view! {
                                    <tr>
                                        <td><code>{item.sku.clone()}</code></td>
                                        <td>{item.name.clone()}</td>
                                        <td>{item.qty.to_string()}</td>
                                        <td>{item.reorder.to_string()}</td>
                                        <td><span class=format!("badge {}", status_class)>{item.status.clone()}</span></td>
                                        <td>
                                            <A href=format!("/inventory/{}", item.sku) attr:class="btn btn-sm btn-ghost">"View"</A>
                                        </td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

#[derive(Clone)]
struct InventoryRow {
    sku: String,
    name: String,
    qty: i32,
    reorder: i32,
    status: String,
}

/// Inventory detail page
#[component]
pub fn InventoryDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").cloned().unwrap_or_default();

    view! {
        <div class="page inventory-detail">
            <div class="page-header">
                <A href="/inventory" attr:class="back-link">"← Back to Inventory"</A>
                <h1>"Item: " {id}</h1>
            </div>

            <div class="detail-grid">
                <Card title=Some("Item Details".to_string())>
                    <dl class="detail-list">
                        <dt>"SKU"</dt><dd>{id}</dd>
                        <dt>"Name"</dt><dd>"Widget Pro"</dd>
                        <dt>"Category"</dt><dd>"Electronics"</dd>
                        <dt>"Unit"</dt><dd>"Each"</dd>
                        <dt>"Barcode"</dt><dd>"5012345678901"</dd>
                    </dl>
                </Card>

                <Card title=Some("Stock Levels".to_string())>
                    <div class="stock-info">
                        <div class="stock-stat">
                            <span class="stat-value">"1,234"</span>
                            <span class="stat-label">"On Hand"</span>
                        </div>
                        <div class="stock-stat">
                            <span class="stat-value">"50"</span>
                            <span class="stat-label">"Reserved"</span>
                        </div>
                        <div class="stock-stat">
                            <span class="stat-value">"100"</span>
                            <span class="stat-label">"Reorder Point"</span>
                        </div>
                    </div>
                </Card>
            </div>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/pages/shipping.rs
// =============================================================================

//! Shipping Page

use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::Card;

/// Shipping list page
#[component]
pub fn ShippingPage() -> impl IntoView {
    view! {
        <div class="page shipping-page">
            <div class="page-header">
                <div>
                    <h1>"Shipping"</h1>
                    <p class="subtitle">"Manage outbound shipments"</p>
                </div>
                <div class="page-actions">
                    <A href="/shipping/new" attr:class="btn btn-primary">"+ New Shipment"</A>
                </div>
            </div>

            <div class="shipments-tabs">
                <button class="tab active">"All"</button>
                <button class="tab">"Draft"</button>
                <button class="tab">"Picking"</button>
                <button class="tab">"Packed"</button>
                <button class="tab">"Shipped"</button>
            </div>

            <Card>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Shipment #"</th>
                            <th>"Customer"</th>
                            <th>"Items"</th>
                            <th>"Ship Date"</th>
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"SHP-00000123"</code></td>
                            <td>"Acme Corp"</td>
                            <td>"5"</td>
                            <td>"2024-01-15"</td>
                            <td><span class="badge badge-warning">"Picking"</span></td>
                            <td>
                                <button class="btn btn-sm btn-ghost">"View"</button>
                                <button class="btn btn-sm btn-primary">"Print Label"</button>
                            </td>
                        </tr>
                        <tr>
                            <td><code>"SHP-00000122"</code></td>
                            <td>"Tech Solutions"</td>
                            <td>"12"</td>
                            <td>"2024-01-15"</td>
                            <td><span class="badge badge-success">"Shipped"</span></td>
                            <td>
                                <button class="btn btn-sm btn-ghost">"View"</button>
                                <button class="btn btn-sm btn-ghost">"Track"</button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

/// New shipment page
#[component]
pub fn NewShipmentPage() -> impl IntoView {
    view! {
        <div class="page new-shipment">
            <div class="page-header">
                <A href="/shipping" attr:class="back-link">"← Back to Shipping"</A>
                <h1>"New Shipment"</h1>
            </div>

            <form class="shipment-form">
                <Card title=Some("Ship To".to_string())>
                    <div class="form-grid">
                        <div class="form-group">
                            <label>"Customer"</label>
                            <select class="form-select">
                                <option>"Select customer..."</option>
                                <option>"Acme Corp"</option>
                                <option>"Tech Solutions"</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label>"Name"</label>
                            <input type="text" class="form-input" placeholder="Recipient name" />
                        </div>
                        <div class="form-group full-width">
                            <label>"Address"</label>
                            <input type="text" class="form-input" placeholder="Street address" />
                        </div>
                        <div class="form-group">
                            <label>"City"</label>
                            <input type="text" class="form-input" />
                        </div>
                        <div class="form-group">
                            <label>"State"</label>
                            <input type="text" class="form-input" />
                        </div>
                        <div class="form-group">
                            <label>"ZIP Code"</label>
                            <input type="text" class="form-input" />
                        </div>
                    </div>
                </Card>

                <Card title=Some("Items".to_string())>
                    <div class="items-section">
                        <button type="button" class="btn btn-secondary">"+ Add Item"</button>
                        <button type="button" class="btn btn-secondary">"📷 Scan"</button>
                    </div>
                </Card>

                <div class="form-actions">
                    <button type="button" class="btn btn-secondary">"Save Draft"</button>
                    <button type="submit" class="btn btn-primary">"Create Shipment"</button>
                </div>
            </form>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/pages/receiving.rs
// =============================================================================

//! Receiving Page

use leptos::prelude::*;
use crate::components::Card;

#[component]
pub fn ReceivingPage() -> impl IntoView {
    view! {
        <div class="page receiving-page">
            <div class="page-header">
                <div>
                    <h1>"Receiving"</h1>
                    <p class="subtitle">"Process inbound shipments"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-primary">"+ New Receipt"</button>
                </div>
            </div>

            <Card>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Receipt #"</th>
                            <th>"PO Number"</th>
                            <th>"Supplier"</th>
                            <th>"Expected"</th>
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"RCV-00000045"</code></td>
                            <td>"PO-2024-0123"</td>
                            <td>"Global Supplies Inc"</td>
                            <td>"Today"</td>
                            <td><span class="badge badge-warning">"Receiving"</span></td>
                            <td>
                                <button class="btn btn-sm btn-primary">"Continue"</button>
                            </td>
                        </tr>
                        <tr>
                            <td><code>"RCV-00000044"</code></td>
                            <td>"PO-2024-0122"</td>
                            <td>"Parts Direct"</td>
                            <td>"Yesterday"</td>
                            <td><span class="badge badge-success">"Completed"</span></td>
                            <td>
                                <button class="btn btn-sm btn-ghost">"View"</button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/pages/deliveries.rs
// =============================================================================

//! Deliveries Page

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::components::Card;

#[component]
pub fn DeliveriesPage() -> impl IntoView {
    view! {
        <div class="page deliveries-page">
            <div class="page-header">
                <div>
                    <h1>"Deliveries"</h1>
                    <p class="subtitle">"Track and optimize delivery routes"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-secondary">"🗺️ Route Planner"</button>
                    <button class="btn btn-primary">"+ New Delivery"</button>
                </div>
            </div>

            <div class="delivery-stats">
                <div class="stat-card">
                    <span class="stat-icon">"🚚"</span>
                    <span class="stat-value">"12"</span>
                    <span class="stat-label">"In Transit"</span>
                </div>
                <div class="stat-card">
                    <span class="stat-icon">"✅"</span>
                    <span class="stat-value">"28"</span>
                    <span class="stat-label">"Delivered Today"</span>
                </div>
                <div class="stat-card">
                    <span class="stat-icon">"⏳"</span>
                    <span class="stat-value">"5"</span>
                    <span class="stat-label">"Pending"</span>
                </div>
            </div>

            <Card title=Some("Today's Deliveries".to_string())>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Delivery #"</th>
                            <th>"Customer"</th>
                            <th>"Address"</th>
                            <th>"Time Window"</th>
                            <th>"Status"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"DEL-00000789"</code></td>
                            <td>"John Smith"</td>
                            <td>"123 Main St, NYC"</td>
                            <td>"9:00 - 12:00"</td>
                            <td><span class="badge badge-info">"En Route"</span></td>
                            <td>
                                <A href="/deliveries/DEL-00000789" attr:class="btn btn-sm btn-ghost">"Track"</A>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

#[component]
pub fn DeliveryDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").cloned().unwrap_or_default();

    view! {
        <div class="page delivery-detail">
            <div class="page-header">
                <A href="/deliveries" attr:class="back-link">"← Back to Deliveries"</A>
                <h1>"Delivery: " {id}</h1>
            </div>

            <div class="delivery-map">
                <div class="map-placeholder">"🗺️ Map would render here"</div>
            </div>

            <Card title=Some("Delivery Details".to_string())>
                <dl class="detail-list">
                    <dt>"Customer"</dt><dd>"John Smith"</dd>
                    <dt>"Address"</dt><dd>"123 Main St, New York, NY 10001"</dd>
                    <dt>"Phone"</dt><dd>"+1 (555) 123-4567"</dd>
                    <dt>"Instructions"</dt><dd>"Leave at front door"</dd>
                </dl>
            </Card>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/pages/customers.rs
// =============================================================================

//! Customers Page

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::components::Card;

#[component]
pub fn CustomersPage() -> impl IntoView {
    view! {
        <div class="page customers-page">
            <div class="page-header">
                <div>
                    <h1>"Customers"</h1>
                    <p class="subtitle">"Manage customer relationships"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-primary">"+ Add Customer"</button>
                </div>
            </div>

            <Card>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Customer #"</th>
                            <th>"Company"</th>
                            <th>"Contact"</th>
                            <th>"Email"</th>
                            <th>"Type"</th>
                            <th>"Actions"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>"CUS-000001"</code></td>
                            <td>"Acme Corporation"</td>
                            <td>"Jane Doe"</td>
                            <td>"jane@acme.com"</td>
                            <td><span class="badge">"Wholesale"</span></td>
                            <td>
                                <A href="/customers/CUS-000001" attr:class="btn btn-sm btn-ghost">"View"</A>
                            </td>
                        </tr>
                        <tr>
                            <td><code>"CUS-000002"</code></td>
                            <td>"Tech Solutions"</td>
                            <td>"Bob Smith"</td>
                            <td>"bob@techsol.com"</td>
                            <td><span class="badge">"Retail"</span></td>
                            <td>
                                <A href="/customers/CUS-000002" attr:class="btn btn-sm btn-ghost">"View"</A>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        </div>
    }
}

#[component]
pub fn CustomerDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").cloned().unwrap_or_default();

    view! {
        <div class="page customer-detail">
            <div class="page-header">
                <A href="/customers" attr:class="back-link">"← Back to Customers"</A>
                <h1>"Customer: " {id}</h1>
            </div>

            <div class="detail-grid">
                <Card title=Some("Contact Information".to_string())>
                    <dl class="detail-list">
                        <dt>"Company"</dt><dd>"Acme Corporation"</dd>
                        <dt>"Contact"</dt><dd>"Jane Doe"</dd>
                        <dt>"Email"</dt><dd>"jane@acme.com"</dd>
                        <dt>"Phone"</dt><dd>"+1 (555) 987-6543"</dd>
                    </dl>
                </Card>

                <Card title=Some("Shipping Address".to_string())>
                    <address>
                        "Acme Corporation"<br/>
                        "456 Business Ave"<br/>
                        "Suite 100"<br/>
                        "Los Angeles, CA 90001"
                    </address>
                </Card>
            </div>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/pages/timesheets.rs
// =============================================================================

//! Timesheets Page

use leptos::prelude::*;
use crate::components::Card;

#[component]
pub fn TimesheetsPage() -> impl IntoView {
    let clocked_in = RwSignal::new(false);

    let toggle_clock = move |_| {
        clocked_in.update(|c| *c = !*c);
    };

    view! {
        <div class="page timesheets-page">
            <div class="page-header">
                <div>
                    <h1>"Timesheets"</h1>
                    <p class="subtitle">"Track your work hours"</p>
                </div>
                <div class="page-actions">
                    <button class="btn btn-secondary">"Export"</button>
                </div>
            </div>

            <Card class=Some("clock-card".to_string())>
                <div class="clock-section">
                    <div class="current-time">"09:45 AM"</div>
                    <div class="clock-status">
                        {move || if clocked_in.get() {
                            "Clocked in since 8:00 AM"
                        } else {
                            "Not clocked in"
                        }}
                    </div>
                    <button
                        class=move || format!("clock-btn {}", if clocked_in.get() { "clocked-in" } else { "" })
                        on:click=toggle_clock
                    >
                        {move || if clocked_in.get() { "🔴 Clock Out" } else { "🟢 Clock In" }}
                    </button>
                    <p class="biometric-note">"Biometric verification required"</p>
                </div>
            </Card>

            <Card title=Some("This Week".to_string())>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th>"Date"</th>
                            <th>"Clock In"</th>
                            <th>"Clock Out"</th>
                            <th>"Break"</th>
                            <th>"Total"</th>
                            <th>"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Mon, Jan 15"</td>
                            <td>"8:00 AM"</td>
                            <td>"5:30 PM"</td>
                            <td>"30 min"</td>
                            <td>"9.0 hrs"</td>
                            <td><span class="badge badge-success">"Approved"</span></td>
                        </tr>
                        <tr>
                            <td>"Tue, Jan 16"</td>
                            <td>"8:15 AM"</td>
                            <td>"5:00 PM"</td>
                            <td>"30 min"</td>
                            <td>"8.25 hrs"</td>
                            <td><span class="badge badge-success">"Approved"</span></td>
                        </tr>
                        <tr class="current-row">
                            <td>"Wed, Jan 17"</td>
                            <td>"8:00 AM"</td>
                            <td>"-"</td>
                            <td>"-"</td>
                            <td>"-"</td>
                            <td><span class="badge badge-info">"Active"</span></td>
                        </tr>
                    </tbody>
                </table>

                <div class="week-summary">
                    <span>"Week Total: "</span>
                    <strong>"17.25 hrs"</strong>
                    <span class="overtime">" (0.25 hrs overtime)"</span>
                </div>
            </Card>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/pages/settings.rs
// =============================================================================

//! Settings Page

use leptos::prelude::*;
use leptos::context::use_context;
use leptos::ev::Event;
use wasm_bindgen::JsCast;
use crate::components::Card;
use crate::state::{AppState, Theme};

fn event_target_value(ev: &Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
        .map(|t| t.value())
        .unwrap_or_default()
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    view! {
        <div class="page settings-page">
            <div class="page-header">
                <h1>"Settings"</h1>
                <p class="subtitle">"Configure application preferences"</p>
            </div>

            <div class="settings-grid">
                <Card title=Some("Appearance".to_string())>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Theme"</span>
                            <span class="setting-description">"Choose your preferred color scheme"</span>
                        </div>
                        <select
                            class="form-select"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                state.theme.set(if value == "dark" { Theme::Dark } else { Theme::Light });
                            }
                        >
                            <option value="dark" selected=move || matches!(state.theme.get(), Theme::Dark)>"Dark"</option>
                            <option value="light" selected=move || matches!(state.theme.get(), Theme::Light)>"Light"</option>
                        </select>
                    </div>
                </Card>

                <Card title=Some("Synchronization".to_string())>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Auto Sync"</span>
                            <span class="setting-description">"Automatically sync when online"</span>
                        </div>
                        <label class="toggle">
                            <input type="checkbox" checked />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Sync Interval"</span>
                            <span class="setting-description">"How often to sync in background"</span>
                        </div>
                        <select class="form-select">
                            <option>"Every 5 minutes"</option>
                            <option>"Every 15 minutes"</option>
                            <option>"Every 30 minutes"</option>
                            <option>"Manual only"</option>
                        </select>
                    </div>
                </Card>

                <Card title=Some("Notifications".to_string())>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Low Stock Alerts"</span>
                            <span class="setting-description">"Get notified when items are low"</span>
                        </div>
                        <label class="toggle">
                            <input type="checkbox" checked />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Delivery Updates"</span>
                            <span class="setting-description">"Receive delivery status notifications"</span>
                        </div>
                        <label class="toggle">
                            <input type="checkbox" checked />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </Card>

                <Card title=Some("About".to_string())>
                    <dl class="about-list">
                        <dt>"Version"</dt><dd>"0.1.0"</dd>
                        <dt>"Build"</dt><dd>"2024.01.15"</dd>
                        <dt>"Runtime"</dt><dd>"Tauri v2 + Leptos"</dd>
                        <dt>"Database"</dt><dd>"SQLite + SQLCipher"</dd>
                    </dl>
                </Card>
            </div>
        </div>
    }
}

// =============================================================================
// FILE: frontend/src/pages/not_found.rs
// =============================================================================

//! 404 Not Found Page

use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="not-found-page">
            <div class="not-found-content">
                <span class="not-found-icon">"📦"</span>
                <h1>"404"</h1>
                <p>"Page not found"</p>
                <p class="not-found-message">
                    "The page you're looking for doesn't exist or has been moved."
                </p>
                <A href="/" attr:class="btn btn-primary">"Back to Dashboard"</A>
            </div>
        </div>
    }
}
