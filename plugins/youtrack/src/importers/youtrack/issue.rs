use serde::{Deserialize, Serialize};

use super::common::User;

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
    pub summary: Option<String>,
    #[serde(rename = "commentsCount")]
    pub comments_count: Option<i32>,
    pub description: Option<String>,
    pub created: Option<i64>,
    #[serde(rename = "draftOwner")]
    pub draft_owner: Option<User>,
    #[serde(rename = "isDraft")]
    pub is_draft: Option<bool>,
}
