-- Timesheet and Workforce Management Tables

-- Time entries (clock in/out records)
CREATE TABLE IF NOT EXISTS time_entries (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    entry_date TEXT NOT NULL,
    clock_in_time TEXT NOT NULL,
    clock_out_time TEXT,
    clock_in_location_lat REAL,
    clock_in_location_lng REAL,
    clock_out_location_lat REAL,
    clock_out_location_lng REAL,
    clock_in_method TEXT NOT NULL DEFAULT 'biometric', -- biometric, manual, auto_geofence
    clock_out_method TEXT,
    clock_in_device TEXT,
    clock_out_device TEXT,
    break_duration_minutes INTEGER DEFAULT 0,
    total_hours REAL,
    overtime_hours REAL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'active', -- active, completed, edited, approved, rejected
    notes TEXT,
    edited_by TEXT,
    edited_reason TEXT,
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (edited_by) REFERENCES users(id),
    FOREIGN KEY (approved_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_time_entries_user ON time_entries(user_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_date ON time_entries(entry_date);
CREATE INDEX IF NOT EXISTS idx_time_entries_status ON time_entries(status);

-- Break records
CREATE TABLE IF NOT EXISTS time_breaks (
    id TEXT PRIMARY KEY,
    time_entry_id TEXT NOT NULL,
    break_type TEXT NOT NULL DEFAULT 'unpaid', -- paid, unpaid, meal
    start_time TEXT NOT NULL,
    end_time TEXT,
    duration_minutes INTEGER,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (time_entry_id) REFERENCES time_entries(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_time_breaks_entry ON time_breaks(time_entry_id);

-- Task/job codes for time allocation
CREATE TABLE IF NOT EXISTS job_codes (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    department TEXT,
    is_billable INTEGER DEFAULT 0,
    hourly_rate REAL,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Time allocation by job code
CREATE TABLE IF NOT EXISTS time_allocations (
    id TEXT PRIMARY KEY,
    time_entry_id TEXT NOT NULL,
    job_code_id TEXT NOT NULL,
    hours REAL NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (time_entry_id) REFERENCES time_entries(id) ON DELETE CASCADE,
    FOREIGN KEY (job_code_id) REFERENCES job_codes(id)
);

CREATE INDEX IF NOT EXISTS idx_time_allocations_entry ON time_allocations(time_entry_id);
CREATE INDEX IF NOT EXISTS idx_time_allocations_job ON time_allocations(job_code_id);

-- Pay periods
CREATE TABLE IF NOT EXISTS pay_periods (
    id TEXT PRIMARY KEY,
    period_name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open', -- open, closed, processing, paid
    closed_at TEXT,
    closed_by TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (closed_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_pay_periods_dates ON pay_periods(start_date, end_date);
CREATE INDEX IF NOT EXISTS idx_pay_periods_status ON pay_periods(status);

-- Timesheet summaries (pre-calculated for reporting)
CREATE TABLE IF NOT EXISTS timesheet_summaries (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    pay_period_id TEXT NOT NULL,
    regular_hours REAL DEFAULT 0,
    overtime_hours REAL DEFAULT 0,
    double_time_hours REAL DEFAULT 0,
    sick_hours REAL DEFAULT 0,
    vacation_hours REAL DEFAULT 0,
    holiday_hours REAL DEFAULT 0,
    total_hours REAL DEFAULT 0,
    total_breaks_minutes INTEGER DEFAULT 0,
    days_worked INTEGER DEFAULT 0,
    late_arrivals INTEGER DEFAULT 0,
    early_departures INTEGER DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'draft', -- draft, submitted, approved, rejected
    submitted_at TEXT,
    approved_by TEXT,
    approved_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (pay_period_id) REFERENCES pay_periods(id),
    FOREIGN KEY (approved_by) REFERENCES users(id),
    UNIQUE(user_id, pay_period_id)
);

CREATE INDEX IF NOT EXISTS idx_timesheet_summaries_user ON timesheet_summaries(user_id);
CREATE INDEX IF NOT EXISTS idx_timesheet_summaries_period ON timesheet_summaries(pay_period_id);

-- Schedule templates
CREATE TABLE IF NOT EXISTS schedule_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    day_of_week INTEGER NOT NULL, -- 0=Sunday, 6=Saturday
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    break_duration_minutes INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- User schedules
CREATE TABLE IF NOT EXISTS user_schedules (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    schedule_date TEXT NOT NULL,
    scheduled_start TEXT NOT NULL,
    scheduled_end TEXT NOT NULL,
    department TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    UNIQUE(user_id, schedule_date)
);

CREATE INDEX IF NOT EXISTS idx_user_schedules_user ON user_schedules(user_id);
CREATE INDEX IF NOT EXISTS idx_user_schedules_date ON user_schedules(schedule_date);

