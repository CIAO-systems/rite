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
/// YYYY-MM-DD or ISO 8601
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
mod tests;
