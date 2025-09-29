use std::env;

use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use model::import::handlers::ClosureRecordHandler;
use model::import::{handlers::CollectingRecordHandler, Importer};
use model::record::Record;
use model::value::Value;
use model::{xml::config::Configuration, BoxedError, Initializable};
use prost_types::Timestamp;

use crate::connection::clients::manager::tests::mocks::get_mock_client_manager;
use crate::importers::absences::{add_timestamp_field, call_get_absences, get_start_and_end_date};
use crate::importers::common::protobuf_to_date;
use crate::{
    com::atoss::atc::protobuf::AbsencesRequest,
    importers::absences::{
        utc_to_atc, Absences, CFG_FILTER_ACCOUNTS, CFG_FILTER_EMPLOYEES, CFG_FILTER_PERIOD,
    },
};

#[test]
fn test_absences() {
    let employee_ids = vec!["01".to_string(), "02".to_string(), "03".to_string()];
    let request = AbsencesRequest {
        employee_ids: employee_ids.clone(),
        start_date: None,
        end_date: None,
        account_ids: vec![1, 2, 3],
        plan_version: -1,
        options: None,
    };

    // Asserting specific field's value
    assert_eq!(request.employee_ids.len(), 3);
    assert_eq!(employee_ids, request.employee_ids);
    assert!(request.start_date.is_none());
    assert_eq!(request.plan_version, -1);
}

#[test]
#[ignore = "for manual testing"]
fn test_import() -> Result<(), BoxedError> {
    dotenv::from_filename("../../../.env").ok();

    // Arrange
    let mut absences = Absences::new();
    let mut config = Configuration::new();
    config.insert_str("url", env::var("ATC_URL").unwrap().as_str());
    config.insert_str("auth-token", "test-token");
    config.insert_str("user", env::var("ATC_USER").unwrap().as_str());
    config.insert_str("password", env::var("ATC_PASSWORD").unwrap().as_str());

    config.insert_str(CFG_FILTER_EMPLOYEES, "");
    config.insert_str(CFG_FILTER_ACCOUNTS, "");
    config.insert_str(CFG_FILTER_PERIOD, "2025-01-01:2025-12-31");

    absences.init(Some(config))?;

    // Act
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);
    absences.read(&mut handler)?;

    // Assert
    assert!(records.len() > 0);
    Ok(())
}

#[test]
fn test_utc_to_atc_cet() {
    use chrono::TimeZone;
    // Example timestamp during CET (UTC+1)
    let utc_datetime = Utc.with_ymd_and_hms(2025, 3, 1, 23, 0, 0).unwrap();
    let timestamp = Timestamp {
        seconds: utc_datetime.timestamp(),
        nanos: utc_datetime.timestamp_subsec_nanos() as i32,
    };

    let result = utc_to_atc(timestamp).unwrap();
    let result_datetime =
        DateTime::<Utc>::from_timestamp(result.seconds, result.nanos as u32).unwrap();

    // Assert the converted time is as expected
    assert_eq!(result_datetime.hour(), 0);
    assert_eq!(result_datetime.day(), 2);
}

#[test]
fn test_utc_to_atc_cest() {
    use chrono::TimeZone;
    // Example timestamp during CET (UTC+1)
    let utc_datetime = Utc.with_ymd_and_hms(2025, 4, 2, 22, 0, 0).unwrap();
    let timestamp = Timestamp {
        seconds: utc_datetime.timestamp(),
        nanos: utc_datetime.timestamp_subsec_nanos() as i32,
    };

    let result = utc_to_atc(timestamp).unwrap();
    let result_datetime =
        DateTime::<Utc>::from_timestamp(result.seconds, result.nanos as u32).unwrap();

    // Assert the converted time is as expected
    assert_eq!(result_datetime.hour(), 0);
    assert_eq!(result_datetime.day(), 3);
}

#[test]
fn test_importer() {
    let mut absences = Absences::new();
    let mut config = Configuration::new();
    config.insert_str("key", "value");
    let result = absences.init(Some(config));
    assert!(result.is_ok());
    assert!(absences
        .config
        .as_ref()
        .is_some_and(|c| c.get("key").is_some_and(|v| v.to_string() == "value")));

    let mut handler = ClosureRecordHandler::new(|r| println!("{:?}", r));
    let result = absences.read(&mut handler);
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
}

#[tokio::test]
async fn test_importer_with_mock_server() {
    let cm = get_mock_client_manager(50056).await;
    assert!(cm.is_ok());

    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_ACCOUNTS, "0");

    let mut expected_record_found = false;
    let mut handler = ClosureRecordHandler::new(|r| {
        expected_record_found = true; // mock returns one moocked absence
        println!("{:?}", r);
    });

    let result = call_get_absences(&config, cm.unwrap().absences_client, &mut handler).await;
    println!("{:?}", result);

    assert!(expected_record_found);
}

#[test]
fn test_add_timestamp_field() {
    let mut record = Record::new();
    let value = Some(Timestamp {
        seconds: 0,
        nanos: 0,
    });
    add_timestamp_field(record.fields_as_mut(), "field_name", value);
    println!("{:?}", record);
    assert_eq!(
        record.field_by_name("field_name").unwrap().value(),
        Value::String("1970-01-01T01:00:00Z".into())
    )
}

#[test]
fn test_utc_to_atc_none() {
    let value = Timestamp {
        seconds: 2_146_764_484 * 86_400, // too big
        nanos: 0,
    };
    let result = utc_to_atc(value);
    assert!(result.is_err_and(|e| e.to_string() == "utc_to_atc failed"));
}

#[test]
fn test_get_start_and_end_date() {
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_PERIOD, "2025-01-01"); // Only start
    let result = get_start_and_end_date(&config);
    println!("{:?}", result);
    assert!(result.is_ok());
    let (start, end) = result.unwrap();
    assert!(start.is_some());
    assert!(end.is_some());
    let (start, end) = (start.unwrap(), end.unwrap());
    assert_eq!(start.seconds, 1735689600);
    assert_eq!(end.seconds, 1767225600);

    let today = Local::now().date_naive();
    config.insert_str(CFG_FILTER_PERIOD, ":2025-01-01"); // Only end
    let result = get_start_and_end_date(&config);
    println!("{:?}", result);
    assert!(result.is_ok());
    let (start, end) = result.unwrap();
    assert!(start.is_some());
    assert!(end.is_some());
    let (start, end) = (start.unwrap(), end.unwrap());
    assert!(protobuf_to_date(Some(start)).unwrap() >= today); // Start is today, so it should be same or greater 
    assert_eq!(end.seconds, 1735689600);

    config.insert_str(CFG_FILTER_PERIOD, "2025-01-01:2025-01-01"); // start and end
    let result = get_start_and_end_date(&config);
    println!("{:?}", result);
    assert!(result.is_ok());
    let (start, end) = result.unwrap();
    assert!(start.is_some());
    assert!(end.is_some());
    let (start, end) = (start.unwrap(), end.unwrap());
    assert_eq!(start.seconds, 1735689600);
    assert_eq!(end.seconds, 1735689600);
}
