use ciao_rs::ciao::{common::Date, time_tracking::absences::Absence};
use model::{import::handlers::ClosureRecordHandler, record::Record, xml::config::Configuration, Initializable};

use crate::importers::absences::{add_date_field, handle_absence, Absences};

#[test]
fn test_init() {
    let mut importer = Absences::new();
    let config = Configuration::new();
    let result = importer.init(Some(config));
    assert!(result.is_ok());
}

#[test]
fn test_add_date_field_none() {
    let mut record = Record::new();
    let result = add_date_field(record.fields_as_mut(), "name", None);
    assert!(result.is_ok());

    // Field should not have been added
    assert!(record.field_by_name("name").is_none());
}

#[test]
fn test_add_date_field_some() {
    let mut record = Record::new();
    let result = add_date_field(
        record.fields_as_mut(),
        "name",
        Some(Date {
            year: 2025,
            month: 12,
            day: 1,
        }),
    );
    assert!(result.is_ok());

    // Field should have been added
    let field = record.field_by_name("name");
    assert!(field.is_some());

    let field = field.unwrap();
    assert_eq!(field.value(), "2025-12-01".into());
}

#[test]
fn test_handle_absence() {
    let absence = Absence {
        id: "id".into(),
        start_date: None,
        end_date: None,
        start_half_day: false,
        end_half_day: false,
        time_type_id: "time-type-id".into(),
        user_id: "user-id".into(),
        deleted: false,
    };
    let mut handler = ClosureRecordHandler::new(|r| {
        assert!(r.field_by_name("startDate").is_none());
        assert!(r.field_by_name("endDate").is_none());
        assert_eq!(r.field_by_name("id").unwrap().value().to_string(), "id");
        assert_eq!(r.field_by_name("timeTypeId").unwrap().value().to_string(), "time-type-id");
        assert_eq!(r.field_by_name("userId").unwrap().value().to_string(), "user-id");
        assert_eq!(r.field_by_name("startHalfDay").unwrap().value(), false.into());
        assert_eq!(r.field_by_name("endHalfDay").unwrap().value(), false.into());
        assert_eq!(r.field_by_name("deleted").unwrap().value(), false.into());

    });
    let result = handle_absence(&absence, &mut handler);
    assert!(result.is_ok());
}
