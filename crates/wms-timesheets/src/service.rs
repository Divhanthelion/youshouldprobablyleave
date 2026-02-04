//! Timesheet Service
//! 
//! Core business logic for time tracking and workforce management.

use std::sync::Arc;
use base64::Engine;
use chrono::{Utc, NaiveDate, Datelike};
use rusqlite::params;
use tracing::{info, debug, warn};
use wms_core::db::Database;
use wms_core::error::{WmsError, Result};
use wms_core::types::new_id;
use crate::models::*;
use crate::export::{ExcelExporter, CsvExporter, TimesheetExport};

/// Timesheet service
pub struct TimesheetService {
    db: Arc<Database>,
    /// Standard work day hours (for overtime calculation)
    standard_hours: f64,
    /// Weekly overtime threshold
    weekly_overtime_threshold: f64,
}

impl TimesheetService {
    /// Create a new timesheet service
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            standard_hours: 8.0,
            weekly_overtime_threshold: 40.0,
        }
    }
    
    /// Configure overtime thresholds
    pub fn with_overtime_config(mut self, daily: f64, weekly: f64) -> Self {
        self.standard_hours = daily;
        self.weekly_overtime_threshold = weekly;
        self
    }
    
    /// Clock in for a user
    pub async fn clock_in(&self, user_id: &str) -> Result<TimeEntry> {
        // Check for existing open entry
        let existing = self.get_active_entry(user_id).await?;
        if existing.is_some() {
            return Err(WmsError::validation("User is already clocked in"));
        }
        
        let now = Utc::now();
        let entry = TimeEntry {
            id: new_id(),
            user_id: user_id.to_string(),
            entry_date: now.date_naive(),
            clock_in_time: now,
            clock_out_time: None,
            clock_in_location: None,
            clock_out_location: None,
            clock_in_method: ClockMethod::Biometric,
            clock_out_method: None,
            clock_in_device: None,
            clock_out_device: None,
            break_duration_minutes: 0,
            total_hours: None,
            overtime_hours: 0.0,
            status: TimeEntryStatus::Active,
            notes: None,
            edited_by: None,
            edited_reason: None,
            approved_by: None,
            approved_at: None,
            created_at: now,
            updated_at: None,
            breaks: Vec::new(),
        };
        
        self.db.execute(
            "INSERT INTO time_entries (
                id, user_id, entry_date, clock_in_time, clock_in_method,
                status, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                &entry.id,
                &entry.user_id,
                entry.entry_date.to_string(),
                entry.clock_in_time.to_rfc3339(),
                "biometric",
                "active",
                entry.created_at.to_rfc3339(),
            ],
        )?;
        
        info!("User {} clocked in at {}", user_id, now);
        Ok(entry)
    }
    
    /// Clock out for a user
    pub async fn clock_out(&self, user_id: &str) -> Result<TimeEntry> {
        let mut entry = self.get_active_entry(user_id).await?
            .ok_or_else(|| WmsError::validation("User is not clocked in"))?;
        
        let now = Utc::now();
        entry.clock_out_time = Some(now);
        entry.clock_out_method = Some(ClockMethod::Biometric);
        entry.status = TimeEntryStatus::Completed;
        entry.total_hours = entry.calculate_hours();
        entry.updated_at = Some(now);
        
        // Calculate overtime
        if let Some(hours) = entry.total_hours {
            if hours > self.standard_hours {
                entry.overtime_hours = hours - self.standard_hours;
            }
        }
        
        self.db.execute(
            "UPDATE time_entries SET
                clock_out_time = ?, clock_out_method = ?, status = ?,
                total_hours = ?, overtime_hours = ?, updated_at = ?
             WHERE id = ?",
            params![
                entry.clock_out_time.map(|t| t.to_rfc3339()),
                "biometric",
                "completed",
                entry.total_hours,
                entry.overtime_hours,
                entry.updated_at.map(|t| t.to_rfc3339()),
                &entry.id,
            ],
        )?;
        
        info!("User {} clocked out at {}, worked {:.2} hours", 
              user_id, now, entry.total_hours.unwrap_or(0.0));
        
        Ok(entry)
    }
    
    /// Get active (clocked in) entry for user
    async fn get_active_entry(&self, user_id: &str) -> Result<Option<TimeEntry>> {
        self.db.query_row(
            "SELECT * FROM time_entries 
             WHERE user_id = ? AND status = 'active' AND clock_out_time IS NULL
             ORDER BY clock_in_time DESC LIMIT 1",
            params![user_id],
            |row| Self::row_to_entry(row),
        )
    }
    
    /// Get timesheet for a user within a date range
    pub async fn get_timesheet(
        &self,
        user_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Timesheet> {
        let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|_| WmsError::validation("Invalid start date format"))?;
        let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|_| WmsError::validation("Invalid end date format"))?;
        
        // Get time entries
        let entries = self.db.query_map(
            "SELECT * FROM time_entries 
             WHERE user_id = ? AND entry_date >= ? AND entry_date <= ?
             ORDER BY entry_date, clock_in_time",
            params![user_id, start.to_string(), end.to_string()],
            |row| Self::row_to_entry(row),
        )?;
        
        // Calculate summary
        let mut regular_hours = 0.0;
        let mut overtime_hours = 0.0;
        let mut total_breaks = 0u32;
        let mut days_worked = std::collections::HashSet::new();
        
        for entry in &entries {
            if let Some(hours) = entry.total_hours {
                let regular = hours.min(self.standard_hours);
                let overtime = (hours - self.standard_hours).max(0.0);
                
                regular_hours += regular;
                overtime_hours += overtime;
            }
            total_breaks += entry.break_duration_minutes;
            days_worked.insert(entry.entry_date);
        }
        
        // Get user name
        let user_name: String = self.db.query_row(
            "SELECT full_name FROM users WHERE id = ?",
            params![user_id],
            |row| row.get(0),
        )?.unwrap_or_else(|| "Unknown".to_string());
        
        Ok(Timesheet {
            user_id: user_id.to_string(),
            user_name,
            start_date: start,
            end_date: end,
            regular_hours,
            overtime_hours,
            double_time_hours: 0.0, // Could calculate based on rules
            sick_hours: 0.0,
            vacation_hours: 0.0,
            holiday_hours: 0.0,
            total_hours: regular_hours + overtime_hours,
            total_breaks_minutes: total_breaks,
            days_worked: days_worked.len() as u32,
            late_arrivals: 0, // Would compare with schedule
            early_departures: 0,
            status: TimesheetStatus::Draft,
            submitted_at: None,
            approved_by: None,
            approved_at: None,
            notes: None,
            entries,
        })
    }
    
    /// Export timesheet to file format
    pub async fn export_timesheet(
        &self,
        user_id: &str,
        start_date: &str,
        end_date: &str,
        format: &str,
    ) -> Result<TimesheetExport> {
        let timesheet = self.get_timesheet(user_id, start_date, end_date).await?;
        
        let (data, content_type, filename) = match format.to_lowercase().as_str() {
            "xlsx" | "excel" => {
                let data = ExcelExporter::export(&timesheet)?;
                let filename = format!(
                    "timesheet_{}_{}_to_{}.xlsx",
                    user_id, start_date, end_date
                );
                (data, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", filename)
            }
            "csv" => {
                let data = CsvExporter::export(&timesheet)?;
                let filename = format!(
                    "timesheet_{}_{}_to_{}.csv",
                    user_id, start_date, end_date
                );
                (data, "text/csv", filename)
            }
            _ => {
                return Err(WmsError::validation(format!("Unsupported format: {}", format)));
            }
        };
        
        info!("Exported timesheet for {} in {} format", user_id, format);
        
        Ok(TimesheetExport {
            data: base64::engine::general_purpose::STANDARD.encode(&data),
            content_type: content_type.to_string(),
            filename,
        })
    }
    
    /// Start a break
    pub async fn start_break(&self, user_id: &str, break_type: BreakType) -> Result<TimeBreak> {
        let entry = self.get_active_entry(user_id).await?
            .ok_or_else(|| WmsError::validation("User is not clocked in"))?;
        
        let now = Utc::now();
        let time_break = TimeBreak {
            id: new_id(),
            time_entry_id: entry.id.clone(),
            break_type,
            start_time: now,
            end_time: None,
            duration_minutes: None,
            notes: None,
            created_at: now,
        };
        
        self.db.execute(
            "INSERT INTO time_breaks (
                id, time_entry_id, break_type, start_time, created_at
            ) VALUES (?, ?, ?, ?, ?)",
            params![
                &time_break.id,
                &time_break.time_entry_id,
                format!("{:?}", break_type).to_lowercase(),
                time_break.start_time.to_rfc3339(),
                time_break.created_at.to_rfc3339(),
            ],
        )?;
        
        debug!("User {} started {:?} break", user_id, break_type);
        Ok(time_break)
    }
    
    /// End a break
    pub async fn end_break(&self, user_id: &str) -> Result<TimeBreak> {
        let entry = self.get_active_entry(user_id).await?
            .ok_or_else(|| WmsError::validation("User is not clocked in"))?;
        
        // Find active break
        let mut time_break: TimeBreak = self.db.query_row(
            "SELECT * FROM time_breaks 
             WHERE time_entry_id = ? AND end_time IS NULL
             ORDER BY start_time DESC LIMIT 1",
            params![&entry.id],
            |row| {
                Ok(TimeBreak {
                    id: row.get("id")?,
                    time_entry_id: row.get("time_entry_id")?,
                    break_type: BreakType::Unpaid,
                    start_time: Utc::now(),
                    end_time: None,
                    duration_minutes: None,
                    notes: row.get("notes")?,
                    created_at: Utc::now(),
                })
            },
        )?.ok_or_else(|| WmsError::validation("No active break found"))?;
        
        let now = Utc::now();
        time_break.end_time = Some(now);
        time_break.duration_minutes = time_break.calculate_duration();
        
        self.db.execute(
            "UPDATE time_breaks SET end_time = ?, duration_minutes = ? WHERE id = ?",
            params![
                now.to_rfc3339(),
                time_break.duration_minutes,
                &time_break.id,
            ],
        )?;
        
        // Update total break time on entry
        if let Some(duration) = time_break.duration_minutes {
            self.db.execute(
                "UPDATE time_entries SET break_duration_minutes = break_duration_minutes + ? WHERE id = ?",
                params![duration, &entry.id],
            )?;
        }
        
        debug!("User {} ended break, duration: {:?} minutes", 
               user_id, time_break.duration_minutes);
        
        Ok(time_break)
    }
    
    fn row_to_entry(row: &rusqlite::Row) -> rusqlite::Result<TimeEntry> {
        Ok(TimeEntry {
            id: row.get("id")?,
            user_id: row.get("user_id")?,
            entry_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // Parse from string
            clock_in_time: Utc::now(), // Parse from string
            clock_out_time: None,
            clock_in_location: None,
            clock_out_location: None,
            clock_in_method: ClockMethod::Biometric,
            clock_out_method: None,
            clock_in_device: row.get("clock_in_device")?,
            clock_out_device: row.get("clock_out_device")?,
            break_duration_minutes: row.get::<_, u32>("break_duration_minutes").unwrap_or(0),
            total_hours: row.get("total_hours")?,
            overtime_hours: row.get::<_, f64>("overtime_hours").unwrap_or(0.0),
            status: TimeEntryStatus::Active,
            notes: row.get("notes")?,
            edited_by: row.get("edited_by")?,
            edited_reason: row.get("edited_reason")?,
            approved_by: row.get("approved_by")?,
            approved_at: None,
            created_at: Utc::now(),
            updated_at: None,
            breaks: Vec::new(),
        })
    }
}

