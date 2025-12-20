//! Timesheet Export Functionality
//! 
//! Exports timesheets to Excel (XLSX) and CSV formats.

use rust_xlsxwriter::{Workbook, Format, FormatAlign, FormatBorder};
use csv::Writer;
use serde::{Deserialize, Serialize};
use wms_core::error::{WmsError, Result};
use crate::models::Timesheet;

/// Exported timesheet data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetExport {
    /// Base64 encoded file data
    pub data: String,
    /// MIME content type
    pub content_type: String,
    /// Suggested filename
    pub filename: String,
}

/// Excel exporter for timesheets
pub struct ExcelExporter;

impl ExcelExporter {
    /// Export timesheet to XLSX format
    pub fn export(timesheet: &Timesheet) -> Result<Vec<u8>> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        
        // Define formats
        let header_format = Format::new()
            .set_bold()
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin);
        
        let date_format = Format::new()
            .set_num_format("yyyy-mm-dd");
        
        let time_format = Format::new()
            .set_num_format("hh:mm");
        
        let hours_format = Format::new()
            .set_num_format("0.00");
        
        // Write header
        worksheet.set_column_width(0, 15).ok();
        worksheet.set_column_width(1, 12).ok();
        worksheet.set_column_width(2, 12).ok();
        worksheet.set_column_width(3, 10).ok();
        worksheet.set_column_width(4, 10).ok();
        worksheet.set_column_width(5, 12).ok();
        
        // Title
        worksheet.write_string(0, 0, "Timesheet Report").ok();
        worksheet.write_string(1, 0, &format!("Employee: {}", timesheet.user_name)).ok();
        worksheet.write_string(2, 0, &format!("Period: {} to {}", 
            timesheet.start_date, timesheet.end_date)).ok();
        
        // Column headers
        let headers = ["Date", "Clock In", "Clock Out", "Break (min)", "Hours", "Overtime", "Status"];
        for (col, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(4, col as u16, header, &header_format).ok();
        }
        
        // Data rows
        let mut row = 5u32;
        for entry in &timesheet.entries {
            worksheet.write_string(row, 0, &entry.entry_date.to_string()).ok();
            worksheet.write_string(row, 1, &entry.clock_in_time.format("%H:%M").to_string()).ok();
            
            if let Some(clock_out) = entry.clock_out_time {
                worksheet.write_string(row, 2, &clock_out.format("%H:%M").to_string()).ok();
            }
            
            worksheet.write_number(row, 3, entry.break_duration_minutes as f64).ok();
            
            if let Some(hours) = entry.total_hours {
                worksheet.write_number_with_format(row, 4, hours, &hours_format).ok();
            }
            
            worksheet.write_number_with_format(row, 5, entry.overtime_hours, &hours_format).ok();
            worksheet.write_string(row, 6, &format!("{:?}", entry.status)).ok();
            
            row += 1;
        }
        
        // Summary section
        row += 2;
        worksheet.write_string(row, 0, "Summary").ok();
        row += 1;
        worksheet.write_string(row, 0, "Regular Hours:").ok();
        worksheet.write_number_with_format(row, 1, timesheet.regular_hours, &hours_format).ok();
        row += 1;
        worksheet.write_string(row, 0, "Overtime Hours:").ok();
        worksheet.write_number_with_format(row, 1, timesheet.overtime_hours, &hours_format).ok();
        row += 1;
        worksheet.write_string(row, 0, "Total Hours:").ok();
        worksheet.write_number_with_format(row, 1, timesheet.total_hours, &hours_format).ok();
        row += 1;
        worksheet.write_string(row, 0, "Days Worked:").ok();
        worksheet.write_number(row, 1, timesheet.days_worked as f64).ok();
        
        // Save to buffer
        let buffer = workbook.save_to_buffer()
            .map_err(|e| WmsError::Export(format!("Failed to create Excel file: {}", e)))?;
        
        Ok(buffer)
    }
}

/// CSV exporter for timesheets
pub struct CsvExporter;

