use model::{field::Field, record::Record};

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
    fields.push(Field::new_value(
        "created".to_string(),
        model::value::Value::I64(work_item.created),
    ));
    fields.push(Field::new_value(
        "date".to_string(),
        model::value::Value::I64(work_item.date),
    ));
    fields.push(Field::new_value(
        "work_item_id".to_string(),
        model::value::Value::String(work_item.id),
    ));
    fields.push(Field::new_value(
        "user_id".to_string(),
        model::value::Value::String(work_item.author.id),
    ));
    fields.push(Field::new_value(
        "user_name".to_string(),
        model::value::Value::String(work_item.author.name),
    ));
    fields.push(Field::new_value(
        "duration_minutes".to_string(),
        model::value::Value::I32(work_item.duration.minutes),
    ));
    callback(&record);
}
