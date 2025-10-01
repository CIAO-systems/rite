use ciao_rs::ciao::{
    common::{TimeRange, Timestamp},
    ClientManager,
};
use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};
use model::{xml::config::get_config_value, BoxedError};

pub const CFG_URL: &str = "url";
pub const CFG_API_KEY: &str = "api-key";

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
                Err(format!("{CFG_API_KEY} not configured").into())
            }
        } else {
            Err(format!("{CFG_URL} not configured").into())
        }
    }
}

/// Read a [TimeRange] with prefix `key` from the configuration `config`.
///
/// A time range configuration has two keys, the `startTime` and the `endTime`
/// If any of the parts are missing or can't be parsed, the result will be [None]
/// The format of the time values is a ISO 8601 date/time string in UTC
///
/// # Arguments
/// * `config`: A reference to a Option<Configuration>
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
mod tests;
