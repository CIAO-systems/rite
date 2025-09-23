use grpc_utils_rs::interceptors;
use model::BoxedError;

use super::{clients::manager::ClientManager, interceptor::ATCClientInterceptor};

pub const CFG_URL: &str = "url";
pub const CFG_AUTH_TOKEN: &str = "auth-token";
pub const CFG_USER: &str = "user";
pub const CFG_PASSWORD: &str = "password";

pub const CFG_FILTER_TABLE: &str = "filter.table";

#[derive(Debug)]
pub struct ConnectionConfiguration {
    pub(crate) url: Option<String>,
    pub(crate) auth_token: Option<String>,
    pub(crate) user: Option<String>,
    pub(crate) password: Option<String>,
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

#[cfg(test)]
mod tests {
    use model::xml::config::Configuration;

    use crate::connection::config::{ConnectionConfiguration, CFG_AUTH_TOKEN, CFG_PASSWORD, CFG_URL, CFG_USER};

    #[test]
    fn test_new() {
        let config = ConnectionConfiguration::new();
        assert!(config.url.is_none());
        assert!(config.auth_token.is_none());
        assert!(config.user.is_none());
        assert!(config.password.is_none());
    }

    #[test]
    fn test_from() {
        let mut config = Configuration::new();
        config.insert_str(CFG_URL, "url");
        config.insert_str(CFG_AUTH_TOKEN, "token");
        config.insert_str(CFG_USER, "user");
        config.insert_str(CFG_PASSWORD, "password");
        let config = ConnectionConfiguration::from(&config);

        assert!(config.url.is_some_and(|v|v == "url"));
        assert!(config.auth_token.is_some_and(|v|v == "token"));
        assert!(config.user.is_some_and(|v|v == "user"));
        assert!(config.password.is_some_and(|v|v == "password"));
    }

    #[tokio::test]
    async fn test_connext() {
        let mut config = Configuration::new();
        let sut = ConnectionConfiguration::from(&config);
        let result = sut.connect().await;
        assert!(result.is_err_and(|e|e.to_string()=="url not configured"));

        config.insert_str(CFG_URL, "url");
        let sut = ConnectionConfiguration::from(&config);
        let result = sut.connect().await;
        assert!(result.is_err_and(|e|e.to_string()=="auth-token not configured"));


        config.insert_str(CFG_AUTH_TOKEN, "token");
        let sut = ConnectionConfiguration::from(&config);
        let result = sut.connect().await;
        assert!(result.is_err_and(|e|e.to_string()=="user not configured"));

        config.insert_str(CFG_USER, "user");
        let sut = ConnectionConfiguration::from(&config);
        let result = sut.connect().await;
        assert!(result.is_err_and(|e|e.to_string()=="password not configured"));

        config.insert_str(CFG_PASSWORD, "password");
        let sut = ConnectionConfiguration::from(&config);
        let result = sut.connect().await;
        println!("{:?}", result);
        assert!(result.is_err()); // url is not a valid url
    }
}
