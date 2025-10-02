use model::{record::Record, value::Value};
use serde::Deserialize;
use serde_json::json;

use crate::importers::youtrack::factory::{
    serialize::{YouTrackObject, create_object, derserialize_and_add},
    serialize_to_record,
};

static TEST_DATA: &str = r#"
[
  {
    "description": null,
    "summary": "Sprint 3. Task 2",
    "reporter": {
      "login": "root",
      "$type": "User"
    },
    "idReadable": "SP-38",
    "id": "2-42",
    "$type": "Issue"
  },
  {
    "description": "Let's create new issue from REST API",
    "summary": "Issue from REST #1",
    "reporter": {
      "login": "root",
      "$type": "User"
    },
    "idReadable": "SP-7",
    "id": "2-6",
    "$type": "Issue"
  }
]    
    "#;

#[test]
fn test_automatic_type_creation() -> Result<(), Box<dyn std::error::Error>> {
    let json: serde_json::Value = serde_json::from_str(TEST_DATA)?;
    // println!("{:#?}", json);

    assert!(json.is_array());

    match json.as_array() {
        Some(array) => {
            // Get object type from first element of the array
            assert!(array.len() > 0);
            // Iterate over the array
            for element in array {
                // Create a rust object from the JSON, based on $type
                let x = YouTrackObject::from_type(element)?;
                println!("{:#?}", x);
            }
        }
        None => panic!("Not an array"),
    }

    Ok(())
}

fn minimal_user() -> serde_json::Value {
    let json: &str = r#"{
        "$type": "User",
        "id": "42"
    }"#;
    serde_json::from_str(json).unwrap()
}

#[test]
fn test_user() {
    assert!(matches!(
        YouTrackObject::from_type(&minimal_user()).unwrap(),
        YouTrackObject::User(_)
    ));

    let result = serialize_to_record(minimal_user());
    assert_eq!(
        result.field_by_name("id").unwrap().value(),
        model::value::Value::String("42".into())
    );
}

fn minimal_project() -> serde_json::Value {
    let json: &str = r#"{
        "$type": "Project",
        "id": "42-73"
    }"#;
    serde_json::from_str(json).unwrap()
}

#[test]
fn test_project() {
    assert!(matches!(
        YouTrackObject::from_type(&minimal_project()).unwrap(),
        YouTrackObject::Project(_)
    ));
    let result = serialize_to_record(minimal_project());
    assert_eq!(
        result.field_by_name("id").unwrap().value(),
        model::value::Value::String("42-73".into())
    );
}

fn undefined() -> serde_json::Value {
    let json: &str = r#"{
        "$type": "Anything",
        "id": "r2d2"
    }"#;
    serde_json::from_str(json).unwrap()
}

#[test]
fn test_undefined() {
    assert!(matches!(
        YouTrackObject::from_type(&undefined()).unwrap(),
        YouTrackObject::None
    ));
    let result = serialize_to_record(undefined());
    assert_eq!(
        result.field_by_name("id").unwrap().value(),
        model::value::Value::String("r2d2".into())
    );
}

fn typed_element(t: &str) -> serde_json::Value {
    json!({
        "$type": t,
        "id": "42"
    })
}

fn record_asserting_callback(record: &Record) {
    println!("{:?}", record);
    assert_eq!(
        record.field_by_name("id").unwrap().value(),
        Value::String("42".into())
    );
}

#[test]
fn test_derserialize_and_add() {
    let result = derserialize_and_add(&mut record_asserting_callback, &typed_element("User"));
    assert!(result.is_ok());
}

#[test]
fn test_derserialize_and_add_issue() {
    let result = derserialize_and_add(&mut record_asserting_callback, &typed_element("Issue"));
    assert!(result.is_ok());
}

#[test]
fn test_derserialize_and_add_issue_work_item() {
    let result = derserialize_and_add(
        &mut record_asserting_callback,
        &typed_element("IssueWorkItem"),
    );
    assert!(result.is_ok());
}

#[test]
fn test_derserialize_and_add_project() {
    let result = derserialize_and_add(&mut record_asserting_callback, &typed_element("Project"));
    assert!(result.is_ok());
}

#[test]
fn test_derserialize_and_add_undefined() {
    let result = derserialize_and_add(&mut record_asserting_callback, &typed_element("Undefined"));
    assert!(result.is_ok());
}

#[test]
fn test_create_object_err() {
    #[derive(Debug, Deserialize)]
    struct TestStruct {
        id: u32,
    }

    // `id` should be a number, but here it's a string → deserialization fails
    let bad_value = json!({
        "id": "not_a_number"
    });

    let result: Result<TestStruct, _> = create_object(&bad_value);

    assert!(result.is_err(), "Expected deserialization to fail");
}
