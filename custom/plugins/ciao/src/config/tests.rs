use model::xml::config::Configuration;

use super::*;

#[test]
fn test_valid_time_range() {
    let mut config = model::xml::config::Configuration::new();
    config.insert(
        "my_range.startTime".to_string(),
        "2024-07-20T10:00:00Z".to_string(),
    );
    config.insert(
        "my_range.endTime".to_string(),
        "2024-07-21T12:00:00Z".to_string(),
    );
    let config = Some(config);

    let time_range = get_config_time_range(&config, "my_range");
    assert!(time_range.is_some());
    let time_range = time_range.unwrap();
    assert_eq!(
        time_range.start_time,
        Some(Timestamp::parse_from("2024-07-20T10:00:00Z").unwrap())
    );
    assert_eq!(
        time_range.end_time,
        Some(Timestamp::parse_from("2024-07-21T12:00:00Z").unwrap())
    );
}

#[test]
fn test_missing_start_time() {
    let mut config = model::xml::config::Configuration::new();
    config.insert(
        "my_range.endTime".to_string(),
        "2024-07-21T12:00:00Z".to_string(),
    );
    let config = Some(config);

    let time_range = get_config_time_range(&config, "my_range");
    assert!(time_range.is_none());
}

#[test]
fn test_missing_end_time() {
    let mut config = model::xml::config::Configuration::new();
    config.insert(
        "my_range.startTime".to_string(),
        "2024-07-20T10:00:00Z".to_string(),
    );
    let config = Some(config);
    let time_range = get_config_time_range(&config, "my_range");
    assert!(time_range.is_none());
}

#[test]
fn test_invalid_start_time_format() {
    let mut config = model::xml::config::Configuration::new();
    config.insert("my_range.startTime".to_string(), "invalid_time".to_string());
    config.insert(
        "my_range.endTime".to_string(),
        "2024-07-21T12:00:00Z".to_string(),
    );
    let config = Some(config);

    let time_range = get_config_time_range(&config, "my_range");
    assert!(time_range.is_none());
}

#[test]
fn test_invalid_end_time_format() {
    let mut config = model::xml::config::Configuration::new();
    config.insert(
        "my_range.startTime".to_string(),
        "2024-07-20T10:00:00Z".to_string(),
    );
    config.insert("my_range.endTime".to_string(), "invalid_time".to_string());
    let config = Some(config);

    let time_range = get_config_time_range(&config, "my_range");
    assert!(time_range.is_none());
}

#[test]
fn test_empty_config() {
    let config = None;
    let time_range = get_config_time_range(&config, "my_range");
    assert!(time_range.is_none());
}

#[test]
fn test_config_with_wrong_key() {
    let mut config = model::xml::config::Configuration::new();
    config.insert(
        "other_key.startTime".to_string(),
        "2024-07-20T10:00:00Z".to_string(),
    );
    config.insert(
        "other_key.endTime".to_string(),
        "2024-07-21T12:00:00Z".to_string(),
    );
    let config = Some(config);

    let time_range = get_config_time_range(&config, "my_range");
    assert!(time_range.is_none());
}

#[test]
fn test_connenction_configuration_new() {
    let config = ConnectionConfiguration::new();
    assert!(config.url.is_none());
    assert!(config.api_key.is_none());
}

#[test]
fn test_connenction_configuration_from_empty() {
    let config = Configuration::new();
    let config = ConnectionConfiguration::from(&config);
    assert!(config.url.is_none());
    assert!(config.api_key.is_none());
}

#[test]
fn test_connenction_configuration_from() {
    let mut config = Configuration::new();
    config.insert_str(CFG_URL, "url");
    config.insert_str(CFG_API_KEY, "api-key");
    let config = ConnectionConfiguration::from(&config);
    assert!(config.url.is_some_and(|s| s == "url"));
    assert!(config.api_key.is_some_and(|s| s == "api-key"));
}

#[tokio::test]
async fn test_connenction_configuration_connect_no_api_key() {
    let mut config = Configuration::new();
    config.insert_str(CFG_URL, "url");
    let config = ConnectionConfiguration::from(&config);
    assert!(config.url.clone().is_some_and(|s| s == "url"));
    let result = config.connect().await;
    assert!(result.is_err_and(|e| e.to_string() == "api-key not configured"));
}

#[tokio::test]
async fn test_connenction_configuration_connect_no_url() {
    let mut config = Configuration::new();
    config.insert_str(CFG_API_KEY, "api-key");
    let config = ConnectionConfiguration::from(&config);
    assert!(config.api_key.clone().is_some_and(|s| s == "api-key"));
    let result = config.connect().await;
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
}
