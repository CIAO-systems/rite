use super::*;

#[test]
fn test_new_bool() {
    let field = Field::new_value("active", Value::Bool(true));
    assert_eq!(field.name, "active");
    assert!(matches!(field.value, Value::Bool(true)));
}

#[test]
fn test_new_i32() {
    let field = Field::new_value("age", Value::I32(42));
    assert_eq!(field.name, "age");
    assert!(matches!(field.value, Value::I32(42)));
}

#[test]
fn test_new_usize() {
    let field = Field::new_value("size", Value::USize(73));
    assert_eq!(field.name, "size");
    assert!(matches!(field.value, Value::USize(73)));
}

#[test]
fn test_new_f64() {
    let field = Field::new_value("price", Value::F64(73.37));
    assert_eq!(field.name, "price");
    assert!(matches!(field.value, Value::F64(f) if (f - 73.37).abs() < f64::EPSILON));
}

#[test]
fn test_new_string() {
    let field = Field::new_value("name", Value::String("Alice".to_string()));
    assert_eq!(field.name, "name");
    assert!(matches!(field.value, Value::String(ref s) if s == "Alice"));
}

#[test]
fn test_new_blob() {
    let blob = vec![0x00, 0x01, 0x02, 0x03];
    let field = Field::new_value("data", Value::Blob(blob.clone()));
    assert_eq!(field.name, "data");
    assert!(matches!(field.value, Value::Blob(ref b) if b == &blob));
}

#[test]
fn test_new_null() {
    let field = Field::new("optional");
    assert_eq!(field.name, "optional");
    assert!(matches!(field.value, Value::None));
}

#[test]
fn test_field_name_uniqueness() {
    let field1 = Field::new_value("id", Value::I32(1));
    let field2 = Field::new_value("id", Value::I32(2));
    assert_eq!(field1.name, field2.name);
    assert!(matches!(field1.value, Value::I32(1)));
    assert!(matches!(field2.value, Value::I32(2)));
}

#[test]
fn test_field_value_type_mismatch() {
    let int_field = Field::new_value("number", Value::I32(42));
    assert!(matches!(int_field.value, Value::I32(_)));
    assert!(!matches!(int_field.value, Value::String(_)));
}

#[test]
fn test_name_getter() {
    let field = Field::new("username");
    assert_eq!(field.name(), "username");
}

#[test]
fn test_value_getter_string() {
    let field = Field::new_value("email", Value::String("alice@example.com".to_string()));
    match field.value() {
        Value::String(s) => assert_eq!(s, "alice@example.com"),
        _ => panic!("Expected String value"),
    }
}

#[test]
fn test_value_getter_i32() {
    let field = Field::new_value("age", Value::I32(30));
    match field.value() {
        Value::I32(i) => assert_eq!(i, 30),
        _ => panic!("Expected I32 value"),
    }
}

#[test]
fn test_value_getter_bool() {
    let field = Field::new_value("is_active", Value::Bool(true));
    match field.value() {
        Value::Bool(b) => assert_eq!(b, true),
        _ => panic!("Expected Bool value"),
    }
}

#[test]
fn test_value_getter_bool_as_ref() {
    let field = Field::new_value("is_active", Value::Bool(true));
    match field.value_as_ref() {
        Value::Bool(b) => assert_eq!(*b, true),
        _ => panic!("Expected Bool value"),
    }
}

#[test]
fn test_value_getter_null() {
    let field = Field::new("optional");
    assert!(matches!(field.value(), Value::None));
}

#[test]
fn test_add_str() {
    let mut fields: Vec<Field> = Vec::new();
    add_field(&mut fields, "name", "value".into());

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "name");
    assert_eq!(fields[0].value, Value::String("value".to_string()));
}

#[test]
fn test_add_optional_str_some() {
    let mut fields: Vec<Field> = Vec::new();
    add_optional_field::<&str>(&mut fields, "name", Some("value".into()));

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "name");
    assert_eq!(fields[0].value, Value::String("value".to_string()));
}

#[test]
fn test_add_optional_str_none() {
    let mut fields: Vec<Field> = Vec::new();
    let value: Option<&str> = None;
    add_optional_field(&mut fields, "name", value);

    assert_eq!(fields.len(), 0);
}

#[test]
fn test_add_field_f64() {
    let mut fields: Vec<Field> = Vec::new();
    add_field(&mut fields, "f64_field", 3.14.into());

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "f64_field");
    assert_eq!(fields[0].value, Value::F64(3.14));
}

#[test]
fn test_add_field_i64() {
    let mut fields: Vec<Field> = Vec::new();
    add_field(&mut fields, "i64_field", (42 as i64).into());

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "i64_field");
    assert_eq!(fields[0].value, Value::I64(42));
}

#[test]
fn test_add_optional_field_f64() {
    let mut fields: Vec<Field> = Vec::new();
    add_optional_field(&mut fields, "f64_field", Some(3.14));

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "f64_field");
    assert_eq!(fields[0].value, Value::F64(3.14));
}

#[test]
fn test_add_optional_field_i64() {
    let mut fields: Vec<Field> = Vec::new();
    add_optional_field(&mut fields, "i64_field", Some(42 as i64));

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "i64_field");
    assert_eq!(fields[0].value, Value::I64(42));
}

#[test]
fn test_add_optional_field_none() {
    let mut fields: Vec<Field> = Vec::new();
    add_optional_field(&mut fields, "missing_field", None::<f64>);

    assert_eq!(fields.len(), 0);
}
