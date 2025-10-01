use ciao_rs::ciao::common::Timestamp;
use model::{field::add_field, record::Record, xml::config::Configuration};

use crate::model::add_timestamp_parse;

use super::*;

#[test]
fn test_get_field_found() {
    // Create a mock record with a field
    let field_name = "test_field";
    let field_value = Value::String("test_value".to_string());
    let field = Field::new_value(field_name, field_value.clone());
    let mut record = Record::new();
    record.fields_as_mut().push(field);

    // Call the function and assert the result
    let result = get_field(&record, field_name);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value(), field_value);
}

#[test]
fn test_get_field_not_found() {
    // Create a mock record without the desired field
    let record = Record::new();

    // Call the function and assert the error
    let result = get_field(&record, "missing_field");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Field 'missing_field' missing in record"
    );
}

#[test]
fn test_get_timestamp() -> Result<(), BoxedError> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_timestamp_parse(fields, "timestamp", "2025-02-12 08:00", "%Y-%m-%d %H:%M")?;
    let ts = get_timestamp(&record)?;
    println!("{:?}", ts);

    assert_eq!(ts.time_utc.unwrap().seconds, 1739347200);
    assert_eq!(ts.time_utc.unwrap().nanos, 0);
    assert_eq!(ts.time_zone, "Europe/Berlin");

    Ok(())
}

#[test]
fn test_get_identity_user() -> Result<(), BoxedError> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(
        fields,
        "identity.userId",
        Value::String("user-id".to_string()),
    );
    let identity = get_identity(&record)?;
    println!("{:?}", identity);
    assert_eq!(identity, Identity::UserId("user-id".to_string()));

    Ok(())
}

#[test]
fn test_get_identity_badge() -> Result<(), BoxedError> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(
        fields,
        "identity.badgeId",
        Value::String("badge-id".to_string()),
    );
    let identity = get_identity(&record)?;
    println!("{:?}", identity);
    assert_eq!(identity, Identity::BadgeId("badge-id".to_string()));

    Ok(())
}

#[test]
fn test_get_identity_none() {
    let record = Record::new();
    let identity = get_identity(&record);
    assert!(identity.is_err_and(|e| {
        e.to_string() == "Neither 'identity.userId' nor 'identity.badgeId' found in record"
    }));
}

#[test]
#[ignore = "for manual testing"]
fn test_clock_exporter() -> Result<(), BoxedError> {
    let mut exporter = ClockEntries::new();
    let mut config = Configuration::new();
    config.insert_str("url", "http://localhost:50051");
    config.insert_str("api-key", "top-secret-api-key");

    exporter.init(Some(config))?;

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_timestamp_parse(fields, "timestamp", "2025-02-12 08:00", "%Y-%m-%d %H:%M")?;
    add_field(
        fields,
        "identity.userId",
        Value::String("user-id".to_string()),
    );

    exporter.write(&record)?;

    Ok(())
}

#[test]
fn test_init() -> Result<(), BoxedError> {
    let mut exporter = ClockEntries::new();
    let config = Configuration::new();
    let result = exporter.init(Some(config));
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
    Ok(())
}

#[test]
fn test_write() -> Result<(), BoxedError> {
    let mut exporter = ClockEntries::new();
    let record = Record::new();
    let result = exporter.write(&record);
    println!("{:?}", result);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_clock_record_from_missing_time_utc() -> Result<(), BoxedError> {
    let record = Record::new();
    let result = clock_record_from(&record);
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "Field 'timestamp.timeUtc' missing in record"));
    Ok(())
}

#[test]
fn test_clock_record_from_missing_time_zone() -> Result<(), BoxedError> {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "timestamp.timeUtc", Value::I64(0));
    let result = clock_record_from(&record);
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "Field 'timestamp.timeZone' missing in record"));
    Ok(())
}

#[test]
fn test_clock_record_from_missing_user_or_badge() -> Result<(), BoxedError> {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "timestamp.timeUtc", Value::I64(0));
    add_field(
        record.fields_as_mut(),
        "timestamp.timeZone",
        Value::String("Utc".to_string()),
    );
    let result = clock_record_from(&record);
    println!("{:?}", result);
    assert!(result.is_err_and(
        |e| e.to_string() == "Neither 'identity.userId' nor 'identity.badgeId' found in record"
    ));
    Ok(())
}

#[test]
fn test_clock_record_from() -> Result<(), BoxedError> {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "timestamp.timeUtc", Value::I64(0));
    add_field(
        record.fields_as_mut(),
        "timestamp.timeZone",
        Value::String("Utc".to_string()),
    );
    add_field(
        record.fields_as_mut(),
        "identity.userId",
        Value::String("user-id".to_string()),
    );
    let result = clock_record_from(&record);
    println!("{:?}", result);
    assert!(result.is_ok());

    let clock = result.unwrap();
    assert_eq!(clock.id, None);
    assert_eq!(clock.identity, Some(Identity::UserId("user-id".into())));
    assert_eq!(
        clock.timestamp,
        Some(Timestamp {
            time_utc: Some(prost_types::Timestamp {
                seconds: 0,
                nanos: 0
            }),
            time_zone: "Utc".into()
        })
    );

    assert!(clock.cost_center_id.is_none());
    assert!(clock.device_id.is_none());
    assert!(clock.time_type_id.is_none());
    assert!(clock.project_id.is_none());
    assert!(clock.project_task_id.is_none());
    Ok(())
}
