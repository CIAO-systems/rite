use std::error::Error;

use model::field::add_field;

use crate::transformer::mapper::config::{
    Name, Type,
    pattern::{Pattern, Patterns},
};

use super::*;

// Mock implementations for testing
fn create_field(target_type: &str) -> config::Field {
    config::Field {
        name: config::Name {
            source: "source".to_string(),
            target: "target".to_string(),
        },
        field_type: config::Type {
            source: "source_type".to_string(),
            target: target_type.to_string(),
        },
        patterns: None,
        values: Some(config::values::Values { value: vec![] }),
    }
}

fn create_value(target: String) -> config::values::Value {
    config::values::Value {
        source: "source_value".to_string(),
        target,
    }
}

#[test]
fn test_parse_successful_conversions() {
    // Test successful integer conversions
    assert!(matches!(parse::<i32, _>("42", Value::I32), Value::I32(42)));
    assert!(matches!(
        parse::<i64, _>("9223372036854775807", Value::I64),
        Value::I64(9223372036854775807)
    ));

    // Test successful float conversions
    assert!(
        matches!(parse::<f32, _>("3.14", Value::F32), Value::F32(x) if (x - 3.14).abs() < f32::EPSILON)
    );
    assert!(
        matches!(parse::<f64, _>("3.14159", Value::F64), Value::F64(x) if (x - 3.14159).abs() < f64::EPSILON)
    );
}

#[test]
fn test_parse_failed_conversions() {
    // Test parsing failures
    assert!(matches!(
        parse::<i32, _>("not a number", Value::I32),
        Value::None
    ));
    assert!(matches!(parse::<i64, _>("xyz", Value::I64), Value::None));
    assert!(matches!(parse::<f32, _>("hello", Value::F32), Value::None));
    assert!(matches!(parse::<f64, _>("world", Value::F64), Value::None));
}

#[test]
fn test_convert_value_successful_conversions() {
    // Test string conversion
    let string_field = create_field("string");
    let string_value = create_value("hello".to_string());
    assert!(matches!(
        convert_value(&string_field, string_value),
        Value::String(s) if s == "hello"
    ));

    // Test integer conversions
    let i32_field = create_field("i32");
    let i32_value = create_value("42".to_string());
    assert!(matches!(
        convert_value(&i32_field, i32_value),
        Value::I32(42)
    ));

    let i64_field = create_field("i64");
    let i64_value = create_value("9223372036854775807".to_string());
    assert!(matches!(
        convert_value(&i64_field, i64_value),
        Value::I64(9223372036854775807)
    ));

    // Test float conversions
    let f32_field = create_field("f32");
    let f32_value = create_value("3.14".to_string());
    assert!(matches!(
        convert_value(&f32_field, f32_value),
        Value::F32(x) if (x - 3.14).abs() < f32::EPSILON
    ));

    let f64_field = create_field("f64");
    let f64_value = create_value("3.14159".to_string());
    assert!(matches!(
        convert_value(&f64_field, f64_value),
        Value::F64(x) if (x - 3.14159).abs() < f64::EPSILON
    ));
}

#[test]
fn test_convert_value_failed_conversions() {
    // Test failed conversions
    let i32_field = create_field("i32");
    let bad_i32_value = create_value("not a number".to_string());
    assert!(matches!(
        convert_value(&i32_field, bad_i32_value),
        Value::None
    ));

    let f64_field = create_field("f64");
    let bad_f64_value = create_value("invalid".to_string());
    assert!(matches!(
        convert_value(&f64_field, bad_f64_value),
        Value::None
    ));

    // Test unknown type
    let unknown_field = create_field("unknown_type");
    let unknown_value = create_value("some value".to_string());
    assert!(matches!(
        convert_value(&unknown_field, unknown_value),
        Value::None
    ));
}

#[test]
fn test_initializable() -> Result<(), Box<dyn Error>> {
    let config = Configuration::with_xml("../../data/star-wars-mapper-config.xml");
    let mut subject = MapperTransformer::new();

    subject.init(Some(config))?;

    let mut record = Record::new();
    add_field(record.fields_as_mut(), "episode_id", "1".into());
    let transformed = subject.process(&record)?;

    assert!(transformed.field_by_name("episode").is_some());
    assert_eq!(
        transformed.field_by_name("episode").unwrap().value(),
        Value::String("One".to_string())
    );
    Ok(())
}

#[test]
fn test_initializable_no_mapper() -> Result<(), Box<dyn Error>> {
    let config = Configuration::new();
    let mut subject = MapperTransformer::new();

    subject.init(Some(config))?;

    let mut record = Record::new();
    add_field(record.fields_as_mut(), "episode_id", "1".into());
    let transformed = subject.process(&record)?;

    assert!(transformed.field_by_name("episode").is_none());
    assert!(transformed.field_by_name("episode_id").is_some());
    assert_eq!(
        transformed.field_by_name("episode_id").unwrap().value(),
        Value::String("1".to_string())
    );
    Ok(())
}

#[test]
fn test_map_field() -> Result<(), Box<dyn Error>> {
    let mut mapping_field = config::Field {
        name: Name {
            source: "source".to_string(),
            target: "target".to_string(),
        },
        field_type: Type {
            source: "".to_string(),
            target: "".to_string(),
        },
        patterns: None,
        values: None,
    };
    let field = Field::new_value("source", "X".into());
    let record = Record::new();
    let mapped = map_field(&mapping_field, &field, Some(&record));

    assert!(mapped.is_none());

    mapping_field.patterns = Some(Patterns {
        pattern: vec![Pattern {
            matcher: "X".to_string(),
            replacement: "V".to_string(),
        }],
    });

    let mapped = map_field(&mapping_field, &field, Some(&record));
    assert!(mapped.is_some());
    assert_eq!(mapped.unwrap().value(), Value::String("V".to_string()));
    Ok(())
}
