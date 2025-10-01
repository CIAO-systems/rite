use model::{field::add_field, record::Record, xml::config::Configuration, Initializable};

use crate::exporters::absences::{absence_from_record, Absences};

#[test]
fn test_init() {
    let mut exporter = Absences::new();
    let config = Configuration::new();
    let result = exporter.init(Some(config));
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"))
}

#[test]
fn test_absence_from_record() {
    let mut record = Record::new();
    // Add mandatory fields
    // "startDate"
    // "endDate"
    // "timeTypeId"
    // "userId"
    let fields = record.fields_as_mut();
    add_field(fields, "startDate", "2025-01-01".into());
    add_field(fields, "endDate", "2025-12-31".into());
    add_field(fields, "timeTypeId", "timeTypeId-value".into());
    add_field(fields, "userId", "userId-value".into());
    let result = absence_from_record(&record);
    println!("{:?}", result);
    assert!(result.is_ok());
}
