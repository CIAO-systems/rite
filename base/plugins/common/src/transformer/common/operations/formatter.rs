use chrono::{DateTime, TimeZone, Utc};
use model::{field::Field, value::Value};

pub struct Formatter {
    field: String,
    format: String,
}

impl Formatter {
    pub(crate) fn new(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 2 {
            Err(format!("Invalid parameter: {}", value).into())
        } else {
            Ok(Self {
                field: parts[0].to_string(),
                format: parts[1].to_string(),
            })
        }
    }

    pub fn apply(&self, field: &Field) -> Option<Value> {
        if field.name().eq(&self.field) {
            match self.format.as_str() {
                "isodatetime" => convert_isodatetime(field),
                "isodate" => convert_isodate(field),
                "unixtime" => convert_unixtime(field),
                _ => None,
            }
        } else {
            None
        }
    }
}

/// Converts an ISO 8601 date time string to an integer
fn convert_unixtime(field: &Field) -> Option<Value> {
    use chrono::NaiveDateTime;
    use chrono::TimeZone;
    use chrono::prelude::*;

    if let Value::String(datetime_str) = field.value() {
        // Attempt to parse with timezone first (e.g., "Z" or "+02:00")
        if let Ok(datetime) = DateTime::parse_from_rfc3339(&datetime_str) {
            return Some(Value::I64(datetime.timestamp_millis()));
        }

        // Then, attempt to parse as a NaiveDateTime with a timezone assumed
        // This is useful for strings like "2023-05-15T10:30:00" without a timezone.
        // We'll assume UTC for these cases.
        if let Ok(naive_datetime) =
            NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S")
        {
            let datetime_utc = Utc.from_utc_datetime(&naive_datetime);
            return Some(Value::I64(datetime_utc.timestamp_millis()));
        }

        // Finally, attempt to parse as a date-only string and assume midnight UTC
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(&datetime_str, "%Y-%m-%d") {
            return naive_date_to_millis(naive_date);
        }
    } else if let Value::Date(naive_date) = field.value() {
        return naive_date_to_millis(naive_date);
    }

    // If none of the parsing attempts succeed, return None.
    None
}

fn naive_date_to_millis(naive_date: chrono::NaiveDate) -> Option<Value> {
    if let Some(naive_datetime) = naive_date.and_hms_opt(0, 0, 0) {
        let datetime_utc = Utc.from_utc_datetime(&naive_datetime);
        return Some(Value::I64(datetime_utc.timestamp_millis()));
    }
    None
}

/// Converts an integer or a float to an ISO 8601 date time string
fn convert_isodate(field: &Field) -> Option<Value> {
    let value = field.value();
    match value {
        Value::F64(unix_time) => Some(Value::String(unix_to_iso_ymd(unix_time))),
        Value::I64(unix_time) => Some(Value::String(unix_to_iso_ymd(unix_time as f64))),
        Value::U64(unix_time) => Some(Value::String(unix_to_iso_ymd(unix_time as f64))),
        Value::I32(unix_time) => Some(Value::String(unix_to_iso_ymd(unix_time as f64))),
        Value::U32(unix_time) => Some(Value::String(unix_to_iso_ymd(unix_time as f64))),
        _ => None,
    }
}

/// Converts an integer or a float to an ISO 8601 date time string
fn convert_isodatetime(field: &Field) -> Option<Value> {
    let value = field.value();
    match value {
        Value::F64(unix_time) => Some(Value::String(unix_to_iso_datetime(unix_time))),
        Value::I64(unix_time) => Some(Value::String(unix_to_iso_datetime(unix_time as f64))),
        Value::U64(unix_time) => Some(Value::String(unix_to_iso_datetime(unix_time as f64))),
        Value::I32(unix_time) => Some(Value::String(unix_to_iso_datetime(unix_time as f64))),
        Value::U32(unix_time) => Some(Value::String(unix_to_iso_datetime(unix_time as f64))),
        _ => None,
    }
}

/// Formats a unix timestamp into yMD
fn unix_to_iso_ymd(timestamp: f64) -> String {
    let date_time = millis_to_date_time(timestamp);
    match date_time {
        Some(datetime) => datetime.format("%Y-%m-%d").to_string(),
        _ => format!("{}", timestamp),
    }
}

/// Formats a unix timestamp into ISO 8601
fn unix_to_iso_datetime(timestamp: f64) -> String {
    let date_time = millis_to_date_time(timestamp);
    match date_time {
        Some(datetime) => {
            // Format as an ISO 8601 string
            datetime.to_rfc3339()
        }
        None => format!("{}", timestamp),
    }
}

