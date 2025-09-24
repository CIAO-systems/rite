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

#[test]
fn test_get_optional_string_none() {
    let record = Record::new();
    let result = get_optional_string(&record, "field");
    assert!(result.is_none());
}   

#[test]
fn test_get_optional_string_some() {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "field", Value::I32(4273));
    let result = get_optional_string(&record, "field");
    assert!(result.is_some_and(|s| s == "4273"));
}   
