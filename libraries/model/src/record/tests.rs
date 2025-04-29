use serde_json::json;

use crate::value::Value;

use super::*;

#[test]
fn test_record_new() {
    let mut record = Record::new();
    assert!(record.fields_as_mut().is_empty());
}

#[test]
fn test_record_field_by_name() {
    let mut record = Record::new();
    record
        .fields
        .push(Field::new_value("name", Value::String("Alice".to_string())));
    record.fields.push(Field::new_value("age", Value::I32(30)));

    assert!(record.field_by_name("unknown").is_none());
    assert!(record.field_by_name("name").is_some());
    assert!(record.field_by_name("age").is_some());

    let value = record.field_by_name("name").unwrap().value();
    assert!(matches!(value, Value::String(_)));
    match value {
        Value::String(s) => assert_eq!(s, "Alice"),
        _ => panic!("Wrong type"),
    }
}

#[test]
fn test_record_fields() {
    let mut record = Record::new();
    record
        .fields
        .push(Field::new_value("name", Value::String("Alice".to_string())));
    record.fields.push(Field::new_value("age", Value::I32(30)));

    let fields = record.fields_as_mut();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "name");
    assert_eq!(fields[1].name(), "age");
}

#[test]
fn test_record_fields_immutability() {
    let mut record = Record::new();
    record
        .fields
        .push(Field::new_value("active", Value::Bool(true)));

    let fields = record.fields_as_mut();
    assert_eq!(fields.len(), 1);

    // This next line would cause a compilation error if uncommented,
    // demonstrating that fields() returns an immutable reference
    // fields.push(Field::new_string("name".to_string(), "Bob".to_string()));
}

#[test]
fn test_record_multiple_fields() {
    let mut record = Record::new();
    record.fields.push(Field::new_value(
        "name",
        Value::String("Charlie".to_string()),
    ));
    record.fields.push(Field::new_value("age", Value::I32(25)));
    record
        .fields
        .push(Field::new_value("student", Value::Bool(false)));

    let fields = record.fields_as_mut();
    assert_eq!(fields.len(), 3);
    assert!(matches!(fields[0].value(), Value::String(_)));
    assert!(matches!(fields[1].value(), Value::I32(_)));
    assert!(matches!(fields[2].value(), Value::Bool(_)));
}

#[test]
fn test_record_empty_after_new() {
    let mut record = Record::new();
    assert!(record.fields_as_mut().is_empty());
    assert_eq!(record.fields_as_mut().len(), 0);
}

#[test]
fn test_record_copy() {
    let mut original = Record::new();
    original
        .fields_as_mut()
        .push(Field::new_value("value", Value::I32(42)));
    let copyied = Record::copy(&original);
    assert_eq!(original.fields().len(), copyied.fields().len());
    for field in original.fields() {
        assert!(copyied.field_by_name(field.name()).is_some());
    }
    assert_eq!(
        original.field_by_name("value").unwrap().value(),
        copyied.field_by_name("value").unwrap().value()
    );
}

#[test]
fn test_record_from_json_empty() {
    let json_value = json!({});
    let record = Record::from(json_value);
    assert_eq!(record, Record::new());
}

#[test]
fn test_record_from_json_single_field() {
    let json_value = json!({
        "name": "John Doe"
    });
    let record = Record::from(json_value);

    let mut expected_record = Record::new();
    expected_record.fields_as_mut().push(Field::new_value(
        "name",
        crate::value::Value::String("John Doe".to_string()),
    ));

    assert_eq!(record, expected_record);
}

#[test]
fn test_record_from_json_multiple_fields() {
    let json_value = json!({
        "name": "John Doe",
        "age": 30,
        "is_active": true,
        "items": [1, 2, 3]
    });
    let record = Record::from(json_value);

    let mut expected_record = Record::new();
    expected_record
        .fields_as_mut()
        .push(Field::new_value("age", crate::value::Value::I8(30)));
    expected_record.fields_as_mut().push(Field::new_value(
        "is_active",
        crate::value::Value::Bool(true),
    ));
    expected_record.fields_as_mut().push(Field::new_value(
        "items",
        crate::value::Value::Collection(vec![
            crate::value::Value::I8(1),
            crate::value::Value::I8(2),
            crate::value::Value::I8(3),
        ]),
    ));
    expected_record.fields_as_mut().push(Field::new_value(
        "name",
        crate::value::Value::String("John Doe".to_string()),
    ));

    assert_eq!(record, expected_record);
}

#[test]
fn test_record_from_json_nested_object() {
    let json_value = json!({
        "person": {
            "name": "Jane Smith",
            "age": 25
        }
    });
    let record = Record::from(json_value);

    let mut expected_record = Record::new();
    let mut nested_record = Record::new();
    nested_record
        .fields_as_mut()
        .push(Field::new_value("age", crate::value::Value::I8(25)));
    nested_record.fields_as_mut().push(Field::new_value(
        "name",
        crate::value::Value::String("Jane Smith".to_string()),
    ));

    expected_record.fields_as_mut().push(Field::new_value(
        "person",
        crate::value::Value::Record(nested_record),
    ));

    assert_eq!(record, expected_record);
}
