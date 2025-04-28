use std::collections::HashMap;

use import::handlers::CollectingRecordHandler;
use model::{BoxedError, value::Value};
use personio_rs::personnel::models::{
    CompanyAttendancesProjectsGet200Response, Project, ProjectAttributes,
};

use super::Projects;

#[test]
fn test_handle_response_failure() -> Result<(), BoxedError> {
    let projects = Projects::new();
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);
    let response = CompanyAttendancesProjectsGet200Response {
        success: Some(false),
        data: None,
        additional_properties: HashMap::new(),
    };

    let result = projects.handle_response(response, &mut handler);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "We got an project response, but it was unsuccessful"
    );

    Ok(())
}

#[test]
fn test_handle_response_no_data() -> Result<(), BoxedError> {
    let projects = Projects::new();
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);
    let response = CompanyAttendancesProjectsGet200Response {
        success: Some(true),
        data: None,
        additional_properties: HashMap::new(),
    };

    let result = projects.handle_response(response, &mut handler);
    assert!(result.is_ok());
    assert_eq!(records.len(), 0);

    Ok(())
}

#[test]
fn test_handle_response_no_attributes() -> Result<(), BoxedError> {
    let projects = Projects::new();
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);
    let response = CompanyAttendancesProjectsGet200Response {
        success: Some(true),
        data: Some(vec![Project {
            id: Some(73),
            r#type: Some("Project".to_string()),
            attributes: None,
            additional_properties: HashMap::new(),
        }]),
        additional_properties: HashMap::new(),
    };

    let result = projects.handle_response(response, &mut handler);
    assert!(result.is_ok());
    assert_eq!(records.len(), 1);

    let record = &records[0];
    assert_eq!(record.field_by_name("id").unwrap().value(), Value::I32(73));
    assert_eq!(record.field_by_name("name"), None);
    assert_eq!(record.field_by_name("active"), None);
    assert_eq!(record.field_by_name("created_at"), None);
    assert_eq!(record.field_by_name("updated_at"), None);
    Ok(())
}

#[test]
fn test_handle_response_all_attributes() -> Result<(), BoxedError> {
    let projects = Projects::new();
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);
    let response = CompanyAttendancesProjectsGet200Response {
        success: Some(true),
        data: Some(vec![Project {
            id: Some(73),
            r#type: Some("Project".to_string()),
            attributes: Some(Box::new(ProjectAttributes {
                name: Some("Name".to_string()),
                active: Some(true),
                created_at: Some("2025-01-01T00:00:00".to_string()),
                updated_at: Some("2025-01-01T00:00:00".to_string()),
                additional_properties: HashMap::new(),
            })),
            additional_properties: HashMap::new(),
        }]),
        additional_properties: HashMap::new(),
    };

    let result = projects.handle_response(response, &mut handler);
    assert!(result.is_ok());
    assert_eq!(records.len(), 1);

    let record = &records[0];
    assert_eq!(record.field_by_name("id").unwrap().value(), Value::I32(73));
    assert_eq!(
        record.field_by_name("name").unwrap().value(),
        Value::String("Name".to_string())
    );
    assert_eq!(
        record.field_by_name("active").unwrap().value(),
        Value::Bool(true)
    );
    assert_eq!(
        record.field_by_name("created_at").unwrap().value(),
        Value::String("2025-01-01T00:00:00".to_string())
    );
    assert_eq!(
        record.field_by_name("updated_at").unwrap().value(),
        Value::String("2025-01-01T00:00:00".to_string())
    );
    Ok(())
}
