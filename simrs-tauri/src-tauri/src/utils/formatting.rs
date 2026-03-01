use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

/// Format date for display (Indonesian format)
pub fn format_date(date: &str) -> String {
    if date.is_empty() || date == "0000-00-00" {
        return "-".to_string();
    }

    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|d| d.format("%d-%m-%Y").to_string())
        .unwrap_or_else(|_| date.to_string())
}

/// Format datetime for display
pub fn format_datetime(datetime: &str) -> String {
    if datetime.is_empty() {
        return "-".to_string();
    }

    // Try parsing as datetime
    if let Ok(dt) = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S") {
        return dt.format("%d-%m-%Y %H:%M").to_string();
    }

    datetime.to_string()
}

/// Format time for display
pub fn format_time(time: &str) -> String {
    if time.is_empty() {
        return "-".to_string();
    }

    NaiveTime::parse_from_str(time, "%H:%M:%S")
        .map(|t| t.format("%H:%M").to_string())
        .unwrap_or_else(|_| time.to_string())
}

/// Format number as Indonesian currency
pub fn format_currency(amount: f64) -> String {
    let formatted = format!("{:.0}", amount);
    let mut result = String::new();
    let chars: Vec<char> = formatted.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push('.');
        }
        result.push(*c);
    }

    format!("Rp {}", result)
}

/// Format patient's age from birth date
pub fn format_age(tgl_lahir: &str) -> String {
    if tgl_lahir.is_empty() || tgl_lahir == "0000-00-00" {
        return "-".to_string();
    }

    let birth = match NaiveDate::parse_from_str(tgl_lahir, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return tgl_lahir.to_string(),
    };

    let today = chrono::Local::now().date_naive();
    let diff = today.signed_duration_since(birth);

    let years = diff.num_days() / 365;
    if years > 0 {
        return format!("{} Th", years);
    }

    let months = diff.num_days() / 30;
    if months > 0 {
        return format!("{} Bl", months);
    }

    let days = diff.num_days();
    format!("{} Hr", days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        assert_eq!(format_date("2024-01-15"), "15-01-2024");
        assert_eq!(format_date("0000-00-00"), "-");
        assert_eq!(format_date(""), "-");
    }

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(1500000.0), "Rp 1.500.000");
        assert_eq!(format_currency(50000.0), "Rp 50.000");
    }

    #[test]
    fn test_format_age() {
        // This test depends on current date
        let age = format_age("2000-01-01");
        assert!(age.contains("Th"));
    }
}