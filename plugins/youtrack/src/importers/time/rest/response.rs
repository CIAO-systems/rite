use model::{field::Field, record::Record, value::Value};

use crate::importers::youtrack::{
    factory::serialize::YouTrackObject, issue_work_item::IssueWorkItem,
};

/// A response handler for YouTrack work items
/// See https://www.jetbrains.com/help/youtrack/devportal/resource-api-workItems.html
pub fn handle(
    callback: import::RecordCallback,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = response.json::<serde_json::Value>()?;
    if let Some(array) = json.as_array() {
        for element in array {
            let data = YouTrackObject::from_type(element)?;
            match data {
                YouTrackObject::IssueWorkItem(issue_work_item) => {
                    handle_issue_work_item(issue_work_item, callback)?;
                }
                _ => {
                    // Ignore
                }
            }
        }
    }

    Ok(())
}

fn handle_issue_work_item(
    issue_work_item: IssueWorkItem,
    callback: import::RecordCallback,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();

    add_duration(&mut record, &issue_work_item);
    add_created(&mut record, &issue_work_item);
    add_author(&mut record, &issue_work_item);
    add_issue(&mut record, &issue_work_item);
    add_project(&mut record, &issue_work_item);

    callback(&record);

    Ok(())
}

fn add_project(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref issue) = issue_work_item.issue {
        if let Some(ref project) = issue.project {
            record.fields_as_mut().push(Field::new_value(
                "project".to_string(),
                Value::String(project.id.to_string()),
            ));

            if let Some(ref name) = project.name {
                record.fields_as_mut().push(Field::new_value(
                    "project.name".to_string(),
                    Value::String(name.to_string()),
                ));
            }
        }
    }
}

fn add_issue(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref issue) = issue_work_item.issue {
        if let Some(ref id) = issue.id_readable {
            record.fields_as_mut().push(Field::new_value(
                "issue".to_string(),
                Value::String(id.to_string()),
            ));
        }

        if let Some(ref summary) = issue.summary {
            record.fields_as_mut().push(Field::new_value(
                "issue.summary".to_string(),
                Value::String(summary.to_string()),
            ));
        }
    }
}

fn add_created(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref created) = issue_work_item.created {
        record.fields_as_mut().push(Field::new_value(
            "created".to_string(),
            Value::I64(*created),
        ));
    }
}

fn add_author(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref author) = issue_work_item.author {
        if let Some(ref email) = author.email {
            record
                .fields_as_mut()
                .push(Field::new_string("email".to_string(), email.to_string()));
        }
    }
}

fn add_duration(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref duration) = issue_work_item.duration {
        if let Some(minutes) = duration.minutes {
            record
                .fields_as_mut()
                .push(Field::new_i32("minutes".to_string(), minutes));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::importers::youtrack::{common::project::Project, issue::Issue};

    fn create_project(id: &str, name: Option<&str>) -> Project {
        Project {
            object_type: "Project".to_string(),
            id: id.to_string(),
            name: name.map(|s| s.to_string()),
        }
    }

    fn create_issue(project: Option<Project>) -> Issue {
        Issue {
            object_type: "Issue".to_string(),
            project: project,
            id: "issue-id".to_string(),
            id_readable: None,
            comments_count: None,
            description: None,
            created: None,
            draft_owner: None,
            is_draft: None,
            number_in_project: None,
            resolved: None,
            summary: None,
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

    #[test]
    fn test_add_project_with_complete_data() {
        // Arrange
        let mut record = Record::new();
        let project = create_project("proj-123", Some("Test Project"));
        let issue = create_issue(Some(project));
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
        let issue = create_issue(Some(project));
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
        let issue = create_issue(None);
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
}
