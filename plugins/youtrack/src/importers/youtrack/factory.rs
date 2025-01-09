use model::{field::Field, record::Record, value::Value};
use serde::Serialize;

#[derive(Debug)]
pub enum YouTrackObject {
    Issue(super::issue::Issue),
    IssueWorkItem(super::work_item::IssueWorkItem),
    User(super::common::user::User),
    DurationValue(super::common::duration::DurationValue),
    Project(super::common::project::Project),
    None,
}

impl YouTrackObject {
    pub fn from_type(element: &serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(object) = element.as_object() {
            let object_type = &object["$type"];

            if let Some(object_type) = object_type.as_str() {
                return match object_type {
                    "Issue" => Ok(YouTrackObject::Issue(create_object(element)?)),
                    "IssueWorkItem" => Ok(YouTrackObject::IssueWorkItem(create_object(element)?)),
                    "User" => Ok(YouTrackObject::User(create_object(element)?)),
                    _ => Ok(YouTrackObject::None),
                };
            }
        }
        Ok(YouTrackObject::None)
    }
}

fn create_object<T: serde::de::DeserializeOwned>(
    element: &serde_json::Value,
) -> Result<T, Box<dyn std::error::Error>> {
    match serde_json::from_value::<T>(element.clone()) {
        Ok(object) => Ok(object),
        Err(e) => Err(e.into()),
    }
}

pub fn json_to_record<T: Serialize>(value: T) -> Record {
    let mut record = Record::new();
    if let Ok(json) = serde_json::to_value(value) {
        if let Some(object) = json.as_object() {
            for (name, json_value) in object {
                let value = match json_value {
                    serde_json::Value::Null => Value::None,
                    serde_json::Value::Bool(b) => Value::Bool(*b),
                    serde_json::Value::Number(number) => {
                        match (number.as_f64(), number.as_i64(), number.as_u64()) {
                            (Some(n), None, None) => Value::F64(n),
                            (None, Some(n), None) => Value::I64(n),
                            (None, None, Some(n)) => Value::U64(n),
                            _ => Value::None,
                        }
                    }
                    serde_json::Value::String(s) => Value::String(s.to_string()),
                    serde_json::Value::Array(_) => Value::None, // TODO maybe implement this some day
                    serde_json::Value::Object(_) => Value::None, // TODO maybe implement this some day
                };

                if !name.starts_with('$') && value != Value::None {
                    record
                        .fields_as_mut()
                        .push(Field::new_value(name.to_string(), value));
                }
            }
        }
    }
    record
}
