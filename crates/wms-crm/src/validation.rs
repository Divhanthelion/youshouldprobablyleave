//! Data Validation Utilities
//! 
//! Provides validation for CRM data including phone numbers, emails, etc.

use phonenumber::Mode;
use wms_core::error::{WmsError, Result};

/// Validate and format a phone number
pub fn validate_phone_number(number: &str) -> Result<String> {
    // Try parsing with default country code (US)
    let parsed = phonenumber::parse(Some(phonenumber::country::Id::US), number)
        .map_err(|e| WmsError::validation(format!("Invalid phone number: {}", e)))?;
    
    // Validate the number is valid for its country
    if !phonenumber::is_valid(&parsed) {
        return Err(WmsError::validation("Phone number is not valid for its country"));
    }
    
    // Format in E.164 international format
    Ok(parsed.format().mode(Mode::E164).to_string())
}

/// Validate and format a phone number with country hint
pub fn validate_phone_with_country(number: &str, country_code: &str) -> Result<String> {
    let country = country_code.parse::<phonenumber::country::Id>()
        .map_err(|_| WmsError::validation(format!("Unknown country code: {}", country_code)))?;
    
    let parsed = phonenumber::parse(Some(country), number)
        .map_err(|e| WmsError::validation(format!("Invalid phone number: {}", e)))?;
    
    if !phonenumber::is_valid(&parsed) {
        return Err(WmsError::validation("Phone number is not valid for its country"));
    }
    
    Ok(parsed.format().mode(Mode::E164).to_string())
}

/// Extract phone number type (mobile, landline, etc.)
pub fn get_phone_type(number: &str) -> Result<PhoneType> {
    let parsed = phonenumber::parse(Some(phonenumber::country::Id::US), number)
        .map_err(|e| WmsError::validation(format!("Invalid phone number: {}", e)))?;

    // Use the is_valid_for_region to validate and infer type
    // Since phonenumber 0.3 doesn't expose number_type directly,
    // we return Unknown and rely on is_valid for validation
    if phonenumber::is_valid(&parsed) {
        Ok(PhoneType::Unknown)
    } else {
        Err(WmsError::validation("Invalid phone number"))
    }
}

/// Phone number types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneType {
    Mobile,
    Landline,
    TollFree,
    Voip,
    Unknown,
}

/// Validate email format (basic validation)
pub fn validate_email(email: &str) -> Result<()> {
    // Basic validation - contains @ and at least one .
    if !email.contains('@') {
        return Err(WmsError::validation("Email must contain @"));
    }
    
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(WmsError::validation("Email must have exactly one @"));
    }
    
    let local = parts[0];
    let domain = parts[1];
    
    if local.is_empty() || domain.is_empty() {
        return Err(WmsError::validation("Email local and domain parts cannot be empty"));
    }
    
    if !domain.contains('.') {
        return Err(WmsError::validation("Email domain must contain a dot"));
    }
    
    // Check for invalid characters
    let valid_chars = |c: char| c.is_alphanumeric() || ".-_+".contains(c);
    if !local.chars().all(valid_chars) {
        return Err(WmsError::validation("Email contains invalid characters"));
    }
    
    Ok(())
}

/// Validate tax ID format (basic US EIN/SSN validation)
pub fn validate_tax_id(tax_id: &str) -> Result<TaxIdType> {
    // Remove common separators
    let cleaned: String = tax_id.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    
    match cleaned.len() {
        9 => {
            // Could be EIN (XX-XXXXXXX) or SSN (XXX-XX-XXXX)
            // EIN first two digits are 01-99 (excluding some ranges)
            let first_two: u32 = cleaned[0..2].parse().unwrap_or(0);
            if first_two >= 10 && first_two <= 99 {
                Ok(TaxIdType::Ein)
            } else {
                Ok(TaxIdType::Ssn)
            }
        }
        _ => Err(WmsError::validation("Tax ID must be 9 digits")),
    }
}

/// Tax ID types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxIdType {
    Ein, // Employer Identification Number
    Ssn, // Social Security Number
}

/// Validate credit card number (Luhn algorithm)
pub fn validate_credit_card(number: &str) -> Result<()> {
    let cleaned: String = number.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    
    if cleaned.len() < 13 || cleaned.len() > 19 {
        return Err(WmsError::validation("Credit card number must be 13-19 digits"));
    }
    
    // Luhn algorithm
    let mut sum = 0;
    let mut double = false;
    
    for c in cleaned.chars().rev() {
        let mut digit = c.to_digit(10).unwrap();
        
        if double {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        
        sum += digit;
        double = !double;
    }
    
    if sum % 10 != 0 {
        return Err(WmsError::validation("Invalid credit card number"));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("user.name@domain.co.uk").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
    }
    
    #[test]
    fn test_tax_id_validation() {
        assert!(validate_tax_id("12-3456789").is_ok());
        assert!(validate_tax_id("123456789").is_ok());
        assert!(validate_tax_id("12345").is_err());
    }
    
    #[test]
    fn test_luhn_algorithm() {
        // Valid test card numbers
        assert!(validate_credit_card("4532015112830366").is_ok());
        assert!(validate_credit_card("4111111111111111").is_ok());
        // Invalid
        assert!(validate_credit_card("1234567890123456").is_err());
    }
}

