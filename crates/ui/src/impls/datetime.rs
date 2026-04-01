use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};

pub fn format_utc_ymd_hm(value: &str) -> String {
    match parse_datetime_utc(value) {
        Some(dt) => format!("{} (UTC)", dt.format("%Y-%m-%d %H:%M")),
        None => value.trim().to_string(),
    }
}

pub fn format_utc_ymd_hms(value: &str) -> String {
    match parse_datetime_utc(value) {
        Some(dt) => format!("{} (UTC)", dt.format("%Y-%m-%d %H:%M:%S")),
        None => value.trim().to_string(),
    }
}

fn parse_datetime_utc(value: &str) -> Option<DateTime<Utc>> {
    let raw = value.trim();
    if raw.is_empty() {
        return None;
    }

    if let Ok(dt) = DateTime::parse_from_rfc3339(raw) {
        return Some(dt.with_timezone(&Utc));
    }

    if let Some(offset_fixed) = normalize_short_utc_offset(raw) {
        if let Ok(dt) = DateTime::parse_from_str(&offset_fixed, "%Y-%m-%d %H:%M:%S%.f%:z") {
            return Some(dt.with_timezone(&Utc));
        }
    }

    if let Ok(dt) = DateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S%.f%:z") {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = DateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S%.f%:z") {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(Utc.from_utc_datetime(&naive));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S%.f") {
        return Some(Utc.from_utc_datetime(&naive));
    }
    if let Ok(date) = NaiveDate::parse_from_str(raw, "%Y-%m-%d") {
        let naive = date.and_hms_opt(0, 0, 0)?;
        return Some(Utc.from_utc_datetime(&naive));
    }

    None
}

fn normalize_short_utc_offset(value: &str) -> Option<String> {
    if value.len() < 3 {
        return None;
    }
    let suffix = &value[value.len() - 3..];
    let bytes = suffix.as_bytes();
    let is_short_offset = (bytes[0] == b'+' || bytes[0] == b'-')
        && bytes[1].is_ascii_digit()
        && bytes[2].is_ascii_digit();
    if !is_short_offset {
        return None;
    }
    Some(format!("{value}:00"))
}
