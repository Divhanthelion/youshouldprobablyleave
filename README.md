# wms-rust

An offline-first warehouse management system built in Rust using Tauri v2 and WebAssembly.

## Features

### Core Modules

- **Inventory Management**
  - Real-time stock tracking with CRDT-based conflict resolution
  - Demand forecasting using time series analysis
  - ABC classification and reorder point management
  - Barcode scanning support (EAN-13, UPC, CODE-128, QR)

- **Shipping & Receiving**
  - Outbound shipment management with pick/pack workflow
  - Inbound receipt processing
  - ZPL label generation for thermal printers
  - PDF document generation (packing slips, invoices)
  - Multi-carrier support (UPS, FedEx, USPS, DHL)

- **Deliveries & Logistics**
  - Route optimization using Vehicle Routing Problem (VRP) solver
  - Geofencing for automatic arrival detection
  - Real-time driver location tracking
  - MapLibre-rs integration for offline mapping

- **Customer Relationship Management (CRM)**
  - Customer and supplier management
  - Contact management
  - International phone validation
  - Email validation

- **Timesheets & Workforce**
  - Biometric clock in/out
  - Break tracking
  - Overtime calculation
  - Excel/CSV export

### Technical Overview

- **Offline-First Architecture**: Full functionality without internet connectivity using SQLite + SQLCipher encryption
- **Sync**: Conflict-free data synchronization using Automerge
- **Performance**: Rust backend with Tauri v2
- **Cross-Platform**: Runs on Windows, macOS, Linux, iOS, and Android
- **UI**: Leptos-based reactive frontend compiled to WebAssembly

## Prerequisites

- Rust 1.75+ (2024 edition)
- Node.js 18+ (for build tools)
- Trunk (for Wasm bundling): `cargo install trunk`
- Tauri CLI: `cargo install tauri-cli`

### Platform-Specific

**Windows:**
- Visual Studio Build Tools 2022
- WebView2 (included in Windows 11)

**macOS:**
- Xcode Command Line Tools
- For iOS: Xcode 15+

**Linux:**
- WebKitGTK 4.1+
- `libssl-dev`, `libgtk-3-dev`

## Installation

```bash
# Clone the repository
git clone https://github.com/warehouse/wms-rust
cd wms-rust

# Install dependencies
cargo fetch

# Run in development mode
cargo tauri dev

# Build for production
cargo tauri build
```

## Project Structure

```
wms-rust/
├── src-tauri/           # Tauri backend
│   ├── src/
│   │   ├── commands/    # Tauri command handlers
│   │   ├── lib.rs       # Application entry
│   │   └── state.rs     # App state management
│   ├── Cargo.toml
│   └── tauri.conf.json
├── crates/              # Rust library crates
│   ├── wms-core/        # Database, types, errors
│   ├── wms-sync/        # CRDT sync engine
│   ├── wms-inventory/   # Inventory module
│   ├── wms-shipping/    # Shipping/receiving
│   ├── wms-deliveries/  # Logistics & routing
│   ├── wms-crm/         # Customer management
│   └── wms-timesheets/  # Workforce management
├── src/                 # Leptos frontend
│   ├── components/      # UI components
│   ├── pages/           # Page components
│   ├── api.rs           # Tauri bindings
│   └── state.rs         # Frontend state
├── styles.css           # Application styles
├── index.html           # HTML entry point
├── Cargo.toml           # Workspace manifest
└── Trunk.toml           # Trunk configuration
```

## Configuration

### Environment Variables

```bash
# Database encryption key (required in production)
WMS_DB_KEY=your-secure-key-here

# Server URL for sync (optional)
WMS_SERVER_URL=https://api.warehouse.example.com
```

### Database

The application uses SQLite with SQLCipher encryption. The database is created automatically in the app data directory:

- **Windows:** `%APPDATA%/com.warehouse.wms/wms.db`
- **macOS:** `~/Library/Application Support/com.warehouse.wms/wms.db`
- **Linux:** `~/.config/com.warehouse.wms/wms.db`

## Mobile Development

### Android

```bash
# Initialize Android project
cargo tauri android init

# Run on connected device/emulator
cargo tauri android dev

# Build APK
cargo tauri android build
```

### iOS

```bash
# Initialize iOS project (requires macOS)
cargo tauri ios init

# Run in simulator
cargo tauri ios dev

# Build IPA
cargo tauri ios build
```

## Testing

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p wms-inventory

# Run with coverage
cargo llvm-cov --workspace
```

## Crate Dependencies

| Crate | Purpose |
|-------|---------|
| `tauri` | Application framework |
| `rusqlite` + `sqlcipher` | Encrypted database |
| `automerge` | CRDT for sync |
| `rxing` | Barcode scanning |
| `vrp-core` | Route optimization |
| `geo` | Geospatial analysis |
| `augurs` | Time series forecasting |
| `validator` | Data validation |
| `phonenumber` | Phone number validation |
| `printpdf` | PDF generation |
| `rust_xlsxwriter` | Excel export |
| `leptos` | Frontend framework |

## Security

- All data at rest is encrypted using SQLCipher (AES-256)
- Biometric authentication for time clock operations
- Supply chain security via `cargo-vet` and `cargo-deny`
- No unsafe code in application crates

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit changes (`git commit -m 'Add new feature'`)
4. Push to branch (`git push origin feature/new-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - Application framework
- [Leptos](https://leptos.dev/) - Reactive web framework
- [Automerge](https://automerge.org/) - CRDT library
- [rxing](https://github.com/rxing-core/rxing) - Barcode library
