use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IssueWorkItem {
    #[serde(rename = "$type")]
    #[serde(default)]
    object_type: String,
    pub author: User,
    pub created: i64,
    pub date: i64,
    pub duration: DurationValue,
    pub id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    work_item_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "$type")]
    #[serde(default)]
    object_type: String,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DurationValue {
    #[serde(rename = "$type")]
    #[serde(default)]
    object_type: String,
    pub id: String,
    pub minutes: i32,
    presentation: String,
}

impl IssueWorkItem {
    #[allow(dead_code)]
    pub fn date(&self) -> Option<chrono::NaiveDate> {
        if let Some(date_time) = DateTime::from_timestamp(self.date / 1000, 0) {
            Some(date_time.date_naive())
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::importers::youtrack::IssueWorkItem;

    static TEST_DATA: &str = r#"{
        "$type": "IssueWorkItem",
        "author": {
            "$type": "User",
            "id": "1-2",
            "name": "Chuck Norris"
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

        assert_eq!("IssueWorkItem", work_item.object_type);
        assert_eq!(1723075200000, work_item.date);
        assert_eq!("168-70", work_item.id);
        assert_eq!(None, work_item.work_item_type);

        assert_eq!("User", work_item.author.object_type);
        assert_eq!("1-2", work_item.author.id);
        assert_eq!("Chuck Norris", work_item.author.name);

        assert_eq!("DurationValue", work_item.duration.object_type);
        assert_eq!("240", work_item.duration.id);
        assert_eq!(240, work_item.duration.minutes);
        assert_eq!("4h", work_item.duration.presentation);

        println!("{:#?}", work_item);

        Ok(())
    }
}
