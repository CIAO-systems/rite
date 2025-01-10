use model::record::Record;
use serde::{Deserialize, Serialize};

use super::{common, factory};

/// rust struct for youTrack Issue JSON
/// See https://www.jetbrains.com/help/youtrack/devportal/resource-api-issues.html
#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    #[serde(rename = "idReadable")]
    pub id_readable: Option<String>,
    #[serde(rename = "commentsCount")]
    pub comments_count: Option<i32>,
    pub description: Option<String>,
    pub created: Option<i64>,
    #[serde(rename = "draftOwner")]
    pub draft_owner: Option<common::user::User>,
    #[serde(rename = "isDraft")]
    pub is_draft: Option<bool>,
    #[serde(rename = "numberInProject")]
    pub number_in_project: Option<i32>,
    pub project: Option<common::project::Project>,
    /// The timestamp in milliseconds indicating the moment when the issue
    /// was assigned a state that is considered to be resolved
    pub resolved: Option<i64>,
    pub summary: Option<String>,
    /// The timestamp in milliseconds indicating the last update of the issue
    pub updated: Option<i64>,
    pub updater: Option<common::user::User>,
    pub votes: Option<i16>,
    #[serde(rename = "wikifiedDescription")]
    pub wikified_description: Option<String>,
}

impl From<Issue> for Record {
    fn from(value: Issue) -> Self {
        factory::serialize_to_record(value)
    }
}
