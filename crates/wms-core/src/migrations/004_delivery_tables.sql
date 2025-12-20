-- Delivery and Logistics Tables

-- Delivery routes
CREATE TABLE IF NOT EXISTS delivery_routes (
    id TEXT PRIMARY KEY,
    route_name TEXT NOT NULL,
    route_date TEXT NOT NULL,
    driver_id TEXT,
    vehicle_id TEXT,
    status TEXT NOT NULL DEFAULT 'planning', -- planning, assigned, in_progress, completed, cancelled
    start_location_lat REAL,
    start_location_lng REAL,
    end_location_lat REAL,
    end_location_lng REAL,
    planned_start_time TEXT,
    actual_start_time TEXT,
    planned_end_time TEXT,
    actual_end_time TEXT,
    total_distance_km REAL,
    total_duration_minutes INTEGER,
    optimization_score REAL, -- Quality score from VRP solver
    notes TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT,
    FOREIGN KEY (driver_id) REFERENCES users(id),
    FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_routes_date ON delivery_routes(route_date);
CREATE INDEX IF NOT EXISTS idx_routes_driver ON delivery_routes(driver_id);
CREATE INDEX IF NOT EXISTS idx_routes_status ON delivery_routes(status);

-- Individual deliveries/stops
CREATE TABLE IF NOT EXISTS deliveries (
    id TEXT PRIMARY KEY,
    delivery_number TEXT NOT NULL UNIQUE,
    route_id TEXT,
    shipment_id TEXT,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, assigned, en_route, arrived, delivered, failed, returned
    sequence_number INTEGER, -- Order in route
    customer_id TEXT,
    delivery_name TEXT NOT NULL,
    delivery_address_line1 TEXT NOT NULL,
    delivery_address_line2 TEXT,
    delivery_city TEXT NOT NULL,
    delivery_state TEXT NOT NULL,
    delivery_postal_code TEXT NOT NULL,
    delivery_country TEXT NOT NULL DEFAULT 'US',
    delivery_phone TEXT,
    delivery_email TEXT,
    latitude REAL,
    longitude REAL,
    geofence_radius_meters REAL DEFAULT 100,
    scheduled_date TEXT NOT NULL,
    scheduled_time_window_start TEXT,
    scheduled_time_window_end TEXT,
    estimated_arrival_time TEXT,
    actual_arrival_time TEXT,
    actual_departure_time TEXT,
    delivery_instructions TEXT,
    signature_required INTEGER DEFAULT 0,
    signature_data BLOB,
    signature_name TEXT,
    proof_of_delivery_photo BLOB,
    delivery_notes TEXT,
    failure_reason TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT,
    FOREIGN KEY (route_id) REFERENCES delivery_routes(id),
    FOREIGN KEY (shipment_id) REFERENCES shipments(id),
    FOREIGN KEY (customer_id) REFERENCES customers(id)
);

CREATE INDEX IF NOT EXISTS idx_deliveries_number ON deliveries(delivery_number);
CREATE INDEX IF NOT EXISTS idx_deliveries_route ON deliveries(route_id);
CREATE INDEX IF NOT EXISTS idx_deliveries_status ON deliveries(status);
CREATE INDEX IF NOT EXISTS idx_deliveries_date ON deliveries(scheduled_date);
CREATE INDEX IF NOT EXISTS idx_deliveries_customer ON deliveries(customer_id);

-- Delivery status history
CREATE TABLE IF NOT EXISTS delivery_status_history (
    id TEXT PRIMARY KEY,
    delivery_id TEXT NOT NULL,
    status TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    notes TEXT,
    recorded_by TEXT,
    recorded_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (delivery_id) REFERENCES deliveries(id) ON DELETE CASCADE,
    FOREIGN KEY (recorded_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_delivery_history_delivery ON delivery_status_history(delivery_id);

-- Geofence zones (warehouses, customer sites, etc.)
CREATE TABLE IF NOT EXISTS geofences (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    zone_type TEXT NOT NULL, -- warehouse, customer, restricted, etc.
    geometry_type TEXT NOT NULL DEFAULT 'circle', -- circle, polygon
    center_lat REAL,
    center_lng REAL,
    radius_meters REAL, -- For circle type
    polygon_coords TEXT, -- JSON array of [lat, lng] for polygon type
    trigger_on_enter INTEGER DEFAULT 1,
    trigger_on_exit INTEGER DEFAULT 1,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_geofences_type ON geofences(zone_type);

-- Driver location tracking
CREATE TABLE IF NOT EXISTS driver_locations (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    route_id TEXT,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    accuracy_meters REAL,
    speed_kmh REAL,
    heading REAL, -- Degrees from north
    altitude_meters REAL,
    recorded_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (route_id) REFERENCES delivery_routes(id)
);

CREATE INDEX IF NOT EXISTS idx_driver_locations_user ON driver_locations(user_id);
CREATE INDEX IF NOT EXISTS idx_driver_locations_route ON driver_locations(route_id);
CREATE INDEX IF NOT EXISTS idx_driver_locations_time ON driver_locations(recorded_at);

-- Vehicles
CREATE TABLE IF NOT EXISTS vehicles (
    id TEXT PRIMARY KEY,
    vehicle_number TEXT NOT NULL UNIQUE,
    name TEXT,
    vehicle_type TEXT NOT NULL, -- van, truck, car
    license_plate TEXT,
    capacity_kg REAL,
    capacity_m3 REAL, -- Volume capacity
    fuel_type TEXT,
    is_active INTEGER DEFAULT 1,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

