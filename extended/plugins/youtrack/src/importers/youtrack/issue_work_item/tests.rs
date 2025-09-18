use model::{record::Record, value::Value};

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

fn minimal_item() -> IssueWorkItem {
    let json: &str = r#"{
        "$type": "IssueWorkItem",
        "id": "168-70"
    }"#;
    serde_json::from_str(json).unwrap()
}

#[test]
fn test_date_none() {
    let work_item = minimal_item();
    let date = work_item.date();
    assert!(date.is_none());
}

#[test]
fn test_record_from_item() {
    let work_item = minimal_item();
    let record: Record = work_item.into();
    assert_eq!(record.field_by_name("id").unwrap().value(), Value::String("168-70".into()));
}
