use model::import::RecordHandler;
use model::{field::Field, record::Record, value::Value};

use crate::importers::youtrack::{
    factory::serialize::YouTrackObject, issue_work_item::IssueWorkItem,
};

/// A response handler for YouTrack work items
/// See https://www.jetbrains.com/help/youtrack/devportal/resource-api-workItems.html
/// # Arguments
/// * `callback`: The callback for imported records
/// * `response`: The response from the request. It will be processed as JSON body
pub fn handle(
    handler: &mut dyn RecordHandler,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = response.json::<serde_json::Value>()?;
    handle_json_response(handler, json)
}

// Extracted, to be testable
fn handle_json_response(
    handler: &mut dyn RecordHandler,
    json: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(array) = json.as_array() {
        for element in array {
            let data = YouTrackObject::from_type(element)?;
            match data {
                YouTrackObject::IssueWorkItem(issue_work_item) => {
                    handle_issue_work_item(issue_work_item, handler)?;
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
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();

    add_duration(&mut record, &issue_work_item);
    add_created(&mut record, &issue_work_item);
    add_author(&mut record, &issue_work_item);
    add_issue(&mut record, &issue_work_item);
    add_project(&mut record, &issue_work_item);

    handler.handle_record(&mut record)?;

    Ok(())
}

fn add_project(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref issue) = issue_work_item.issue {
        if let Some(ref project) = issue.project {
            record.fields_as_mut().push(Field::new_value(
                "project",
                Value::String(project.id.to_string()),
            ));

            if let Some(ref name) = project.name {
                record.fields_as_mut().push(Field::new_value(
                    "project.name",
                    Value::String(name.to_string()),
                ));
            }
        }
    }
}

fn add_issue(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref issue) = issue_work_item.issue {
        if let Some(ref id) = issue.id_readable {
            record
                .fields_as_mut()
                .push(Field::new_value("issue", Value::String(id.to_string())));
        }

        if let Some(ref summary) = issue.summary {
            record.fields_as_mut().push(Field::new_value(
                "issue.summary",
                Value::String(summary.to_string()),
            ));
        }
    }
}

fn add_created(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref created) = issue_work_item.created {
        record
            .fields_as_mut()
            .push(Field::new_value("created", Value::I64(*created)));
    }
}

fn add_author(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref author) = issue_work_item.author {
        if let Some(ref email) = author.email {
            record
                .fields_as_mut()
                .push(Field::new_value("email", Value::String(email.to_string())));
        }
    }
}

fn add_duration(record: &mut Record, issue_work_item: &IssueWorkItem) {
    if let Some(ref duration) = issue_work_item.duration {
        if let Some(minutes) = duration.minutes {
            record
                .fields_as_mut()
                .push(Field::new_value("minutes", Value::I32(minutes)));
        }
    }
}

#[cfg(test)]
mod tests;