/// Converts a unix timestamp to a [DateTime]
fn millis_to_date_time(timestamp: f64) -> Option<DateTime<chrono::Utc>> {
    // Convert milliseconds to seconds and fractional seconds
    let seconds = (timestamp / 1000.0).floor() as i64;
    let nanos = ((timestamp % 1000.0) * 1e6).round() as u32;

    // Create a NaiveDateTime from the seconds and nanoseconds
    let date_time = DateTime::from_timestamp(seconds, nanos);
    date_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_timestamp() {
        let timestamp = 1672531200123.0; // Jan 1, 2023 00:00:00.123 UTC
        let result = unix_to_iso_datetime(timestamp);
        assert_eq!(result, "2023-01-01T00:00:00.123+00:00");
    }

    #[test]
    fn test_fractional_milliseconds() {
        let timestamp = 1672531200123.456; // Jan 1, 2023 00:00:00.123456 UTC
        let result = unix_to_iso_datetime(timestamp);
        assert_eq!(result, "2023-01-01T00:00:00.123456055+00:00");
    }

    #[test]
    fn test_zero_timestamp() {
        let timestamp = 0.0; // Epoch start
        let result = unix_to_iso_datetime(timestamp);
        assert_eq!(result, "1970-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_large_timestamp() {
        let timestamp = 32503680000000.0; // Jan 1, 3000 00:00:00.000 UTC
        let result = unix_to_iso_datetime(timestamp);
        assert_eq!(result, "3000-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_negative_timestamp() {
        let timestamp = -2208988800000.0; // Jan 1, 1900 00:00:00.000 UTC
        let result = unix_to_iso_datetime(timestamp);
        assert_eq!(result, "1900-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_invalid_timestamp() {
        let timestamp = f64::INFINITY; // Invalid timestamp
        let result = unix_to_iso_datetime(timestamp);
        assert_eq!(result, format!("{}", timestamp)); // Should return the raw timestamp
    }

    #[test]
    fn test_nan_timestamp() {
        let timestamp = f64::NAN; // NaN timestamp
        let result = unix_to_iso_datetime(timestamp);
        assert_eq!(result, "1970-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_convert_f64() {
        let field = Field::new_value("name", Value::F64(1672531200123.0));
        let result = convert_isodatetime(&field);
        assert_eq!(
            result,
            Some(Value::String("2023-01-01T00:00:00.123+00:00".to_string()))
        );
    }

    #[test]
    fn test_convert_i64() {
        let field = Field::new_value("name", Value::I64(1672531200123));
        let result = convert_isodatetime(&field);
        assert_eq!(
            result,
            Some(Value::String("2023-01-01T00:00:00.123+00:00".to_string()))
        );
    }

    #[test]
    fn test_convert_unsupported_type() {
        let field = Field::new_value("name", Value::String("not a timestamp".to_string()));
        let result = convert_isodatetime(&field);
        assert_eq!(result, None);
    }

    #[test]
    fn test_convert_negative_i64() {
        let field = Field::new_value("name", Value::I64(-2208988800000));
        let result = convert_isodatetime(&field);
        assert_eq!(
            result,
            Some(Value::String("1900-01-01T00:00:00+00:00".to_string()))
        );
    }

    #[test]
    fn test_convert_invalid_f64() {
        let field = Field::new_value("name", Value::F64(f64::NAN));
        let result = convert_isodatetime(&field);
        assert_eq!(
            result,
            Some(Value::String("1970-01-01T00:00:00+00:00".to_string()))
        );
    }

    #[test]
    fn test_convert_u64() {
        let field = Field::new_value("name", Value::U64(1672531200123));
        let result = convert_isodatetime(&field);
        assert_eq!(
            result,
            Some(Value::String("2023-01-01T00:00:00.123+00:00".to_string()))
        );
    }

    #[test]
    fn test_convert_i32() {
        let field = Field::new_value("name", Value::I32(-4273));
        let result = convert_isodatetime(&field);
        assert_eq!(
            result,
            Some(Value::String("1969-12-31T23:59:55+00:00".to_string()))
        );
    }

    #[test]
    fn test_convert_u32() {
        let field = Field::new_value("name", Value::U32(73));
        let result = convert_isodatetime(&field);
        assert_eq!(
            result,
            Some(Value::String("1970-01-01T00:00:00.073+00:00".to_string()))
        );
    }

    #[test]
    fn test_convert_unixtime_valid_iso8601() {
        // GIVEN a valid ISO 8601 date-time string in a Field
        let iso8601_string = "2023-05-15T10:30:00.000Z";
        let field = Field::new_value("field", iso8601_string.into());

        // WHEN the convert_unixtime function is called
        let result = convert_unixtime(&field);

        // THEN the function should return the correct UNIX timestamp in milliseconds
        // The expected value for "2023-05-15T10:30:00.000Z" is 1684146600000.
        let expected_value = Some(Value::I64(1684146600000));
        assert_eq!(
            result, expected_value,
            "The conversion from ISO 8601 to UNIX time should be correct."
        );
    }

    #[test]
    fn test_convert_unixtime_invalid_string() {
        // GIVEN an invalid date-time string in a Field
        let invalid_string = "not-a-date";
        let field = Field::new_value("timestamp", invalid_string.into());

        // WHEN the convert_unixtime function is called
        let result = convert_unixtime(&field);

        // THEN the function should return None
        let expected_value = None;
        assert_eq!(
            result, expected_value,
            "The function should return None for invalid input strings."
        );
    }

    #[test]
    fn test_convert_unixtime_empty_string() {
        // GIVEN an empty string in a Field
        let empty_string = "";
        let field = Field::new_value("timestamp", empty_string.into());

        // WHEN the convert_unixtime function is called
        let result = convert_unixtime(&field);

        // THEN the function should return None
        let expected_value = None;
        assert_eq!(
            result, expected_value,
            "The function should return None for an empty string."
        );
    }

    #[test]
    fn test_convert_unixtime_date_only() {
        // GIVEN a date-only string in a Field
        let date_string = "2023-05-15";
        let field = Field::new_value("birthdate", date_string.into());

        // WHEN the convert_unixtime function is called
        let result = convert_unixtime(&field);

        // THEN the function should parse it as midnight (00:00:00.000) UTC on that date.
        // The expected value for "2023-05-15T00:00:00.000Z" is 1684108800000.
        let expected_value = Some(Value::I64(1684108800000));
        assert_eq!(
            result, expected_value,
            "The conversion of a date-only string should be correct (midnight UTC)."
        );
    }
}
