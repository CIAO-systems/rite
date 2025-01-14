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
mod tests {
    use crate::importers::youtrack::issue_work_item::IssueWorkItem;

    static TEST_DATA: &str = r#"{
        "$type": "IssueWorkItem",
        "author": {
            "$type": "User",
            "id": "1-2",
            "fullName": "Chuck Norris"
        },
        "date": 1723075200000,
        "duration": {
            "$type": "DurationValue",
            "id": "240",
            "minutes": 240,
            "presentation": "4h"
        },
        "id": "168-70",
        "type": null
    }"#;

    #[test]
    fn test_date() -> Result<(), serde_json::Error> {
        let work_item: IssueWorkItem = serde_json::from_str(TEST_DATA)?;
        let date = work_item.date();
        assert!(date.is_some());
        if let Some(date) = date {
            assert_eq!("2024-08-08", date.format("%Y-%m-%d").to_string());
        }
        Ok(())
    }

    #[test]
    fn test() -> Result<(), serde_json::Error> {
        let work_item: IssueWorkItem = serde_json::from_str(TEST_DATA)?;
        println!("{:#?}", work_item);

        assert_eq!("IssueWorkItem", work_item.object_type);
        assert_eq!(Some(1723075200000), work_item.date);
        assert_eq!("168-70", work_item.id);
        assert_eq!(None, work_item.work_item_type);

        assert!(work_item.author.is_some());
        if let Some(author) = work_item.author {
            assert_eq!("User", author.object_type);
            assert_eq!("1-2", author.id);
            assert_eq!(Some("Chuck Norris".to_string()), author.full_name);
        }

        assert!(work_item.duration.is_some());
        if let Some(duration) = work_item.duration {
            assert_eq!("DurationValue", duration.object_type);
            assert_eq!(Some("240".to_string()), duration.id);
            assert_eq!(Some(240), duration.minutes);
            assert_eq!(Some("4h".to_string()), duration.presentation);
        }

        Ok(())
    }
}
