use model::{field::Field, record::Record};

use super::{config::RiteYoutrackImport, rest::make_request, youtrack::issue::Issue};

mod work_item;

/// Handler for the path "issues"
///
/// See documentation: https://www.jetbrains.com/help/youtrack/devportal/resource-api-issues.html
pub(crate) fn handle_issues_path(
    callback: import::RecordCallback,
    xml_config: &RiteYoutrackImport,
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(ref sub_resource) = xml_config.dataset.sub_resource {
        match sub_resource.as_str() {
            "timeTracking/workItems" => {
                make_request(
                    callback,
                    &xml_config,
                    &base_url,
                    &token,
                    work_item::handle_response_time_tracking_workitem,
                )?;
            }
            _ => return Err(format!("Unknown sub-resource '{}'", sub_resource).into()),
        }
    } else {
        //
        make_request(callback, &xml_config, &base_url, &token, handle_issues)?;
    }

    Ok(())
}

/// Handle the issues dataset
///
/// See: https://www.jetbrains.com/help/youtrack/devportal/resource-api-issues.html
#[allow(unused_variables)]
fn handle_issues(
    callback: import::RecordCallback,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    match response.json::<serde_json::Value>() {
        Ok(result) => {
            if let Some(array) = result.as_array() {
                for element in array {
                    match serde_json::from_value::<Issue>(element.clone()) {
                        Ok(issue) => {
                            handle_issue(callback, issue);
                        }
                        Err(e) => return Err(e.into()),
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

/// Processes an Issue
/// See https://www.jetbrains.com/help/youtrack/devportal/resource-api-issues.html
fn handle_issue(callback: import::RecordCallback, issue: Issue) {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    fields.push(Field::new_value(
        "id".to_string(),
        model::value::Value::String(issue.id),
    ));
    if let Some(summary) = issue.summary {
        fields.push(Field::new_value(
            "summary".to_string(),
            model::value::Value::String(summary),
        ));
    }

    callback(&record);
}
