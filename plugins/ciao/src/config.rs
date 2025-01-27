use ciao_rs::{ciao::{interceptor::APIKeyClientInterceptor, ClientManager}, interceptors};
use model::{to_boxed_error, BoxedError};

const CFG_URL: &str = "url";
const CFG_API_KEY: &str = "api-key";

const ERR_NO_URL: &str = "URL not configured";
const ERR_NO_API_KEY: &str = "API key not configured";

pub struct CiaoConnection {
    url: Option<String>,
    api_key: Option<String>,
}

impl CiaoConnection {
    pub fn new() -> Self {
        CiaoConnection {
            url: None,
            api_key: None,
        }
    }

    pub fn from(config: &model::xml::config::Configuration) -> Self {
        let mut result = CiaoConnection::new();
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
                .await
                .map_err(to_boxed_error)?)
            } else {
                Err(to_boxed_error(ERR_NO_API_KEY))
            }
        } else {
            Err(to_boxed_error(ERR_NO_URL))
        }
    }
}