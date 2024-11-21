use crate::value::Value;

use super::*;

#[test]
fn test_record_new() {
    let record = Record::new();
    assert!(record.fields().is_empty());
}

#[test]
fn test_record_fields() {
    let mut record = Record::new();
    record.fields.push(Field::new_string("name".to_string(), "Alice".to_string()));
    record.fields.push(Field::new_i32("age".to_string(), 30));

    let fields = record.fields();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "name");
    assert_eq!(fields[1].name(), "age");
}

#[test]
fn test_record_fields_immutability() {
    let mut record = Record::new();
    record.fields.push(Field::new_bool("active".to_string(), true));

    let fields = record.fields();
    assert_eq!(fields.len(), 1);
    
    // This next line would cause a compilation error if uncommented,
    // demonstrating that fields() returns an immutable reference
    // fields.push(Field::new_string("name".to_string(), "Bob".to_string()));
}

#[test]
fn test_record_multiple_fields() {
    let mut record = Record::new();
    record.fields.push(Field::new_string("name".to_string(), "Charlie".to_string()));
    record.fields.push(Field::new_i32("age".to_string(), 25));
    record.fields.push(Field::new_bool("student".to_string(), false));

    let fields = record.fields();
    assert_eq!(fields.len(), 3);
    assert!(matches!(fields[0].value(), Value::String(_)));
    assert!(matches!(fields[1].value(), Value::I32(_)));
    assert!(matches!(fields[2].value(), Value::Bool(_)));
}

#[test]
fn test_record_empty_after_new() {
    let record = Record::new();
    assert!(record.fields().is_empty());
    assert_eq!(record.fields().len(), 0);
}
