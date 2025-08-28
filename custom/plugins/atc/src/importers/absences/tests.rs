use std::env;

use chrono::{DateTime, Datelike, Timelike, Utc};
use model::import::{handlers::CollectingRecordHandler, Importer};
use model::{xml::config::Configuration, BoxedError, Initializable};
use prost_types::Timestamp;

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
