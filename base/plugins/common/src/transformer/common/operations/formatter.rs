use chrono::DateTime;
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
                _ => None,
            }
        } else {
            None
        }
    }
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
        let field = Field::new_value(
            "name",
            Value::String("not a timestamp".to_string()),
        );
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
}
