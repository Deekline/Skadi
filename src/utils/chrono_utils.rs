use chrono::{DateTime, NaiveDate, NaiveDateTime};

pub fn hhmm(s: &str) -> String {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return dt.format("%H:%M").to_string();
    }

    for fmt in [
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
    ] {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return ndt.format("%H:%M").to_string();
        }
    }

    if s.len() == 5 && s.as_bytes().get(2) == Some(&b':') {
        return s.to_string();
    }

    s.to_string()
}

pub fn weekday_from_date(date: &str) -> Option<String> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    Some(d.format("%a").to_string())
}

#[cfg(test)]
mod tests {
    use super::{hhmm, weekday_from_date};

    #[test]
    fn hhmm_parses_rfc3339_zulu() {
        assert_eq!(hhmm("2026-01-09T12:34:56Z"), "12:34");
    }

    #[test]
    fn hhmm_parses_rfc3339_with_offset() {
        assert_eq!(hhmm("2026-01-09T23:05:00+01:00"), "23:05");
        assert_eq!(hhmm("2026-01-09T00:01:02-05:00"), "00:01");
    }

    #[test]
    fn hhmm_parses_naive_t_separator_no_fraction() {
        assert_eq!(hhmm("2026-01-09T07:08:09"), "07:08");
    }

    #[test]
    fn hhmm_parses_naive_space_separator_with_fraction() {
        assert_eq!(hhmm("2026-01-09 17:45:59.9"), "17:45");
        assert_eq!(hhmm("2026-01-09 17:45:59.987654"), "17:45");
    }

    #[test]
    fn hhmm_accepts_hh_colon_mm_exact() {
        assert_eq!(hhmm("09:07"), "09:07");
        assert_eq!(hhmm("0a:07"), "0a:07");
    }

    #[test]
    fn hhmm_falls_back_to_original_for_unrecognized_strings() {
        assert_eq!(hhmm("not a date"), "not a date");
        assert_eq!(hhmm("2026-01-09T12:34"), "2026-01-09T12:34");
        assert_eq!(hhmm("2026/01/09 12:34:56"), "2026/01/09 12:34:56");
    }

    #[test]
    fn weekday_from_date_valid_dates() {
        assert_eq!(weekday_from_date("2026-01-09").as_deref(), Some("Fri"));
        assert_eq!(weekday_from_date("2026-1-9").as_deref(), Some("Fri"));
        assert_eq!(weekday_from_date("1970-01-01").as_deref(), Some("Thu"));
        assert_eq!(weekday_from_date("2024-02-29").as_deref(), Some("Thu"));
    }

    #[test]
    fn weekday_from_date_rejects_invalid_format() {
        assert_eq!(weekday_from_date("2026/01/09"), None);
        assert_eq!(weekday_from_date("09-01-2026"), None);
        assert_eq!(weekday_from_date(""), None);
    }

    #[test]
    fn weekday_from_date_rejects_invalid_dates() {
        assert_eq!(weekday_from_date("2026-13-01"), None);
        assert_eq!(weekday_from_date("2023-02-29"), None);
    }
}
