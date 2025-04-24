use grpc_utils_rs::interceptors;
use model::BoxedError;

use super::{clients::manager::ClientManager, interceptor::ATCClientInterceptor};

const CFG_URL: &str = "url";
const CFG_AUTH_TOKEN: &str = "auth-token";
const CFG_USER: &str = "user";
const CFG_PASSWORD: &str = "password";

pub const CFG_FILTER_TABLE: &str = "filter.table";

#[derive(Debug)]
pub struct ConnectionConfiguration {
    url: Option<String>,
    auth_token: Option<String>,
    user: Option<String>,
    password: Option<String>,
}

impl ConnectionConfiguration {
    pub fn new() -> Self {
        ConnectionConfiguration {
            url: None,
            auth_token: None,
            user: None,
            password: None,
        }
    }

    pub fn from(config: &model::xml::config::Configuration) -> Self {
        let mut result = ConnectionConfiguration::new();
        if let Some(url) = config.get(CFG_URL) {
            result.url = Some(String::from(url));
        }
        if let Some(auth_token) = config.get(CFG_AUTH_TOKEN) {
            result.auth_token = Some(String::from(auth_token));
        }
        if let Some(user) = config.get(CFG_USER) {
            result.user = Some(String::from(user));
        }
        if let Some(password) = config.get(CFG_PASSWORD) {
            result.password = Some(String::from(password));
        }
        result
    }

    pub async fn connect(&self) -> Result<ClientManager, BoxedError> {
        if let Some(ref url) = self.url {
            if let Some(ref auth_token) = self.auth_token {
                if let Some(ref user) = self.user {
                    if let Some(ref password) = self.password {
                        Ok(ClientManager::new(
                            url,
                            interceptors!(ATCClientInterceptor::new(auth_token, user, password)),
                        )
                        .await?)
                    } else {
                        Err(format!("{CFG_PASSWORD} not configured").into())
                    }
                } else {
                    Err(format!("{CFG_USER} not configured").into())
                }
            } else {
                Err(format!("{CFG_AUTH_TOKEN} not configured").into())
            }
        } else {
            Err(format!("{CFG_URL} not configured").into())
        }
    }
}
