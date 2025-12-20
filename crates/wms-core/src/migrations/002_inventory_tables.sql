-- Inventory Management Tables

-- Product/Item master table
CREATE TABLE IF NOT EXISTS inventory_items (
    id TEXT PRIMARY KEY,
    sku TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    subcategory TEXT,
    unit_of_measure TEXT NOT NULL DEFAULT 'each',
    weight_kg REAL,
    length_cm REAL,
    width_cm REAL,
    height_cm REAL,
    barcode TEXT,
    barcode_type TEXT, -- EAN13, UPC, CODE128, etc.
    min_stock_level REAL DEFAULT 0,
    max_stock_level REAL,
    reorder_point REAL,
    reorder_quantity REAL,
    lead_time_days INTEGER DEFAULT 0,
    abc_class TEXT, -- A, B, C classification
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT,
    CONSTRAINT valid_abc_class CHECK (abc_class IN ('A', 'B', 'C') OR abc_class IS NULL)
);

CREATE INDEX IF NOT EXISTS idx_inventory_items_sku ON inventory_items(sku);
CREATE INDEX IF NOT EXISTS idx_inventory_items_barcode ON inventory_items(barcode);
CREATE INDEX IF NOT EXISTS idx_inventory_items_category ON inventory_items(category);

-- Warehouse locations
CREATE TABLE IF NOT EXISTS locations (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE, -- e.g., A-01-02 (Aisle-Rack-Level)
    zone TEXT NOT NULL, -- RECEIVING, STORAGE, PICKING, SHIPPING
    aisle TEXT,
    rack TEXT,
    level TEXT,
    bin TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    capacity_units REAL,
    current_units REAL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_locations_code ON locations(code);
CREATE INDEX IF NOT EXISTS idx_locations_zone ON locations(zone);

-- Current inventory levels by location
CREATE TABLE IF NOT EXISTS inventory_stock (
    id TEXT PRIMARY KEY,
    item_id TEXT NOT NULL,
    location_id TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 0,
    lot_number TEXT,
    expiry_date TEXT,
    serial_number TEXT,
    cost_per_unit REAL,
    last_count_date TEXT,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id),
    FOREIGN KEY (location_id) REFERENCES locations(id),
    UNIQUE(item_id, location_id, lot_number)
);

CREATE INDEX IF NOT EXISTS idx_inventory_stock_item ON inventory_stock(item_id);
CREATE INDEX IF NOT EXISTS idx_inventory_stock_location ON inventory_stock(location_id);
CREATE INDEX IF NOT EXISTS idx_inventory_stock_lot ON inventory_stock(lot_number);

-- Inventory transactions (adjustments, movements, counts)
CREATE TABLE IF NOT EXISTS inventory_transactions (
    id TEXT PRIMARY KEY,
    item_id TEXT NOT NULL,
    location_id TEXT,
    transaction_type TEXT NOT NULL, -- RECEIVE, PICK, ADJUST, TRANSFER, COUNT
    quantity REAL NOT NULL,
    previous_quantity REAL,
    new_quantity REAL,
    reference_type TEXT, -- SHIPMENT, RECEIPT, ADJUSTMENT, etc.
    reference_id TEXT,
    lot_number TEXT,
    reason_code TEXT,
    notes TEXT,
    user_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id),
    FOREIGN KEY (location_id) REFERENCES locations(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_inventory_tx_item ON inventory_transactions(item_id);
CREATE INDEX IF NOT EXISTS idx_inventory_tx_date ON inventory_transactions(created_at);
CREATE INDEX IF NOT EXISTS idx_inventory_tx_type ON inventory_transactions(transaction_type);

-- Inventory forecasts (precomputed)
CREATE TABLE IF NOT EXISTS inventory_forecasts (
    id TEXT PRIMARY KEY,
    item_id TEXT NOT NULL,
    forecast_date TEXT NOT NULL,
    predicted_demand REAL NOT NULL,
    confidence_lower REAL,
    confidence_upper REAL,
    model_type TEXT, -- ETS, ARIMA, etc.
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id),
    UNIQUE(item_id, forecast_date)
);

CREATE INDEX IF NOT EXISTS idx_forecasts_item ON inventory_forecasts(item_id);
CREATE INDEX IF NOT EXISTS idx_forecasts_date ON inventory_forecasts(forecast_date);

