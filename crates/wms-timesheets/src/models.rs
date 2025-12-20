//! Timesheet Data Models

use chrono::{DateTime, NaiveDate, Utc, Duration};
use serde::{Deserialize, Serialize};

/// Time entry (clock in/out record)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: String,
    pub user_id: String,
    pub entry_date: NaiveDate,
    pub clock_in_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_out_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_in_location: Option<GeoLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_out_location: Option<GeoLocation>,
    pub clock_in_method: ClockMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_out_method: Option<ClockMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_in_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_out_device: Option<String>,
    #[serde(default)]
    pub break_duration_minutes: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_hours: Option<f64>,
    #[serde(default)]
    pub overtime_hours: f64,
    pub status: TimeEntryStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// Break records
    #[serde(default)]
    pub breaks: Vec<TimeBreak>,
}

impl TimeEntry {
    /// Calculate total worked hours
    pub fn calculate_hours(&self) -> Option<f64> {
        let clock_out = self.clock_out_time?;
        let duration = clock_out.signed_duration_since(self.clock_in_time);
        let total_minutes = duration.num_minutes() as f64 - self.break_duration_minutes as f64;
        Some((total_minutes / 60.0).max(0.0))
    }
    
    /// Check if currently clocked in (no clock out)
    pub fn is_clocked_in(&self) -> bool {
        self.clock_out_time.is_none()
    }
}

/// Geographic location for clock events
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoLocation {
    pub lat: f64,
    pub lng: f64,
}

/// Clock in/out methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ClockMethod {
    #[default]
    Biometric,
    Manual,
    AutoGeofence,
    Badge,
    Pin,
}

/// Time entry status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TimeEntryStatus {
    #[default]
    Active,
    Completed,
    Edited,
    Approved,
    Rejected,
}

/// Break record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBreak {
    pub id: String,
    pub time_entry_id: String,
    pub break_type: BreakType,
    pub start_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TimeBreak {
    /// Calculate break duration
    pub fn calculate_duration(&self) -> Option<u32> {
        let end = self.end_time?;
        let duration = end.signed_duration_since(self.start_time);
        Some(duration.num_minutes() as u32)
    }
}

/// Break types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum BreakType {
    #[default]
    Unpaid,
    Paid,
    Meal,
    Rest,
}

/// Timesheet summary for a period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timesheet {
    pub user_id: String,
    pub user_name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub regular_hours: f64,
    pub overtime_hours: f64,
    pub double_time_hours: f64,
    pub sick_hours: f64,
    pub vacation_hours: f64,
    pub holiday_hours: f64,
    pub total_hours: f64,
    pub total_breaks_minutes: u32,
    pub days_worked: u32,
    pub late_arrivals: u32,
    pub early_departures: u32,
    pub status: TimesheetStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Individual time entries
    pub entries: Vec<TimeEntry>,
}

/// Timesheet status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TimesheetStatus {
    #[default]
    Draft,
    Submitted,
    Approved,
    Rejected,
}

/// Pay period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPeriod {
    pub id: String,
    pub period_name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: PayPeriodStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Pay period status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PayPeriodStatus {
    #[default]
    Open,
    Closed,
    Processing,
    Paid,
}

/// User schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSchedule {
    pub id: String,
    pub user_id: String,
    pub schedule_date: NaiveDate,
    pub scheduled_start: String, // HH:MM format
    pub scheduled_end: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl UserSchedule {
    /// Check if a clock-in time is late
    pub fn is_late(&self, clock_in: &DateTime<Utc>) -> bool {
        // Parse scheduled start time
        let parts: Vec<&str> = self.scheduled_start.split(':').collect();
        if parts.len() != 2 {
            return false;
        }
        
        let hour: u32 = parts[0].parse().unwrap_or(0);
        let minute: u32 = parts[1].parse().unwrap_or(0);
        
        let scheduled = clock_in
            .date_naive()
            .and_hms_opt(hour, minute, 0)
            .map(|dt| dt.and_utc());
        
        match scheduled {
            Some(s) => clock_in > &s,
            None => false,
        }
    }
}

