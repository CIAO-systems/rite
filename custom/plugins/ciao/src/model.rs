use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use ciao_rs::ciao::common::{Date, Timestamp};
use model::{
    field::{add_field, Field},
    record::Record,
    value::Value,
    BoxedError,
};

/// A function that addes a timestamp to a [Record] by adding two fields,
/// `<field_name>.timeUtc` and `<field_name>.timeZone`
///
/// # Arguments
/// * `fields`: The fields to add a new field to
/// * `field_name`: The prefix of the two fields (timeUtc and timeZone)
/// * `date_str`: The date/time value as string
/// * `date_format`: The date format for parsing the string
///
pub fn add_timestamp_parse(
    fields: &mut Vec<Field>,
    field_name: &str,
    date_str: &str,
    date_format: &str,
) -> Result<(), BoxedError> {
    let naive = NaiveDateTime::parse_from_str(date_str, date_format)?;
    add_field(
        fields,
        &format!("{}.timeUtc", field_name),
        Value::I64(naive.and_utc().timestamp_millis()),
    );
    add_field(
        fields,
        &format!("{}.timeZone", field_name),
        Value::String("Europe/Berlin".to_string()),
    );
    Ok(())
}

/// Add a date/time value to the record by adding two fields,
/// `<field_name>.timeUtc` and `<field_name>.timeZone`
///
/// # Arguments
/// * `fields`: The fields to add a new field to
/// * `field_name`: The prefix of the two fields (timeUtc and timeZone)

pub fn add_timestamp(
    fields: &mut Vec<Field>,
    field_name: &str,
    timestamp: ciao_rs::ciao::common::Timestamp,
) {
    if let Some(timestamp) = timestamp.time_utc {
        add_field(
            fields,
            &format!("{}.timeUtc", field_name),
            Value::I64(seconds_and_nanos_to_millis(
                timestamp.seconds,
                timestamp.nanos,
            )),
        );
    }
    add_field(
        fields,
        &format!("{}.timeZone", field_name),
        Value::String(timestamp.time_zone),
    );
}

/// Converts seconds and nanos to millis
pub fn seconds_and_nanos_to_millis(seconds: i64, nanos: i32) -> i64 {
    let millis_from_seconds = seconds * 1000;
    let millis_from_nanos = nanos / 1_000_000;
    millis_from_seconds + millis_from_nanos as i64
}

/// Convert milliseconds to seconds and nanos
pub fn millis_to_seconds_and_nanos(millis: i64) -> (i64, i32) {
    let seconds = millis / 1000;
    let remaining_millis = millis % 1000;
    let nanoseconds = remaining_millis * 1_000_000;
    (seconds, nanoseconds as i32)
}

/// Get a timestamp field from the record. It reads two fields:
/// `<field_name>.timeUtc` and `<field_name>.timeZone`
///
pub fn get_timestamp(record: &Record, field_name: &str) -> Result<Timestamp, BoxedError> {
    let field_time_utc = &format!("{}.timeUtc", field_name);
    let field_time_zone = &format!("{}.timeZone", field_name);

    let time_utc = record.field_by_name(field_time_utc);
    let time_zone = record.field_by_name(field_time_zone);
    if let Some(time_utc) = time_utc {
        let (seconds, nanos) = millis_to_seconds_and_nanos(time_utc.value().to_string().parse()?);

        Ok(Timestamp {
            time_utc: Some(prost_types::Timestamp { seconds, nanos }),
            time_zone: time_zone.map(|f| f.value().to_string()).unwrap_or_default(),
        })
    } else {
        Err(format!("Field {field_time_utc} missing").into())
    }
}

/// Gets an optional string field from the record.
///
pub fn get_optional_string(record: &Record, field_name: &str) -> Option<String> {
    if let Some(field) = record.field_by_name(field_name) {
        return Some(field.value().to_string());
    }

    None
}

/// Gets a mandatory string field from the record.
///
pub fn get_mandatory_string(record: &Record, field_name: &str) -> Result<String, BoxedError> {
    Ok(record
        .field_by_name(field_name)
        .map(|field| field.value().to_string())
        .ok_or_else(|| BoxedError::from(format!("Mandatory field '{field_name}' not found")))?)
}

/// Gets a boolean value from the record. If the field is missing, the `default`
/// will be returned
///
pub fn get_bool(record: &Record, field_name: &str, default: bool) -> bool {
    if let Some(field) = record.field_by_name(field_name) {
        if let Value::Bool(value) = field.value() {
            return value;
        }
    }

    default
}

