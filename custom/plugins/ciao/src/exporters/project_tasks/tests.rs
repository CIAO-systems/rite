use model::export::Exporter;
use model::{
    field::add_field, record::Record, value::Value, xml::config::Configuration, BoxedError,
    Initializable,
};

use crate::exporters::project_tasks::task_from_record;

use super::ProjectTasks;

fn create_test_record() -> Result<Record, BoxedError> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::String("task-id".to_string()));
    add_field(fields, "projectId", Value::String("project-id".to_string()));
    add_field(fields, "name", Value::String("task-name".to_string()));
    Ok(record)
}

#[test]
fn test_project_from_record() -> Result<(), BoxedError> {
    let record = create_test_record()?;
    let task = task_from_record(&record);
    assert_eq!(task.id, "task-id");
    assert_eq!(task.project_id, "project-id".to_string());
    assert_eq!(task.name, "task-name");
    Ok(())
}

#[test]
#[ignore = "for manual testing"]
fn test_project_task_exporter() -> Result<(), BoxedError> {
    let mut exporter = ProjectTasks::new();
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
    let mut exporter = ProjectTasks::new();
    let config = Configuration::new();
    let result = exporter.init(Some(config));
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
    Ok(())
}

#[test]
fn test_write() -> Result<(), BoxedError> {
    let mut exporter = ProjectTasks::new();
    let record = Record::new();
    let result = exporter.write(&record);
    println!("{:?}", result);
    assert!(result.is_ok());
    Ok(())
}
