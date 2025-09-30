use ciao_rs::ciao::{
    common::Timestamp,
    time_tracking::{clock_record::Identity, ClockRecord},
};
use model::{
    import::{handlers::ClosureRecordHandler, Importer}, value::Value, xml::config::Configuration, Initializable
};

use crate::importers::clock_entries::{handle_clock_entry, ClockEntries};

#[test]
fn test_init() {
    let mut importer = ClockEntries::new();
    let config = Configuration::new();
    let result = importer.init(Some(config));
    assert!(result.is_ok());
}

#[test]
fn test_read() {
    let mut importer = ClockEntries::new();
    let mut handler = ClosureRecordHandler::new(|_| {});
    let config = Configuration::new();
    let result = importer.init(Some(config));
    assert!(result.is_ok());
    let result = importer.read(&mut handler);
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
}

#[test]
fn test_handle_clock_entry_badge() {
    let badge = ClockRecord {
        timestamp: Some(Timestamp {
            time_utc: Some(prost_types::Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            time_zone: "Utc".into(),
        }),
        device_id: Some("device_id".into()),
        time_type_id: Some("time_type_id".into()),
        project_id: Some("project_id".into()),
        cost_center_id: Some("cost_center_id".into()),
        project_task_id: Some("project_task_id".into()),
        id: Some("id".into()),
        identity: Some(Identity::BadgeId("badge-id".into())),
    };
    let mut handler = ClosureRecordHandler::new(|r| {
        assert_eq!(r.field_by_name("id").unwrap().value(), "id".into());
        assert_eq!(
            r.field_by_name("identitiy.badgeId").unwrap().value(),
            "badge-id".into()
        );
        assert_eq!(r.field_by_name("deviceId").unwrap().value(), "device_id".into());
        assert_eq!(r.field_by_name("timeTypeId").unwrap().value(), "time_type_id".into());
        assert_eq!(r.field_by_name("projectId").unwrap().value(), "project_id".into());
        assert_eq!(r.field_by_name("projectTaskId").unwrap().value(), "project_task_id".into());
        assert_eq!(r.field_by_name("costCenterId").unwrap().value(), "cost_center_id".into());

        assert_eq!(r.field_by_name("timestamp.timeUtc").unwrap().value(), Value::I64(0));
        assert_eq!(r.field_by_name("timestamp.timeZone").unwrap().value(), "Utc".into());
    });
    let result = handle_clock_entry(&badge, &mut handler);
    assert!(result.is_ok());
}

#[test]
fn test_handle_clock_entry_user() {
    let badge = ClockRecord {
        timestamp: Some(Timestamp {
            time_utc: Some(prost_types::Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            time_zone: "Utc".into(),
        }),
        device_id: Some("device_id".into()),
        time_type_id: Some("time_type_id".into()),
        project_id: Some("project_id".into()),
        cost_center_id: Some("cost_center_id".into()),
        project_task_id: Some("project_task_id".into()),
        id: Some("id".into()),
        identity: Some(Identity::UserId("user-id".into())),
    };
    let mut handler = ClosureRecordHandler::new(|r| {
        assert_eq!(r.field_by_name("id").unwrap().value(), "id".into());
        assert_eq!(
            r.field_by_name("identitiy.userId").unwrap().value(),
            "user-id".into()
        );
        assert_eq!(r.field_by_name("deviceId").unwrap().value(), "device_id".into());
        assert_eq!(r.field_by_name("timeTypeId").unwrap().value(), "time_type_id".into());
        assert_eq!(r.field_by_name("projectId").unwrap().value(), "project_id".into());
        assert_eq!(r.field_by_name("projectTaskId").unwrap().value(), "project_task_id".into());
        assert_eq!(r.field_by_name("costCenterId").unwrap().value(), "cost_center_id".into());

        assert_eq!(r.field_by_name("timestamp.timeUtc").unwrap().value(), Value::I64(0));
        assert_eq!(r.field_by_name("timestamp.timeZone").unwrap().value(), "Utc".into());
    });
    let result = handle_clock_entry(&badge, &mut handler);
    assert!(result.is_ok());
}