/// Gets a CIAO date value from a string field in record. String can be in format
/// YYY-MM-DD or ISO 8601
///
pub fn get_date(
    record: &Record,
    field_name: &str,
) -> Result<ciao_rs::ciao::common::Date, BoxedError> {
    let field = record
        .field_by_name(field_name)
        .ok_or_else(|| BoxedError::from(format!("Mandatory field '{field_name}' not found")))?;

    if let Value::String(field_value) = field.value() {
        match NaiveDate::parse_from_str(&field_value, "%Y-%m-%d") {
            Ok(date) => {
                return Ok(Date {
                    year: date.year(),
                    month: date.month0() as i32 + 1,
                    day: date.day0() as i32 + 1,
                })
            }
            Err(_) => {
                let date_time = field_value
                    .parse::<DateTime<Utc>>()
                    .map_err(|e| format!("Field '{field_name}' is invalid: {e}"))?;
                return Ok(Date {
                    year: date_time.year(),
                    month: date_time.month0() as i32 + 1,
                    day: date_time.day0() as i32 + 1,
                });
            }
        }
    }

    Err(format!("Mandatory field '{field_name}' is not a YYYY-MM-DD string").into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost_types::Timestamp as ProstTimestamp;

    #[test]
    fn test_add_timestamp_parse_success() {
        let mut fields = Vec::new();
        let date_str = "2024-03-15 10:30:00";
        let date_format = "%Y-%m-%d %H:%M:%S";
        let field_name = "event_time";

        let result = add_timestamp_parse(&mut fields, field_name, date_str, date_format);
        assert!(result.is_ok());
        assert_eq!(fields.len(), 2);

        let utc_field = fields
            .iter()
            .find(|f| f.name() == "event_time.timeUtc")
            .unwrap();
        let tz_field = fields
            .iter()
            .find(|f| f.name() == "event_time.timeZone")
            .unwrap();

        let naive_dt = NaiveDateTime::parse_from_str(date_str, date_format).unwrap();
        let expected_millis = naive_dt.and_utc().timestamp_millis();
        assert_eq!(utc_field.value(), Value::I64(expected_millis));
        assert_eq!(tz_field.value(), Value::String("Europe/Berlin".to_string()));
    }

    #[test]
    fn test_add_timestamp_parse_failure() {
        let mut fields = Vec::new();
        let date_str = "invalid date";
        let date_format = "%Y-%m-%d %H:%M:%S";
        let field_name = "event_time";

        let result = add_timestamp_parse(&mut fields, field_name, date_str, date_format);
        assert!(result.is_err());
        assert_eq!(fields.len(), 0); // No fields should be added on error.
    }

    #[test]
    fn test_add_timestamp() {
        let mut fields = Vec::new();
        let field_name = "event_time";
        let timestamp = Timestamp {
            time_utc: Some(ProstTimestamp {
                seconds: 1678886400, // Example timestamp
                nanos: 500000000,
            }),
            time_zone: "America/New_York".to_string(),
        };

        add_timestamp(&mut fields, field_name, timestamp);

        assert_eq!(fields.len(), 2);
        let utc_field = fields
            .iter()
            .find(|f| f.name() == "event_time.timeUtc")
            .unwrap();
        let tz_field = fields
            .iter()
            .find(|f| f.name() == "event_time.timeZone")
            .unwrap();

        assert_eq!(utc_field.value(), Value::I64(1678886400500)); // Calculated millis
        assert_eq!(
            tz_field.value(),
            Value::String("America/New_York".to_string())
        );

        // Test with time_utc = None
        let mut fields2 = Vec::new();
        let timestamp2 = Timestamp {
            time_utc: None,
            time_zone: "America/New_York".to_string(),
        };
        add_timestamp(&mut fields2, field_name, timestamp2);
        assert_eq!(fields2.len(), 1);
        let tz_field2 = fields2
            .iter()
            .find(|f| f.name() == "event_time.timeZone")
            .unwrap();
        assert_eq!(
            tz_field2.value(),
            Value::String("America/New_York".to_string())
        );
    }

    #[test]
    fn test_seconds_and_nanos_to_millis() {
        assert_eq!(seconds_and_nanos_to_millis(1, 500000000), 1500);
        assert_eq!(seconds_and_nanos_to_millis(0, 0), 0);
        assert_eq!(seconds_and_nanos_to_millis(10, 999999999), 10999);
    }

    #[test]
    fn test_millis_to_seconds_and_nanos() {
        assert_eq!(millis_to_seconds_and_nanos(1500), (1, 500000000));
        assert_eq!(millis_to_seconds_and_nanos(0), (0, 0));
        assert_eq!(millis_to_seconds_and_nanos(10999), (10, 999000000));
    }

    #[test]
    fn test_get_timestamp_success() {
        let mut record = Record::new();
        let field_name = "event_time";
        let millis = 1678886900500;
        let (seconds, nanos) = millis_to_seconds_and_nanos(millis);

        add_field(
            record.fields_as_mut(),
            &format!("{}.timeUtc", field_name),
            Value::I64(millis),
        );
        add_field(
            record.fields_as_mut(),
            &format!("{}.timeZone", field_name),
            Value::String("America/New_York".to_string()),
        );

        let timestamp = get_timestamp(&record, field_name).unwrap();

        assert_eq!(timestamp.time_utc, Some(ProstTimestamp { seconds, nanos }));
        assert_eq!(timestamp.time_zone, "America/New_York".to_string());
    }

    #[test]
    fn test_get_timestamp_missing_field() {
        let record = Record::new(); // Empty record

        let result = get_timestamp(&record, "event_time");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Field event_time.timeUtc missing"
        );
    }

    #[test]
    fn test_get_timestamp_invalid_millis() {
        let mut record = Record::new();
        let field_name = "event_time";

        add_field(
            record.fields_as_mut(),
            &format!("{}.timeUtc", field_name),
            Value::String("invalid".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            &format!("{}.timeZone", field_name),
            Value::String("America/New_York".to_string()),
        );

        let result = get_timestamp(&record, field_name);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_mandatory_string() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "field",
            Value::String("value".to_string()),
        );
        let result = get_mandatory_string(&record, "field");
        assert!(result.is_ok());
        assert_eq!("value", result.unwrap());
    }

    #[test]
    fn test_get_mandatory_string_missing() {
        let record = Record::new();
        let result = get_mandatory_string(&record, "field");
        assert!(result.is_err());
        let e = format!("{}", result.unwrap_err());
        assert_eq!("Mandatory field 'field' not found", e);
    }

    #[test]
    fn test_get_bool() {
        let mut record = Record::new();
        add_field(record.fields_as_mut(), "bool", Value::Bool(true));
        let result = get_bool(&record, "bool", false);
        assert!(result);
    }

    #[test]
    fn test_get_bool_default() {
        let record = Record::new();
        let result = get_bool(&record, "bool", false);
        assert!(!result);
        let result = get_bool(&record, "bool", true);
        assert!(result);
    }

    #[test]
    fn test_get_date() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "date",
            Value::String("2025-01-01".to_string()),
        );

        let result = get_date(&record, "date");
        assert!(result.is_ok());
        let value = result.unwrap();

        assert_eq!(
            value,
            Date {
                year: 2025,
                month: 1,
                day: 1
            }
        );
    }

    #[test]
    fn test_get_date_missing() {
        let record = Record::new();
        let result = get_date(&record, "date");
        assert!(result.is_err());
        let e = format!("{}", result.unwrap_err());
        assert_eq!("Mandatory field 'date' not found", e);
    }

    #[test]
    fn test_get_date_invalid() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "date",
            Value::String("This is not a date".to_string()),
        );
        let result = get_date(&record, "date");
        assert!(result.is_err());
        let e = format!("{}", result.unwrap_err());
        assert_eq!(
            "Field 'date' is invalid: input contains invalid characters",
            e
        );
    }

    #[test]
    fn test_get_date_with_time() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "date",
            Value::String("2025-01-01T23:00:00Z".to_string()),
        );
        let result = get_date(&record, "date");
        assert!(result.is_ok(), "Result should be ok, but is err");
        assert_eq!(
            result.unwrap(),
            Date {
                year: 2025,
                month: 1,
                day: 1
            }
        );
    }

    #[test]
    fn test_get_date_no_string() {
        let mut record = Record::new();
        add_field(record.fields_as_mut(), "date", Value::I32(4273));

        let result = get_date(&record, "date");
        assert!(result.is_err());
        let e = format!("{}", result.unwrap_err());
        assert_eq!("Mandatory field 'date' is not a YYYY-MM-DD string", e);
    }
}
