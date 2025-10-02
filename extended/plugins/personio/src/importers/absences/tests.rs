use model::{import::handlers::ClosureRecordHandler, record::Record};
use personio_rs::personnel::models::{
    Absence, AbsencePeriodsResponse, AbsencePeriodsResponseAllOfData, AbsenceTimeOffType,
    AbsenceTimeOffTypeAttributes, ShortEmployee, ShortEmployeeAttributes,
};
use serde_json::json;

use crate::importers::absences::{Absences, add_employee, add_time_off_type};

fn absence(id: i32) -> AbsencePeriodsResponseAllOfData {
    let mut absence = Absence::new();
    absence.id = Some(id);
    let mut result = AbsencePeriodsResponseAllOfData::new(None, absence);
    result.additional_properties = [
        ("prop1".into(), json!("value1")),
        ("prop2".into(), json!("value2")),
        ("prop3".into(), json!("value3")),
    ]
    .into_iter()
    .collect();
    result
}

#[test]
fn test_handle_response() {
    let absences = Absences::new();
    let mut count = 0;
    let mut handler = ClosureRecordHandler::new(|r| {
        println!("{:?}", r);
        assert!(r.field_by_name("id").is_some());
        assert!(r.field_by_name("prop1").is_some());
        count += 1;
    });
    let page = AbsencePeriodsResponse::new(true, vec![absence(1), absence(2)]);

    let result = absences.handle_response(&mut handler, page);
    println!("{:?}", result);
    assert!(result.is_ok());
    assert_eq!(count, 2);
}

#[test]
fn test_add_time_off_type() {
    let mut record = Record::new();
    let mut time_off_type = AbsenceTimeOffType::new();
    time_off_type.attributes = Some(AbsenceTimeOffTypeAttributes::new().into());
    add_time_off_type(&mut record, &Some(time_off_type.into()));
    assert!(record.field_by_name("time_off_type").is_some());
}

#[test]
fn test_add_employee() {
    let mut record = Record::new();
    let mut employee = ShortEmployee::new();
    employee.attributes = Some(ShortEmployeeAttributes::new().into());
    add_employee(&mut record, &Some(employee.into()));
    assert!(record.field_by_name("employee").is_some());
}
