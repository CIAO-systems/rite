use std::time::{SystemTime, UNIX_EPOCH};

use import::handlers::{ClosureRecordHandler, CollectingRecordHandler};
use model::record::Record;

use super::*;
use crate::importers::youtrack::{
    common::{duration::DurationValue, project::Project, user::User},
    issue::Issue,
};

fn create_project(id: &str, name: Option<&str>) -> Project {
    Project {
        object_type: "Project".to_string(),
        id: id.to_string(),
        name: name.map(|s| s.to_string()),
    }
}

fn create_issue(
    id_readable: Option<&str>,
    summary: Option<&str>,
    project: Option<Project>,
) -> Issue {
    Issue {
        object_type: "Issue".to_string(),
        project: project,
        id: "issue-id".to_string(),
        id_readable: id_readable.map(|s| s.to_string()),
        summary: summary.map(|s| s.to_string()),
        comments_count: None,
        description: None,
        created: None,
        draft_owner: None,
        is_draft: None,
        number_in_project: None,
        resolved: None,
        updated: None,
        updater: None,
        votes: None,
        wikified_description: None,
    }
}

fn create_issue_item(issue: Option<Issue>) -> IssueWorkItem {
    IssueWorkItem {
        issue: issue,
        object_type: "IssueWorkItem".to_string(),
        id: "id".to_string(),
        author: None,
        creator: None,
        created: None,
        updated: None,
        date: None,
        duration: None,
        work_item_type: None,
        text: None,
        text_preview: None,
    }
}

fn create_author(email: Option<&str>) -> User {
    User {
        object_type: "User".to_string(),
        id: "id".to_string(),
        login: None,
        full_name: None,
        email: email.map(|s| s.to_string()),
    }
}

fn create_duration(minutes: Option<i32>) -> DurationValue {
    DurationValue {
        object_type: "DurationValue".to_string(),
        id: None,
        minutes: minutes,
        presentation: None,
    }
}

#[test]
fn test_add_project_with_complete_data() {
    // Arrange
    let mut record = Record::new();
    let project = create_project("proj-123", Some("Test Project"));
    let issue = create_issue(None, None, Some(project));
    let issue_work_item = create_issue_item(Some(issue));

    // Act
    add_project(&mut record, &issue_work_item);

    // Assert
    let fields = record.fields();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name(), "project");
    assert_eq!(fields[0].value(), Value::String("proj-123".to_string()));

    assert_eq!(fields[1].name(), "project.name");
    assert_eq!(fields[1].value(), Value::String("Test Project".to_string()));
}

#[test]
fn test_add_project_without_name() {
    // Arrange
    let mut record = Record::new();
    let project = create_project("proj-123", None);
    let issue = create_issue(None, None, Some(project));
    let issue_work_item = create_issue_item(Some(issue));

    // Act
    add_project(&mut record, &issue_work_item);

    // Assert
    let fields = record.fields();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name(), "project");
    assert_eq!(fields[0].value(), Value::String("proj-123".to_string()));
}

#[test]
fn test_add_project_without_project() {
    // Arrange
    let mut record = Record::new();
    let issue = create_issue(None, None, None);
    let issue_work_item = create_issue_item(Some(issue));

    // Act
    add_project(&mut record, &issue_work_item);

    // Assert
    let fields = record.fields();
    assert_eq!(fields.len(), 0);
}

#[test]
fn test_add_project_without_issue() {
    // Arrange
    let mut record = Record::new();
    let issue_work_item = create_issue_item(None);

    // Act
    add_project(&mut record, &issue_work_item);

    // Assert
    let fields = record.fields();
    assert_eq!(fields.len(), 0);
}

#[test]
fn test_add_issue_with_id_and_summary() {
    // Arrange
    let mut record = Record::new();
    let issue = create_issue(Some("ISSUE-123"), Some("My Issue Summary"), None);
    let issue_work_item = create_issue_item(Some(issue));

    // Act
    add_issue(&mut record, &issue_work_item);

    println!("{:#?}", record);
    // Assert
    assert!(record
        .field_by_name("issue")
        .is_some_and(|v| v.value().eq(&Value::String("ISSUE-123".to_string()))));
    assert!(record
        .field_by_name("issue.summary")
        .is_some_and(|v| v.value().eq(&Value::String("My Issue Summary".to_string()))));
}

#[test]
fn test_add_issue_with_id_only() {
    // Arrange
    let mut record = Record::new();
    let issue = create_issue(Some("ISSUE-456"), None, None);
    let issue_work_item = create_issue_item(Some(issue));

    // Act
    add_issue(&mut record, &issue_work_item);

    // Assert
    assert!(record
        .field_by_name("issue")
        .is_some_and(|v| v.value().eq(&Value::String("ISSUE-456".to_string()))));
    assert!(record.field_by_name("issue.summary").is_none());
}

#[test]
fn test_add_issue_with_summary_only() {
    // Arrange
    let mut record = Record::new();
    let issue = create_issue(None, Some("Another Issue Summary"), None);
    let issue_work_item = create_issue_item(Some(issue));

    // Act
    add_issue(&mut record, &issue_work_item);

    // Assert
    assert!(record.field_by_name("issue").is_none());
    assert!(record.field_by_name("issue.summary").is_some_and(|v| v
        .value()
        .eq(&Value::String("Another Issue Summary".to_string()))));
}

#[test]
fn test_add_issue_with_no_issue() {
    // Arrange
    let mut record = Record::new();
    let issue_work_item = create_issue_item(None);

    // Act
    add_issue(&mut record, &issue_work_item);

    // Assert
    assert_eq!(record.fields().len(), 0);
}

