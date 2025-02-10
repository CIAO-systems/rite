use ciao_rs::{
    ciao::{
        common::{TimeRange, Timestamp},
        interceptor::APIKeyClientInterceptor,
        ClientManager,
    },
    interceptors,
};
use model::{xml::config::get_config_value, BoxedError};

const CFG_URL: &str = "url";
const CFG_API_KEY: &str = "api-key";

const ERR_NO_URL: &str = "URL not configured";
const ERR_NO_API_KEY: &str = "API key not configured";

#[derive(Debug)]
pub struct ConnectionConfiguration {
    url: Option<String>,
    api_key: Option<String>,
}

impl ConnectionConfiguration {
    pub fn new() -> Self {
        ConnectionConfiguration {
            url: None,
            api_key: None,
        }
    }

    pub fn from(config: &model::xml::config::Configuration) -> Self {
        let mut result = ConnectionConfiguration::new();
        if let Some(url) = config.get(CFG_URL) {
            result.url = Some(String::from(url));
        }
        if let Some(api_key) = config.get(CFG_API_KEY) {
            result.api_key = Some(String::from(api_key));
        }
        result
    }

    pub async fn connect(&self) -> Result<ClientManager, BoxedError> {
        if let Some(ref url) = self.url {
            if let Some(ref api_key) = self.api_key {
                Ok(ClientManager::new(
                    url,
                    interceptors!(APIKeyClientInterceptor::new(api_key.to_string())),
                )
                .await?)
            } else {
                Err(ERR_NO_API_KEY.into())
            }
        } else {
            Err(ERR_NO_URL.into())
        }
    }
}

/// Read a [TimeRange] with prefix `key` from the confiuration `config`.
/// 
/// A time range configuration has two keys, the `startTime` and the `endTime`
/// If any of the parts are missing or can't be parsed, the result will be [None]
/// The format of the time alues is a ISO 8601 date/time string in UTC
/// 
/// # Arguments
/// * `config`: A reference to a [Configuration]
/// * `key`: The prefix  for the two range keys: `<key>.startTime` and `<key>.endTime`
/// 
pub fn get_config_time_range(
    config: &Option<model::xml::config::Configuration>,
    key: &str,
) -> Option<ciao_rs::ciao::common::TimeRange> {
    if let Some(start_time) = get_config_value::<String>(config, &format!("{key}.startTime")) {
        if let Ok(start_time) = Timestamp::parse_from(&start_time) {
            if let Some(end_time) = get_config_value::<String>(config, &format!("{key}.endTime")) {
                if let Ok(end_time) = Timestamp::parse_from(&end_time) {
                    return Some(TimeRange {
                        start_time: Some(start_time),
                        end_time: Some(end_time),
                    });
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
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
}