impl CsvExporter {
    /// Export timesheet to CSV format
    pub fn export(timesheet: &Timesheet) -> Result<Vec<u8>> {
        let mut writer = Writer::from_writer(Vec::new());
        
        // Write header
        writer.write_record(&[
            "Date",
            "Clock In",
            "Clock Out",
            "Break (min)",
            "Hours",
            "Overtime",
            "Status",
        ]).map_err(|e| WmsError::Export(format!("CSV write error: {}", e)))?;
        
        // Write data rows
        for entry in &timesheet.entries {
            let clock_out = entry.clock_out_time
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_default();
            
            let hours = entry.total_hours
                .map(|h| format!("{:.2}", h))
                .unwrap_or_default();
            
            writer.write_record(&[
                entry.entry_date.to_string(),
                entry.clock_in_time.format("%H:%M").to_string(),
                clock_out,
                entry.break_duration_minutes.to_string(),
                hours,
                format!("{:.2}", entry.overtime_hours),
                format!("{:?}", entry.status),
            ]).map_err(|e| WmsError::Export(format!("CSV write error: {}", e)))?;
        }
        
        // Write summary
        writer.write_record(&["", "", "", "", "", "", ""]).ok();
        writer.write_record(&["Summary", "", "", "", "", "", ""]).ok();
        writer.write_record(&[
            "Regular Hours",
            &format!("{:.2}", timesheet.regular_hours),
            "", "", "", "", ""
        ]).ok();
        writer.write_record(&[
            "Overtime Hours",
            &format!("{:.2}", timesheet.overtime_hours),
            "", "", "", "", ""
        ]).ok();
        writer.write_record(&[
            "Total Hours",
            &format!("{:.2}", timesheet.total_hours),
            "", "", "", "", ""
        ]).ok();
        writer.write_record(&[
            "Days Worked",
            &timesheet.days_worked.to_string(),
            "", "", "", "", ""
        ]).ok();
        
        let data = writer.into_inner()
            .map_err(|e| WmsError::Export(format!("CSV flush error: {}", e)))?;
        
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};
    use crate::models::{TimeEntry, TimeEntryStatus, ClockMethod};
    
    fn create_test_timesheet() -> Timesheet {
        Timesheet {
            user_id: "user1".to_string(),
            user_name: "John Doe".to_string(),
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            regular_hours: 40.0,
            overtime_hours: 5.0,
            double_time_hours: 0.0,
            sick_hours: 0.0,
            vacation_hours: 0.0,
            holiday_hours: 0.0,
            total_hours: 45.0,
            total_breaks_minutes: 150,
            days_worked: 5,
            late_arrivals: 1,
            early_departures: 0,
            status: crate::models::TimesheetStatus::Draft,
            submitted_at: None,
            approved_by: None,
            approved_at: None,
            notes: None,
            entries: vec![
                TimeEntry {
                    id: "entry1".to_string(),
                    user_id: "user1".to_string(),
                    entry_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    clock_in_time: Utc::now(),
                    clock_out_time: Some(Utc::now()),
                    clock_in_location: None,
                    clock_out_location: None,
                    clock_in_method: ClockMethod::Biometric,
                    clock_out_method: Some(ClockMethod::Biometric),
                    clock_in_device: None,
                    clock_out_device: None,
                    break_duration_minutes: 30,
                    total_hours: Some(8.5),
                    overtime_hours: 0.5,
                    status: TimeEntryStatus::Completed,
                    notes: None,
                    edited_by: None,
                    edited_reason: None,
                    approved_by: None,
                    approved_at: None,
                    created_at: Utc::now(),
                    updated_at: None,
                    breaks: vec![],
                },
            ],
        }
    }
    
    #[test]
    fn test_excel_export() {
        let timesheet = create_test_timesheet();
        let result = ExcelExporter::export(&timesheet);
        assert!(result.is_ok());
        
        let data = result.unwrap();
        assert!(!data.is_empty());
        // XLSX files start with PK (zip signature)
        assert_eq!(&data[0..2], &[0x50, 0x4B]);
    }
    
    #[test]
    fn test_csv_export() {
        let timesheet = create_test_timesheet();
        let result = CsvExporter::export(&timesheet);
        assert!(result.is_ok());
        
        let data = result.unwrap();
        let content = String::from_utf8(data).unwrap();
        assert!(content.contains("Date,Clock In,Clock Out"));
        assert!(content.contains("Summary"));
    }
}

