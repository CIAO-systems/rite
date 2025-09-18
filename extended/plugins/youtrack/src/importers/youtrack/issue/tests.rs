use model::{record::Record, value::Value};

use crate::importers::youtrack::issue::Issue;

fn minimal_issue() -> Issue {
    let json: &str = r#"{
        "$type": "Issue",
        "id": "73"
    }"#;
    serde_json::from_str(json).unwrap()
}

#[test]
fn test_record_from_issu() {
    let issue = minimal_issue();
    let record: Record = issue.into();
    assert_eq!(record.field_by_name("id").unwrap().value(), Value::String("73".into()));
}
