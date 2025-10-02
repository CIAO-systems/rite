use model::{record::Record, value::Value};
use serde_json::json;

use crate::importers::youtrack::factory::fill_record_from_json;

use super::{json_object_to_value_map, json2model};

static TEST_STR: &str = "This is not the droids, your're looking for";

#[test]
fn test_json2model() {
    assert_eq!(Value::U64(42), json2model(&json!(42)));
    assert_eq!(Value::I64(-42), json2model(&json!(-42)));
    assert_eq!(Value::F64(42.73), json2model(&json!(42.73)));
    assert_eq!(
        Value::String(TEST_STR.to_string()),
        json2model(&json!(TEST_STR))
    );
    assert_eq!(Value::None, json2model(&json!([1, 2, 3])));
    assert_eq!(Value::None, json2model(&json!({"field": 42.73})));
    assert_eq!(Value::Bool(false), json2model(&json!(false)));
    assert_eq!(Value::Bool(true), json2model(&json!(true)));
}

#[test]
fn test_json_object_to_value_map() {
    let map = json_object_to_value_map("prefix", &json!({"field": 42.73}));

    assert_eq!(1, map.len());
    let entry = map.iter().next().unwrap();
    assert_eq!("prefix.field", entry.0);
    assert_eq!(Value::F64(42.73), *entry.1);
}

#[test]
fn test_fill_record_from_json() {
    let mut record = Record::new();
    let json = json!({"record": {"field": "value"}});
    let result = fill_record_from_json(&mut record, &json);
    assert!(result);
    println!("{:?}", record);
    assert!(record.field_by_name("record.field").is_some());
    assert_eq!(
        record.field_by_name("record.field").unwrap().value(),
        Value::String("value".into())
    )
}
