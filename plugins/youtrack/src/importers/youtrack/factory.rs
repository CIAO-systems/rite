use std::collections::HashMap;

use model::{field::Field, record::Record, value::Value};
use serde::Serialize;

#[derive(Debug)]
pub enum YouTrackObject {
    Issue(super::issue::Issue),
    IssueWorkItem(super::issue_work_item::IssueWorkItem),
    User(super::common::user::User),
    Project(super::common::project::Project),
    // DurationValue(super::common::duration::DurationValue),
    None,
}

impl YouTrackObject {
    /// Create a rust object from a JSON Value, based on the YouTrack $type
    /// 
    pub fn from_type(element: &serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(object) = element.as_object() {
            let object_type = &object["$type"];

            if let Some(object_type) = object_type.as_str() {
                return match object_type {
                    "Issue" => Ok(YouTrackObject::Issue(create_object(element)?)),
                    "IssueWorkItem" => Ok(YouTrackObject::IssueWorkItem(create_object(element)?)),
                    "User" => Ok(YouTrackObject::User(create_object(element)?)),
                    "Project" => Ok(YouTrackObject::Project(create_object(element)?)),
                    _ => Ok(YouTrackObject::None),
                };
            }
        }
        Ok(YouTrackObject::None)
    }
}

/// Creates a rust object from a JSON Value
/// 
/// # Arguments
/// * `element`: The JSON Value to deserialize
/// 
fn create_object<T: serde::de::DeserializeOwned>(
    element: &serde_json::Value,
) -> Result<T, Box<dyn std::error::Error>> {
    match serde_json::from_value::<T>(element.clone()) {
        Ok(object) => Ok(object),
        Err(e) => {
            log::debug!("create_object: {:?}", element);
            Err(e.into())
        }
    }
}

/// Creates a [Record] from a JSON object
/// 
/// This functions takes any object, that implements the trait [Serialize], 
/// converts it to a raw JSON object and then adds all fields to a new [Record]
/// 
/// # Arguments
/// * `value`: A serde::Serialize object that will be converted to a JSON object
/// 
pub fn json_to_record<T: Serialize>(value: T) -> Record {
    let mut record = Record::new();
    if let Ok(json) = serde_json::to_value(value) {
        if let Some(object) = json.as_object() {
            for (name, json_value) in object {
                let value = match json_value {
                    serde_json::Value::Object(_) => {
                        // add all composite fields with prefix
                        let fields = json_object_to_value_map(name, json_value);
                        for (name_as_prefix, value) in fields {
                            add_field_value(&mut record, &name_as_prefix, value);
                        }
                        Value::None
                    }
                    _ => json2model(json_value),
                };

                add_field_value(&mut record, name, value);
            }
        }
    }
    record
}

/// Converts a JSON Value to a model Value
/// 
/// # Arguments
/// * `json_value`: The JSON value to be converted to a `model::Value`
/// 
fn json2model(json_value: &serde_json::Value) -> Value {
    match json_value {
        serde_json::Value::Null => Value::None,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(number) => {
            if let Some(n) = number.as_u64() {
                Value::U64(n)
            } else if let Some(n) = number.as_i64() {
                Value::I64(n)
            } else if let Some(n) = number.as_f64() {
                Value::F64(n)
            } else {
                Value::None
            }
        }
        serde_json::Value::String(s) => Value::String(s.to_string()),
        serde_json::Value::Array(_) => Value::None,
        serde_json::Value::Object(_) => Value::None,
    }
}

/// Adds a field to the record with given name and value
/// 
/// The field is only added, it its name does not start with `$` and its value 
/// is not Value::None
/// 
/// # Arguments
/// * `record`: The record to add the field to
/// * `name`: Name of the new field
/// * `value`: [Value] of the new field
fn add_field_value(record: &mut Record, name: &str, value: Value) {
    if !name.starts_with('$') && value != Value::None {
        record
            .fields_as_mut()
            .push(Field::new_value(name.to_string(), value));
    }
}

/// Creates multiple fields (a hashmap of name and value) for a JSON Object value
/// 
/// Only if the `object` actually is an Object, the fields will be added. If it is not an
/// object, an empty maps will be returned.
/// The attributes of the JSON object will be added only, if the name does not start with `$`.
/// The resulting map key will be prefixed with `prefix.`
/// 
/// # Arguments
/// * `prefix`: The name for every field in the JSON object will prefixed with, 
///        separated by a dot
/// * `object`: The JSON object. 
pub fn json_object_to_value_map(prefix: &str, object: &serde_json::Value) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    if let Some(object) = object.as_object() {
        for (name, json_value) in object {
            if !name.starts_with('$') {
                result.insert(format!("{prefix}.{name}"), json2model(json_value));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use model::value::Value;
    use serde_json::json;

    use super::{json2model, json_object_to_value_map};

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
}
