use model::export::Exporter;
use model::{
    field::add_field, record::Record, value::Value, xml::config::Configuration, BoxedError,
    Initializable,
};

use crate::exporters::cost_centers::cost_center_from;

use super::CostCenters;

#[test]
#[ignore = "for manual testing"]
fn test_create_cost_center_manual() -> Result<(), BoxedError> {
    let mut exporter = CostCenters::new();
    let mut config = Configuration::new();
    config.insert_str("url", "http://localhost:50051");
    config.insert_str("api-key", "top-secret-api-key");

    exporter.init(Some(config))?;

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(
        fields,
        "id",
        Value::String("new-cost-center-id".to_string()),
    );
    add_field(
        fields,
        "name",
        Value::String("new-cost-center-name".to_string()),
    );

    exporter.write(&record)?;

    Ok(())
}

#[test]
fn test_init() -> Result<(), BoxedError> {
    let mut exporter = CostCenters::new();
    let config = Configuration::new();
    let result = exporter.init(Some(config));
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
    Ok(())
}

#[test]
fn test_write() -> Result<(), BoxedError> {
    let mut exporter = CostCenters::new();
    let record = Record::new();
    let result = exporter.write(&record);
    println!("{:?}", result);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_cost_center_from_missing_mandatory() -> Result<(), BoxedError> {
    let record = Record::new();
    let result = cost_center_from(&record);
    println!("{:?}", result);
    assert!(result.is_err_and(|e|e.to_string()=="Missing mandatory fields"));

    Ok(())
}

#[test]
fn test_cost_center_from() -> Result<(), BoxedError> {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "name", "name-value".into());
    add_field(record.fields_as_mut(), "id", "id-value".into());
    let result = cost_center_from(&record);
    println!("{:?}", result);
    assert!(result.is_ok());

    let cost_center = result.unwrap();
    assert_eq!(cost_center.id, "id-value");
    assert_eq!(cost_center.name, "name-value");
    assert!(cost_center.external_id.is_none());
    Ok(())
}
