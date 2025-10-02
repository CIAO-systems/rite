use chrono::DateTime;
use model::record::Record;
use serde::{Deserialize, Serialize};

use super::{common, factory, issue::Issue};

#[derive(Debug, Deserialize, Serialize)]
pub struct IssueWorkItem {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    pub author: Option<common::user::User>,
    pub creator: Option<common::user::User>,
    pub created: Option<i64>,
    pub updated: Option<i64>,
    pub date: Option<i64>,
    pub duration: Option<common::duration::DurationValue>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub work_item_type: Option<WorkItemType>,
    pub text: Option<String>,
    #[serde(rename = "textPreview")]
    pub text_preview: Option<String>,
    pub issue: Option<Issue>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct WorkItemType {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "autoAttached")]
    pub auto_attached: Option<bool>,
}

impl IssueWorkItem {
    #[allow(dead_code)]
    pub fn date(&self) -> Option<chrono::NaiveDate> {
        if let Some(date) = self.date {
            if let Some(date_time) = DateTime::from_timestamp(date / 1000, 0) {
                return Some(date_time.date_naive());
            }
        }
        None
    }
}

impl From<IssueWorkItem> for Record {
    fn from(value: IssueWorkItem) -> Self {
        log::debug!("From<IssueWorkItem> {:#?}", value);
        factory::serialize_to_record(value)
    }
}

#[cfg(test)]
mod tests;
