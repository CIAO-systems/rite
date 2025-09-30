use model::{
    export::Exporter, field::add_field, record::Record, xml::config::Configuration, BoxedError, Initializable
};
use uuid::Uuid;

use crate::exporters::badges::{badge_from_record, Badges};

#[test]
fn test_init() -> Result<(), BoxedError> {
    let mut exporter = Badges::new();
    let config = Configuration::new();
    let result = exporter.init(Some(config));
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
    Ok(())
}

#[test]
fn test_write() -> Result<(), BoxedError> {
    let mut exporter = Badges::new();
    let record = Record::new();
    let result = exporter.write(&record);
    println!("{:?}", result);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_badge_from_record_missing_user_id() -> Result<(), BoxedError> {
    let record = Record::new();
    let result = badge_from_record(&record);
    println!("{:?}", result);
    assert!(result.is_err_and(|e|e.to_string() == "Mandatory field 'userId' not found"));

    Ok(())
}

#[test]
fn test_badge_from_record_missing_external_id() -> Result<(), BoxedError> {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "userId", "value for user-id".into());
    let result = badge_from_record(&record);
    println!("{:?}", result);
    assert!(result.is_err_and(|e|e.to_string() == "Mandatory field 'externalId' not found"));
    // let badge = result.unwrap();
    // println!("{:?}", badge);

    Ok(())
}

#[test]
fn test_badge_from_record() -> Result<(), BoxedError> {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "userId", "value for user-id".into());
    add_field(record.fields_as_mut(), "externalId", "value for external-id".into());
    add_field(record.fields_as_mut(), "description", "value for description".into());
    let result = badge_from_record(&record);
    println!("{:?}", result);
    assert!(result.is_ok());
    let badge = result.unwrap();
    println!("{:?}", badge);
    assert_eq!(badge.user_id, "value for user-id");
    assert_eq!(badge.external_id, "value for external-id");
    assert_eq!(badge.description.unwrap(), "value for description");
    assert!(Uuid::parse_str(&badge.id).is_ok());

    Ok(())
}
