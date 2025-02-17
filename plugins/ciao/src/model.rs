use chrono::NaiveDateTime;
use ciao_rs::ciao::common::Timestamp;
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
}