#[test]
fn test_add_created() {
    // Arrange
    let mut record = Record::new();
    let mut issue_work_item = create_issue_item(None);
    let mut millis = 0;
    if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
        millis = now.as_millis() as i64;
        issue_work_item.created = Some(millis);
    }

    // Act
    add_created(&mut record, &issue_work_item);

    // Assert
    assert_eq!(record.fields().len(), 1);
    assert!(record
        .field_by_name("created")
        .is_some_and(|v| v.value().eq(&Value::I64(millis))));
    assert_eq!(record.fields()[0].value(), Value::I64(millis));
}

#[test]
fn test_add_created_none() {
    // Arrange
    let mut record = Record::new();
    let issue_work_item = create_issue_item(None);

    // Act
    add_created(&mut record, &issue_work_item);

    // Assert
    assert_eq!(record.fields().len(), 0);
}

#[test]
fn test_add_author() {
    // Arrange
    let mut record = Record::new();
    let mut issue_work_item = create_issue_item(None);
    issue_work_item.author = Some(create_author(Some("author@writers.room")));

    // Act
    add_author(&mut record, &issue_work_item);

    // Assert
    assert_eq!(record.fields().len(), 1);
    assert!(record.field_by_name("email").is_some_and(|v| v
        .value()
        .eq(&Value::String("author@writers.room".to_string()))));
}

#[test]
fn test_add_author_none() {
    // Arrange
    let mut record = Record::new();
    let mut issue_work_item = create_issue_item(None);
    issue_work_item.author = Some(create_author(None));

    // Act
    add_author(&mut record, &issue_work_item);

    // Assert
    assert_eq!(record.fields().len(), 0);
}

#[test]
fn test_add_duration() {
    // Arrange
    let mut record = Record::new();
    let mut issue_work_item = create_issue_item(None);
    issue_work_item.duration = Some(create_duration(Some(73)));

    // Act
    add_duration(&mut record, &issue_work_item);

    // Assert
    assert_eq!(record.fields().len(), 1);
    assert!(record
        .field_by_name("minutes")
        .is_some_and(|v| v.value().eq(&Value::I32(73))));
}

#[test]
fn test_add_duration_none() {
    // Arrange
    let mut record = Record::new();
    let mut issue_work_item = create_issue_item(None);
    issue_work_item.duration = Some(create_duration(None));

    // Act
    add_duration(&mut record, &issue_work_item);

    // Assert
    assert_eq!(record.fields().len(), 0);
}

#[test]
fn test_handle_issue_work_item() {
    // Arrange
    let project = create_project("project-id", Some("project-name"));
    let issue = create_issue(Some("issue-id"), Some("issue-summary"), Some(project));
    let mut issue_work_item = create_issue_item(Some(issue));
    issue_work_item.author = Some(create_author(Some("issue-author@somewhere")));
    issue_work_item.duration = Some(create_duration(Some(8472)));

    let mut captured_record: Option<Record> = None;
    let mut handler = ClosureRecordHandler::new(|record| {
        captured_record = Some(Record::copy(&record));
    });

    // Act
    let result = handle_issue_work_item(issue_work_item, &mut handler);

    // Assert
    assert!(result.is_ok());
    assert!(captured_record.is_some());
    if let Some(record) = captured_record {
        assert_eq!(record.fields().len(), 6);
        assert_eq!(
            record.field_by_name("issue").unwrap().value(),
            Value::String("issue-id".to_string())
        );
        assert_eq!(
            record.field_by_name("issue.summary").unwrap().value(),
            Value::String("issue-summary".to_string())
        );
        assert_eq!(
            record.field_by_name("project").unwrap().value(),
            Value::String("project-id".to_string())
        );
        assert_eq!(
            record.field_by_name("project.name").unwrap().value(),
            Value::String("project-name".to_string())
        );
        assert_eq!(
            record.field_by_name("email").unwrap().value(),
            Value::String("issue-author@somewhere".to_string())
        );
        assert_eq!(
            record.field_by_name("minutes").unwrap().value(),
            Value::I32(8472)
        );
    }
}

#[test]
fn test_handle_json_response() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);

    let content = std::fs::read_to_string("../../../data/test/time/tracking.json")?;
    let json: serde_json::Value = serde_json::from_str(&content)?;

    // Act
    handle_json_response(&mut handler, json)?;

    // Assert
    assert_eq!(3, records.len());

    let record = records.get(0);
    assert!(record.is_some());

    assert_eq!(
        record.unwrap().field_by_name("issue").unwrap().value(),
        Value::String("RIT-3".to_string())
    );

    assert_eq!(
        record.unwrap().field_by_name("email").unwrap().value(),
        Value::String("chuck.norris@ciao-systems.com".to_string())
    );
    assert_eq!(
        record
            .unwrap()
            .field_by_name("issue.summary")
            .unwrap()
            .value(),
        Value::String("YouTrack import plugin".to_string())
    );
    assert_eq!(
        record.unwrap().field_by_name("project").unwrap().value(),
        Value::String("0-11".to_string())
    );
    assert_eq!(
        record
            .unwrap()
            .field_by_name("project.name")
            .unwrap()
            .value(),
        Value::String("Rust Import/Transform/Export".to_string())
    );
    assert_eq!(
        record.unwrap().field_by_name("minutes").unwrap().value(),
        Value::I32(240)
    );
    assert_eq!(
        record.unwrap().field_by_name("created").unwrap().value(),
        Value::I64(1735898372898)
    );

    Ok(())
}
