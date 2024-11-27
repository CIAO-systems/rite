use model::{field::Field, record::Record, value::Value};
use transform::Transformer;

use super::StringFieldConverter;

#[test]
fn test_to_upper_case() {
    let transfomer = StringFieldConverter::new(super::StringFieldConversion::UpperCase);
    let mut record = Record::new();
    record
        .fields_as_mut()
        .push(Field::new_string("name".to_string(), "alice".to_string()));
    match transfomer.process(&record) {
        Ok(result) => {
            if let Some(field) = result.field_by_name("name") {
                let value = field.value();
                assert!(matches!(value, Value::String(_)));

                if let Value::String(value) = value {
                    assert_eq!("ALICE", value);
                };
            }
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_to_lower_case() {
    let transfomer = StringFieldConverter::new(super::StringFieldConversion::LowerCase);
    let mut record = Record::new();
    record
        .fields_as_mut()
        .push(Field::new_string("name".to_string(), "BOB".to_string()));
    match transfomer.process(&record) {
        Ok(result) => {
            if let Some(field) = result.field_by_name("name") {
                let value = field.value();
                assert!(matches!(value, Value::String(_)));

                if let Value::String(value) = value {
                    assert_eq!("bob", value);
                };
            }
        }
        Err(e) => panic!("{e}"),
    }
}
