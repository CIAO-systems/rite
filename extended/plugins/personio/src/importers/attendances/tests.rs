use std::collections::HashMap;

use model::{import::handlers::ClosureRecordHandler, record::Record};
use personio_rs::personnel::models::{
    Attendance, AttendancePeriodsResponse, AttendancePeriodsResponseAllOfData, AttendanceProject,
    AttendanceProjectAttributes, attendance::Status,
};
use serde_json::json;

use crate::importers::attendances::{Attendances, add_project, add_status};

fn attendance(id: i32) -> AttendancePeriodsResponseAllOfData {
    AttendancePeriodsResponseAllOfData {
        id,
        r#type: None,
        attributes: Attendance::new().into(),
        additional_properties: [
            ("att_prop1".into(), json!("value1")),
            ("att_prop2".into(), json!("value2")),
            ("att_prop3".into(), json!("value3")),
        ]
        .into_iter()
        .collect(),
    }
}

#[test]
fn test_handle_attendance_response() {
    let attendances = Attendances::new();
    let mut handler = ClosureRecordHandler::new(|r| {
        println!("{:?}", r);
        assert!(r.field_by_name("id").is_some());
        assert!(r.field_by_name("att_prop1").is_some());
        assert!(r.field_by_name("att_prop2").is_some());
        assert!(r.field_by_name("att_prop3").is_some());
    });

    let page = AttendancePeriodsResponse {
        success: true,
        data: vec![attendance(1), attendance(2)],
        metadata: None,
        offset: None,
        limit: None,
        additional_properties: [
            ("prop1".into(), json!("value1")),
            ("prop2".into(), json!("value2")),
            ("prop3".into(), json!("value3")),
        ]
        .into_iter()
        .collect(),
    };
    let result = attendances.handle_attendance_response(&mut handler, page);
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[test]
fn test_add_project() {
    let mut record = Record::new();
    let mut attributes = attendance(1).attributes;
    attributes.project = Some(Some(
        AttendanceProject {
            id: None,
            r#type: None,
            attributes: Some(AttendanceProjectAttributes::new().into()),
            additional_properties: HashMap::new(),
        }
        .into(),
    ));
    add_project(&mut record, &attributes);

    assert!(record.field_by_name("project").is_some());
}

#[test]
fn test_add_status() {
    let mut record = Record::new();
    let mut attributes = Box::new(Attendance::new());
    attributes.status = Some(Status::Confirmed.into());
    add_status(&mut record, &attributes);

    assert!(
        record
            .field_by_name("status")
            .is_some_and(|f| f.value().to_string() == "Confirmed")
    );
}
