-- Synchronization and CRDT Tables

-- Sync status tracking
CREATE TABLE IF NOT EXISTS sync_status (
    id TEXT PRIMARY KEY,
    table_name TEXT NOT NULL UNIQUE,
    last_sync_at TEXT,
    last_sync_version INTEGER DEFAULT 0,
    pending_changes INTEGER DEFAULT 0,
    sync_errors INTEGER DEFAULT 0,
    last_error TEXT,
    last_error_at TEXT,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert sync tracking for all major tables
INSERT OR IGNORE INTO sync_status (id, table_name) VALUES
    ('sync_inventory', 'inventory_items'),
    ('sync_stock', 'inventory_stock'),
    ('sync_shipments', 'shipments'),
    ('sync_receipts', 'receipts'),
    ('sync_deliveries', 'deliveries'),
    ('sync_customers', 'customers'),
    ('sync_time_entries', 'time_entries');

-- Pending sync operations (outbox pattern)
CREATE TABLE IF NOT EXISTS sync_outbox (
    id TEXT PRIMARY KEY,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- INSERT, UPDATE, DELETE
    payload TEXT NOT NULL, -- JSON of the record
    version INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    sent_at TEXT,
    acknowledged_at TEXT,
    retry_count INTEGER DEFAULT 0,
    last_error TEXT
);

CREATE INDEX IF NOT EXISTS idx_sync_outbox_pending ON sync_outbox(sent_at) WHERE sent_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_sync_outbox_table ON sync_outbox(table_name);

-- Incoming sync operations (inbox)
CREATE TABLE IF NOT EXISTS sync_inbox (
    id TEXT PRIMARY KEY,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    operation TEXT NOT NULL,
    payload TEXT NOT NULL,
    server_version INTEGER NOT NULL,
    received_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    applied_at TEXT,
    conflict_resolved INTEGER DEFAULT 0,
    resolution_strategy TEXT
);

CREATE INDEX IF NOT EXISTS idx_sync_inbox_pending ON sync_inbox(applied_at) WHERE applied_at IS NULL;

-- CRDT document storage (Automerge)
CREATE TABLE IF NOT EXISTS crdt_documents (
    id TEXT PRIMARY KEY,
    document_type TEXT NOT NULL, -- inventory, shipment, etc.
    record_id TEXT NOT NULL,
    actor_id TEXT NOT NULL, -- Device/user identifier
    heads TEXT NOT NULL, -- JSON array of head hashes
    compressed_changes BLOB NOT NULL, -- Automerge binary format
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(document_type, record_id)
);

CREATE INDEX IF NOT EXISTS idx_crdt_documents_type ON crdt_documents(document_type);
CREATE INDEX IF NOT EXISTS idx_crdt_documents_record ON crdt_documents(record_id);

-- CRDT change history (for debugging and conflict resolution)
CREATE TABLE IF NOT EXISTS crdt_changes (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    change_hash TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    seq_number INTEGER NOT NULL,
    timestamp TEXT NOT NULL,
    operation_summary TEXT, -- Human-readable summary
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES crdt_documents(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_crdt_changes_document ON crdt_changes(document_id);
CREATE INDEX IF NOT EXISTS idx_crdt_changes_actor ON crdt_changes(actor_id);

-- Conflict log
CREATE TABLE IF NOT EXISTS sync_conflicts (
    id TEXT PRIMARY KEY,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    local_version TEXT NOT NULL, -- JSON
    remote_version TEXT NOT NULL, -- JSON
    resolution TEXT, -- JSON of resolved record
    resolution_strategy TEXT, -- merge, local_wins, remote_wins, manual
    resolved_by TEXT,
    resolved_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (resolved_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_sync_conflicts_table ON sync_conflicts(table_name);
CREATE INDEX IF NOT EXISTS idx_sync_conflicts_unresolved ON sync_conflicts(resolved_at) WHERE resolved_at IS NULL;

-- Device registry (for multi-device sync)
CREATE TABLE IF NOT EXISTS sync_devices (
    id TEXT PRIMARY KEY,
    device_name TEXT NOT NULL,
    device_type TEXT, -- mobile, tablet, desktop
    platform TEXT, -- ios, android, windows, macos
    last_sync_at TEXT,
    last_seen_at TEXT,
    push_token TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Network connectivity log
CREATE TABLE IF NOT EXISTS connectivity_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    status TEXT NOT NULL, -- online, offline, slow
    connection_type TEXT, -- wifi, cellular, ethernet
    signal_strength INTEGER,
    recorded_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_connectivity_log_time ON connectivity_log(recorded_at);

