use super::{project_from_record, Projects};
use crate::model::add_timestamp_parse;
use model::export::Exporter;
use model::{
    field::add_field, record::Record, value::Value, xml::config::Configuration, BoxedError,
    Initializable,
};

fn create_test_record() -> Result<Record, BoxedError> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::String("project-id".to_string()));
    add_field(
        fields,
        "externalId",
        Value::String("external-id".to_string()),
    );
    add_field(fields, "name", Value::String("project-name".to_string()));
    add_timestamp_parse(fields, "startDate", "2025-02-01 08:00", "%Y-%m-%d %H:%M")?;
    add_timestamp_parse(fields, "endDate", "2025-02-28 23:00", "%Y-%m-%d %H:%M")?;
    add_timestamp_parse(fields, "closedDate", "2025-03-01 00:00", "%Y-%m-%d %H:%M")?;
    add_field(fields, "parentId", Value::String("parent-id".to_string()));
    Ok(record)
}

#[test]
fn test_project_from_record() -> Result<(), BoxedError> {
    let record = create_test_record()?;
    let project = project_from_record(&record);
    assert_eq!(project.id, "project-id");
    assert_eq!(project.external_id, Some("external-id".to_string()));
    assert_eq!(project.name, "project-name");
    assert_eq!(project.parent_id, Some("parent-id".to_string()));
    assert_date(project.start_date, 1738396800, None);
    assert_date(project.end_date, 1740783600, None);
    assert_date(project.closed_date, 1740787200, None);

    Ok(())
}

fn assert_date(v: Option<ciao_rs::ciao::common::Timestamp>, seconds: i64, tz: Option<&str>) {
    assert!(v.is_some());
    let v = v.unwrap();
    assert!(v.time_utc.is_some());
    if let Some(tz) = tz {
        assert_eq!(v.time_zone, tz);
    }
    let v = v.time_utc.unwrap();
    assert_eq!(v.seconds, seconds);
}

#[test]
#[ignore = "for manual testing"]
fn test_project_exporter() -> Result<(), BoxedError> {
    let mut exporter = Projects::new();
    let mut config = Configuration::new();
    config.insert_str("url", "http://localhost:50051");
    config.insert_str("api-key", "top-secret-api-key");

    exporter.init(Some(config))?;

    let record = create_test_record()?;

    exporter.write(&record)?;

    Ok(())
}

#[test]
fn test_init() -> Result<(), BoxedError> {
    let mut exporter = Projects::new();
    let config = Configuration::new();
    let result = exporter.init(Some(config));
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
    Ok(())
}

#[test]
fn test_write() -> Result<(), BoxedError> {
    let mut exporter = Projects::new();
    let record = Record::new();
    let result = exporter.write(&record);
    println!("{:?}", result);
    assert!(result.is_ok());
    Ok(())
}
