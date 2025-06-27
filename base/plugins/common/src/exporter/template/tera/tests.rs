use model::{field::add_field, record::Record};
use tera::{Number, Value};

use crate::exporter::template::tera::record_to_tera_object;

use super::model_to_tera;

#[test]
fn test_model_to_tera_string() {
    const TEST_VALUE: &str = "This are not the droids, you are looking for";
    let mv = model::value::Value::String(TEST_VALUE.to_string());
    let tv = model_to_tera(mv);
    assert_eq!(tera::Value::String(TEST_VALUE.to_string()), tv);
}

#[test]
fn test_model_to_tera_i32() {
    const TEST_VALUE: i32 = 73;
    let mv = model::value::Value::I32(TEST_VALUE);
    let tv = model_to_tera(mv);
    assert_eq!(tera::Value::Number(TEST_VALUE.into()), tv);
}

#[test]
fn test_model_to_tera_f32() {
    const TEST_VALUE: f32 = 73.42;
    let mv = model::value::Value::F32(TEST_VALUE);
    let tv = model_to_tera(mv);
    let number = Number::from_f64(TEST_VALUE as f64);
    assert_eq!(tera::Value::Number(number.unwrap()), tv);
}

#[test]
fn test_model_to_tera_f64() {
    const TEST_VALUE: f64 = -73.42;
    let mv = model::value::Value::F64(TEST_VALUE);
    let tv = model_to_tera(mv);
    let number = Number::from_f64(TEST_VALUE);
    assert_eq!(tera::Value::Number(number.unwrap()), tv);
}

#[test]
fn test_model_to_tera_record() {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "name", model::value::Value::I32(42));
    let mv = model::value::Value::Record(record);
    let tv = model_to_tera(mv);

    let mut map = tera::Map::new();
    map.insert("name".to_string(), Value::Number(42.into()));

    assert_eq!(Value::Object(map), tv);
}

#[test]
fn test_model_to_tera_collection() {
    let mut collection = Vec::new();
    let mut expected = Vec::new();
    for i in 1..10 {
        collection.push(model::value::Value::I32(i));
        expected.push(tera::Value::Number(i.into()));
    }

    let mv = model::value::Value::Collection(collection);
    let tv = model_to_tera(mv);
    //println!("{:?}", tv);
    assert_eq!(tera::Value::Array(expected), tv);
}

#[test]
fn test_record_to_tera_object() {
    let mut mr = Record::new();
    add_field(
        mr.fields_as_mut(),
        "string_field",
        model::value::Value::String(String::from("string_value")),
    );
    add_field(
        mr.fields_as_mut(),
        "int_field",
        model::value::Value::I64(73),
    );
    add_field(
        mr.fields_as_mut(),
        "float_field",
        model::value::Value::F64(73.42),
    );

    let mut collection = Vec::new();
    let mut expected = Vec::new();
    for i in 1..10 {
        collection.push(model::value::Value::I32(i));
        expected.push(tera::Value::Number(i.into()));
    }
    add_field(
        mr.fields_as_mut(),
        "collection_field",
        model::value::Value::Collection(collection.clone()),
    );

    let to = record_to_tera_object(&mr);
    //println!("{:?}", to);

    assert_eq!("string_value", to["string_field"]);
    assert_eq!(73, to["int_field"]);
    assert_eq!(73.42, to["float_field"]);
    assert_eq!(tera::Value::Array(expected), to["collection_field"]);
}
