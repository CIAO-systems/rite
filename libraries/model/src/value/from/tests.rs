use chrono::NaiveDate;
use serde_json::json;

use crate::{field::Field, record::Record, value::Value};

#[test]
fn test_value_from_json_bool() {
    let json_value = json!(true);
    let value = Value::from(json_value);
    assert_eq!(value, Value::Bool(true));
}

#[test]
fn test_value_from_json_i8() {
    let json_value = json!(123);
    let value = Value::from(json_value);
    assert_eq!(value, Value::I8(123));
}

#[test]
fn test_value_from_json_i16() {
    let json_value = json!(258);
    let value = Value::from(json_value);
    assert_eq!(value, Value::I16(258));
}

#[test]
fn test_value_from_json_array() {
    let json_value = json!([1, "abc", true]);
    let value = Value::from(json_value);
    assert_eq!(
        value,
        Value::Collection(vec![
            Value::I8(1),
            Value::String("abc".to_string()),
            Value::Bool(true)
        ])
    );
}

#[test]
fn test_value_from_json_object() {
    let json_value = json!({
        "name": "John Doe",
        "age": 30
    });
    let value = Value::from(json_value);

    let mut expected_record = Record::new();
    expected_record
        .fields_as_mut()
        .push(Field::new_value("age", Value::I8(30)));
    expected_record.fields_as_mut().push(Field::new_value(
        "name",
        Value::String("John Doe".to_string()),
    ));

    assert_eq!(value, Value::Record(expected_record));
}

#[test]
fn test_value_from_json_null() {
    let json_value = json!(null);
    let value = Value::from(json_value);
    assert_eq!(value, Value::None);
}

#[test]
fn test_value_from_json_date() {
    let json_value = json!("2023-10-27");
    let value = Value::from(json_value);
    assert_eq!(
        value,
        Value::Date(NaiveDate::from_ymd_opt(2023, 10, 27).unwrap())
    );
}

#[test]
fn test_value_from_json_char() {
    let json_value = json!('a');
    let value = Value::from(json_value);
    assert_eq!(value, Value::Char('a'));
}

#[test]
fn test_value_from_json_f64() {
    let json_value = json!(123.45);
    let value = Value::from(json_value);
    assert_eq!(value, Value::F64(123.45));
}
