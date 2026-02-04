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

