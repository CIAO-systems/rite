use model::record::Record;

use crate::importers::youtrack::{
    common::{project::Project, user::User},
    issue::Issue,
    issue_work_item::IssueWorkItem,
};

#[derive(Debug)]
pub enum YouTrackObject {
    Issue(Issue),
    IssueWorkItem(IssueWorkItem),
    User(User),
    Project(Project),
    // DurationValue(super::common::duration::DurationValue),
    None,
}

impl YouTrackObject {
    /// Create a rust object from a JSON Value, based on the YouTrack $type
    ///
    pub fn from_type(element: &serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(object) = element.as_object() {
            if let Some(object_type) = &object.get("$type") {
                if let Some(object_type) = object_type.as_str() {
                    return match object_type {
                        "Issue" => Ok(YouTrackObject::Issue(create_object(element)?)),
                        "IssueWorkItem" => {
                            Ok(YouTrackObject::IssueWorkItem(create_object(element)?))
                        }
                        "User" => Ok(YouTrackObject::User(create_object(element)?)),
                        "Project" => Ok(YouTrackObject::Project(create_object(element)?)),
                        _ => Ok(YouTrackObject::None),
                    };
                }
            }
        }
        Ok(YouTrackObject::None)
    }
}

/// Creates a rust object from a JSON Value
///
/// # Arguments
/// * `element`: The JSON Value to deserialize
///
fn create_object<T: serde::de::DeserializeOwned>(
    element: &serde_json::Value,
) -> Result<T, Box<dyn std::error::Error>> {
    match serde_json::from_value::<T>(element.clone()) {
        Ok(object) => Ok(object),
        Err(e) => {
            log::debug!("create_object: {:?}", element);
            Err(e.into())
        }
    }
}

/// Creates a record via rust objects from the JSON value and call the callback
///
fn derserialize_and_add(
    callback: &mut dyn FnMut(&Record),
    element: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let object = YouTrackObject::from_type(element)?;
    let record = match object {
        YouTrackObject::Issue(issue) => {
            let record: Record = issue.into();
            Some(record)
        }
        YouTrackObject::User(user) => {
            let record: Record = user.into();
            Some(record)
        }
        YouTrackObject::IssueWorkItem(issue_work_item) => {
            let record: Record = issue_work_item.into();
            Some(record)
        }
        YouTrackObject::Project(project) => {
            let record: Record = project.into();
            Some(record)
        }

        // TODO implement
        // YouTrackObject::DurationValue(duration_value) => todo!(),
        // YouTrackObject::Project(project) => todo!(),
        _ => None, // ignore,
    };
    Ok(if let Some(record) = record {
        callback(&record);
    })
}

#[cfg(test)]
mod tests;
