use model::{
    field::{add_field, add_optional_field},
    record::Record,
};

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
    config: &RiteYoutrackImport,
    callback: import::RecordCallback,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    match response.json::<serde_json::Value>() {
        Ok(result) => {
            if let Some(array) = result.as_array() {
                for element in array {
                    match serde_json::from_value::<Issue>(element.clone()) {
                        Ok(issue) => {
                            handle_issue(config, callback, issue);
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
fn handle_issue(config: &RiteYoutrackImport, callback: import::RecordCallback, issue: Issue) {
    let mut record = Record::new();
    let fields = record.fields_as_mut();

    config
        .dataset
        .fields
        .contains("id")
        .then(|| add_field(fields, "id", issue.id.into()));
    config
        .dataset
        .fields
        .contains("idReadable")
        .then(|| add_optional_field(fields, "idReadable", issue.id_readable));
    config
        .dataset
        .fields
        .contains("commentsCount")
        .then(|| add_optional_field(fields, "commentsCount", issue.comments_count));
    config
        .dataset
        .fields
        .contains("description")
        .then(|| add_optional_field(fields, "description", issue.description));
    config.dataset.fields.contains("draftOwner").then(|| {
        if let Some(draft_owner) = issue.draft_owner {
            add_field(fields, "draftOwner.id", draft_owner.id.into());
            add_optional_field(fields, "draftOwner.name", draft_owner.full_name)
        }
    });
    config
        .dataset
        .fields
        .contains("isDraft")
        .then(|| add_optional_field(fields, "isDraft", issue.is_draft));
    config
        .dataset
        .fields
        .contains("numberInProject")
        .then(|| add_optional_field(fields, "numberInProject", issue.number_in_project));
    config.dataset.fields.contains("project").then(|| {
        if let Some(project) = issue.project {
            add_field(fields, "project.id", project.id.into());
            add_optional_field(fields, "project.name", project.name)
        }
    });
    config
        .dataset
        .fields
        .contains("resolved")
        .then(|| add_optional_field(fields, "resolved", issue.resolved));
    config
        .dataset
        .fields
        .contains("summary")
        .then(|| add_optional_field(fields, "summary", issue.summary));
    config
        .dataset
        .fields
        .contains("updated")
        .then(|| add_optional_field(fields, "updated", issue.updated));
    config.dataset.fields.contains("updater").then(|| {
        if let Some(updater) = issue.updater {
            add_field(fields, "updater.id", updater.id.into());
            add_optional_field(fields, "updater.name", updater.full_name)
        }
    });
    config
        .dataset
        .fields
        .contains("votes")
        .then(|| add_optional_field(fields, "votes", issue.votes));
    config
        .dataset
        .fields
        .contains("wikifiedDescription")
        .then(|| add_optional_field(fields, "wikifiedDescription", issue.wikified_description));

    callback(&record);
}
