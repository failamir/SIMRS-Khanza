use regex::Regex;

/// Validate NIK (Nomor Induk Kependudukan)
pub fn validate_nik(nik: &str) -> bool {
    if nik.len() != 16 {
        return false;
    }

    // Check if all characters are digits
    nik.chars().all(|c| c.is_ascii_digit())
}

/// Validate phone number (Indonesian format)
pub fn validate_phone(phone: &str) -> bool {
    let phone = phone.replace(&['-', ' ', '+'][..], "");

    if phone.len() < 10 || phone.len() > 13 {
        return false;
    }

    phone.chars().all(|c| c.is_ascii_digit())
}

/// Validate email format
pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .expect("Invalid regex");

    email_regex.is_match(email)
}

/// Validate date format (YYYY-MM-DD)
pub fn validate_date(date: &str) -> bool {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}

/// Validate no_rawat format (YYYY/MM/DD/XXXXXX)
pub fn validate_no_rawat(no_rawat: &str) -> bool {
    let parts: Vec<&str> = no_rawat.split('/').collect();

    if parts.len() != 4 {
        return false;
    }

    // Validate date parts
    if !validate_date(&format!("{}-{}-{}", parts[0], parts[1], parts[2])) {
        return false;
    }

    // Validate sequence number (6 digits)
    parts[3].len() == 6 && parts[3].chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_nik() {
        assert!(validate_nik("3201234567890001"));
        assert!(!validate_nik("320123456789")); // Too short
        assert!(!validate_nik("320123456789000A")); // Non-digit
    }

    #[test]
    fn test_validate_phone() {
        assert!(validate_phone("08123456789"));
        assert!(validate_phone("0812-3456-7890"));
        assert!(!validate_phone("123")); // Too short
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(!validate_email("invalid-email"));
    }

    #[test]
    fn test_validate_no_rawat() {
        assert!(validate_no_rawat("2024/01/15/000001"));
        assert!(!validate_no_rawat("2024-01-15-000001")); // Wrong format
    }
}