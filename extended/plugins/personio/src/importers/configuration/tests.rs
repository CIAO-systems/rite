use model::{BoxedError, xml::config::Configuration};
use tokio::runtime::Runtime;

use crate::importers::configuration::{
    CFG_APP_ID, CFG_CLIENT_ID, CFG_OPTIONS_LIMIT, CFG_PARTNER_ID,
};

use super::GeneralConfiguration;

#[test]
fn test_new() -> Result<(), BoxedError> {
    let mut config = Configuration::new();
    config.insert_str(CFG_OPTIONS_LIMIT, "73");
    config.insert_str(CFG_APP_ID, "test-app-id");
    config.insert_str(CFG_PARTNER_ID, "test-partner-id");
    let gc = GeneralConfiguration::with_config_token_and_runtime(
        &config,
        "token".to_string(),
        Runtime::new()?,
    );

    assert!(gc.limit.is_some());
    assert_eq!(73, gc.limit.unwrap());

    assert_eq!("token", gc.token.unwrap());
    assert_eq!("test-app-id", gc.personio_headers.app_id.unwrap());
    assert_eq!("test-partner-id", gc.personio_headers.partner_id.unwrap());

    Ok(())
}

#[test]
fn test_load() -> Result<(), BoxedError> {
    let config = Configuration::new();
    let result = GeneralConfiguration::load(&config);
    assert!(result.is_err_and(|e| e.to_string().eq("client_id missing")));

    let mut config = Configuration::new();
    config.insert_str(CFG_CLIENT_ID, "a client id");
    let result = GeneralConfiguration::load(&config);
    assert!(result.is_err_and(|e| e.to_string().eq("client_secret missing")));
    Ok(())
}

#[test]
fn test_get_personnel_configuration() -> Result<(), BoxedError> {
    let config = Configuration::new();
    let gc = GeneralConfiguration::with_config_token_and_runtime(
        &config,
        "token".to_string(),
        Runtime::new()?,
    );

    let result = gc.get_personnel_configuration();
    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(config.bearer_access_token.is_some_and(|bat| bat == "token"));
    Ok(())
}

#[test]
fn test_get_personnel_configuration_err() -> Result<(), BoxedError> {
    let gc = GeneralConfiguration::new();

    let result = gc.get_personnel_configuration();
    assert!(result.is_err_and(|e| e.to_string() == "No valid token stored"));

    Ok(())
}
