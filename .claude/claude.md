# Warehouse Management System (WMS)

A comprehensive offline-first warehouse management system built with Rust, Tauri v2, Leptos, and WebAssembly.

## Architecture

### Tech Stack
- **Backend**: Tauri v2 (Rust) with native commands
- **Frontend**: Leptos 0.7 (Rust → WASM) with CSR
- **Build Tool**: Trunk for WASM compilation
- **Database**: SQLite with SQLCipher encryption (offline-first)
- **Sync**: Automerge CRDTs for conflict-free multi-device sync

### Workspace Structure
```
app-warehouse/
├── frontend/              # Leptos WASM frontend
│   └── src/
│       ├── components/    # Reusable UI components
│       ├── pages/         # Route pages
│       ├── api.rs         # Tauri invoke bindings
│       └── state.rs       # Global app state
├── src-tauri/             # Tauri v2 backend
│   └── src/
│       ├── commands/      # IPC command handlers
│       ├── lib.rs         # Plugin/capability setup
│       └── state.rs       # App state management
└── crates/                # Domain logic crates
    ├── wms-core/          # Shared types, DB, errors
    ├── wms-inventory/     # Stock management, forecasting (augurs)
    ├── wms-shipping/      # Labels, barcodes (rxing, printpdf)
    ├── wms-deliveries/    # Route optimization (vrp-core), geofencing
    ├── wms-crm/           # Customer management, phone validation
    ├── wms-timesheets/    # Time tracking, Excel export (rust_xlsxwriter)
    └── wms-sync/          # CRDT sync engine (automerge)
```

## Development

### Commands
```bash
# Development (runs Trunk + Tauri)
cargo tauri dev

# Build for production
cargo tauri build

# Build frontend only
trunk build --release

# Type check workspace
cargo check --workspace
```

### Dev URLs
- **Frontend dev server**: http://localhost:1420
- **Tauri bundles to**: `dist/`

## Modules

| Module | Description | Key Dependencies |
|--------|-------------|------------------|
| Inventory | Stock levels, locations, demand forecasting | augurs |
| Shipping | Barcode generation, shipping labels | rxing, printpdf |
| Deliveries | Route optimization, geofencing | vrp-core, vrp-pragmatic, geo |
| CRM | Customer records, phone validation | phonenumber, validator |
| Timesheets | Time tracking, Excel/CSV export | rust_xlsxwriter, csv |
| Sync | Offline-first CRDT sync | automerge |

## Conventions

- **Edition**: Rust 2024
- **Error handling**: `thiserror` for library errors, `anyhow` for app errors
- **Async**: Tokio runtime
- **Serialization**: Serde JSON for IPC, `serde-wasm-bindgen` for frontend

## Current State

The project has a working structure with:
- All domain crates implemented with models and services
- Tauri commands wired up for each module
- Leptos frontend with pages for all modules (dashboard, inventory, shipping, receiving, deliveries, customers, timesheets, settings)
- Component library (button, input, modal, card, badge, chart, data_table, toast, sidebar, header)

### Mobile Support
- iOS: Minimum iOS 15.0
- Android: Minimum SDK 26

### Plugins Configured
- File system access
- Shell (open)
- Barcode scanner (camera)
- Biometric auth
- Geolocation
- Notifications
