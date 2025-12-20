-- Shipping and Receiving Tables

-- Carriers/Shipping providers
CREATE TABLE IF NOT EXISTS carriers (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    contact_email TEXT,
    contact_phone TEXT,
    tracking_url_template TEXT, -- URL with {tracking_number} placeholder
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert common carriers
INSERT OR IGNORE INTO carriers (id, code, name, tracking_url_template) VALUES
    ('car_ups', 'UPS', 'United Parcel Service', 'https://www.ups.com/track?tracknum={tracking_number}'),
    ('car_fedex', 'FEDEX', 'FedEx', 'https://www.fedex.com/fedextrack/?trknbr={tracking_number}'),
    ('car_usps', 'USPS', 'USPS', 'https://tools.usps.com/go/TrackConfirmAction?qtc_tLabels1={tracking_number}'),
    ('car_dhl', 'DHL', 'DHL', 'https://www.dhl.com/en/express/tracking.html?AWB={tracking_number}');

-- Outbound shipments
CREATE TABLE IF NOT EXISTS shipments (
    id TEXT PRIMARY KEY,
    shipment_number TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL DEFAULT 'draft', -- draft, confirmed, picking, packed, shipped, delivered, cancelled
    order_reference TEXT,
    customer_id TEXT,
    carrier_id TEXT,
    service_type TEXT, -- ground, express, overnight, etc.
    tracking_number TEXT,
    ship_date TEXT,
    expected_delivery_date TEXT,
    actual_delivery_date TEXT,
    ship_to_name TEXT NOT NULL,
    ship_to_address_line1 TEXT NOT NULL,
    ship_to_address_line2 TEXT,
    ship_to_city TEXT NOT NULL,
    ship_to_state TEXT NOT NULL,
    ship_to_postal_code TEXT NOT NULL,
    ship_to_country TEXT NOT NULL DEFAULT 'US',
    ship_to_phone TEXT,
    ship_to_email TEXT,
    total_weight_kg REAL,
    total_packages INTEGER DEFAULT 1,
    shipping_cost REAL,
    insurance_value REAL,
    special_instructions TEXT,
    label_printed INTEGER DEFAULT 0,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT,
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    FOREIGN KEY (carrier_id) REFERENCES carriers(id),
    FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_shipments_number ON shipments(shipment_number);
CREATE INDEX IF NOT EXISTS idx_shipments_status ON shipments(status);
CREATE INDEX IF NOT EXISTS idx_shipments_customer ON shipments(customer_id);
CREATE INDEX IF NOT EXISTS idx_shipments_date ON shipments(ship_date);

-- Shipment line items
CREATE TABLE IF NOT EXISTS shipment_items (
    id TEXT PRIMARY KEY,
    shipment_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    location_id TEXT,
    quantity_ordered REAL NOT NULL,
    quantity_picked REAL DEFAULT 0,
    quantity_shipped REAL DEFAULT 0,
    lot_number TEXT,
    serial_number TEXT,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, picking, picked, packed, shipped
    picked_by TEXT,
    picked_at TEXT,
    FOREIGN KEY (shipment_id) REFERENCES shipments(id) ON DELETE CASCADE,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id),
    FOREIGN KEY (location_id) REFERENCES locations(id),
    FOREIGN KEY (picked_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_shipment_items_shipment ON shipment_items(shipment_id);
CREATE INDEX IF NOT EXISTS idx_shipment_items_item ON shipment_items(item_id);

-- Shipment packages/cartons
CREATE TABLE IF NOT EXISTS shipment_packages (
    id TEXT PRIMARY KEY,
    shipment_id TEXT NOT NULL,
    package_number INTEGER NOT NULL,
    tracking_number TEXT,
    weight_kg REAL,
    length_cm REAL,
    width_cm REAL,
    height_cm REAL,
    package_type TEXT, -- box, envelope, pallet, etc.
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (shipment_id) REFERENCES shipments(id) ON DELETE CASCADE,
    UNIQUE(shipment_id, package_number)
);

-- Inbound receipts (receiving)
CREATE TABLE IF NOT EXISTS receipts (
    id TEXT PRIMARY KEY,
    receipt_number TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, receiving, completed, cancelled
    po_number TEXT, -- Purchase order reference
    supplier_name TEXT,
    supplier_reference TEXT,
    expected_date TEXT,
    received_date TEXT,
    dock_door TEXT,
    notes TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TEXT,
    completed_by TEXT,
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (completed_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_receipts_number ON receipts(receipt_number);
CREATE INDEX IF NOT EXISTS idx_receipts_status ON receipts(status);
CREATE INDEX IF NOT EXISTS idx_receipts_po ON receipts(po_number);

-- Receipt line items
CREATE TABLE IF NOT EXISTS receipt_items (
    id TEXT PRIMARY KEY,
    receipt_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    location_id TEXT, -- Target putaway location
    quantity_expected REAL NOT NULL,
    quantity_received REAL DEFAULT 0,
    quantity_damaged REAL DEFAULT 0,
    lot_number TEXT,
    expiry_date TEXT,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, partial, complete, damaged
    received_by TEXT,
    received_at TEXT,
    notes TEXT,
    FOREIGN KEY (receipt_id) REFERENCES receipts(id) ON DELETE CASCADE,
    FOREIGN KEY (item_id) REFERENCES inventory_items(id),
    FOREIGN KEY (location_id) REFERENCES locations(id),
    FOREIGN KEY (received_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_receipt_items_receipt ON receipt_items(receipt_id);
CREATE INDEX IF NOT EXISTS idx_receipt_items_item ON receipt_items(item_id);

-- Shipping labels (stored as ZPL or PDF)
CREATE TABLE IF NOT EXISTS shipping_labels (
    id TEXT PRIMARY KEY,
    shipment_id TEXT NOT NULL,
    package_id TEXT,
    label_type TEXT NOT NULL, -- shipping, pallet, carton
    label_format TEXT NOT NULL, -- zpl, pdf, png
    label_data BLOB NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (shipment_id) REFERENCES shipments(id),
    FOREIGN KEY (package_id) REFERENCES shipment_packages(id)
);

