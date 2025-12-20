//! WMS Shipping Module
//! 
//! Provides shipping and receiving functionality including:
//! - Outbound shipment management
//! - Inbound receipt processing
//! - Barcode scanning and decoding
//! - ZPL label generation for thermal printers
//! - PDF document generation

mod models;
mod service;
mod barcode;
mod labels;

pub use models::*;
pub use service::ShippingService;
pub use barcode::{BarcodeDecoder, BarcodeResult};
pub use labels::{ZplLabel, PdfGenerator};

