use std::env;

use import::{handlers::CollectingRecordHandler, Importer};
use model::{xml::config::Configuration, BoxedError, Initializable};

use crate::{
    com::atoss::atc::protobuf::AbsencesRequest,
    importers::absences::{Absences, CFG_FILTER_ACCOUNTS, CFG_FILTER_EMPLOYEES, CFG_FILTER_PERIOD},
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
