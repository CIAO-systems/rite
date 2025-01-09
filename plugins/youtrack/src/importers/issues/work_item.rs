use model::{
    field::{add_field, add_optional_field},
    record::Record,
};

use crate::importers::{config::RiteYoutrackImport, youtrack::work_item::IssueWorkItem};

/// Processs all the IssueWorkItems
///
/// See https://www.jetbrains.com/help/youtrack/devportal/resource-api-issues-issueID-timeTracking.html
/// and https://www.jetbrains.com/help/youtrack/devportal/api-entity-IssueWorkItem.html
pub(crate) fn handle_response_time_tracking_workitem(
    _config: &RiteYoutrackImport,
    callback: import::RecordCallback,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    match response.json::<serde_json::Value>() {
        Ok(result) => {
            if let Some(array) = result.as_array() {
                for element in array {
                    if let Ok(work_item) = serde_json::from_value::<IssueWorkItem>(element.clone())
                    {
                        handle_work_item(callback, work_item);
                    }
                }
            } else {
                return Err("Response is not a JSON Array".into());
            }
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

/// Processes an IssueWorkItem
/// See https://www.jetbrains.com/help/youtrack/devportal/api-entity-IssueWorkItem.html
fn handle_work_item(callback: import::RecordCallback, work_item: IssueWorkItem) {
    let mut record = Record::new();
    let fields = record.fields_as_mut();

    add_optional_field(fields, "created", work_item.created);
    add_optional_field(fields, "date", work_item.date);
    add_field(fields, "work_item_id", work_item.id.into());
    if let Some(author) = work_item.author {
        add_field(fields, "user.id", author.id.into());
        add_optional_field(fields, "user.name", author.full_name);
    }
    if let Some(duration) = work_item.duration {
        add_field(fields, "duration_minutes", duration.minutes.into());
    }

    callback(&record);
}
