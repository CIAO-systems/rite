use model::BoxedError;
use personio_rs::{auth::login, personnel::apis::configuration::Configuration};
use tokio::runtime::Runtime;

const CFG_CLIENT_ID: &str = "client_id";
const CFG_CLIENT_SECRET: &str = "client_secret";
const CFG_PARTNER_ID: &str = "X-Personio-Partner-ID";
const CFG_APP_ID: &str = "X-Personio-App-ID";
const CFG_OPTIONS_LIMIT: &str = "options.limit";

pub struct PersonioHeaders {
    pub partner_id: Option<String>,
    pub app_id: Option<String>,
}

impl PersonioHeaders {
    fn new() -> PersonioHeaders {
        PersonioHeaders {
            partner_id: None,
            app_id: None,
        }
    }

    fn init(partner_id: Option<String>, app_id: Option<String>) -> PersonioHeaders {
        PersonioHeaders { partner_id, app_id }
    }
}

pub struct GeneralConfiguration {
    pub token: Option<String>,
    pub runtime: Option<Runtime>,
    pub limit: Option<i32>,
    pub personio_headers: PersonioHeaders,
}

impl GeneralConfiguration {
    pub fn new() -> Self {
        GeneralConfiguration {
            token: None,
            runtime: None,
            limit: None,
            personio_headers: PersonioHeaders::new(),
        }
    }

    fn with_config_token_and_runtime(
        config: &model::xml::config::Configuration,
        token: String,
        runtime: Runtime,
    ) -> Self {
        GeneralConfiguration {
            token: Some(token),
            runtime: Some(runtime),
            limit: config
                .get(CFG_OPTIONS_LIMIT)
                .and_then(|s| s.parse::<i32>().ok()),
            personio_headers: PersonioHeaders::init(
                config.get(CFG_PARTNER_ID),
                config.get(CFG_APP_ID),
            ),
        }
    }

    /// Load the general configuration from the [Configuration]
    pub fn load(config: &model::xml::config::Configuration) -> Result<Self, BoxedError> {
        let (token, runtime) = init_auth(config)?;
        Ok(GeneralConfiguration::with_config_token_and_runtime(
            config, token, runtime,
        ))
    }

    /// Get the Configuration with the `bearer_access_token`
    pub fn get_personnel_configuration(&self) -> Result<Configuration, BoxedError> {
        if let Some(ref token) = self.token {
            let mut configuration = Configuration::new();
            configuration.bearer_access_token = Some(token.clone());
            Ok(configuration)
        } else {
            Err("No valid token stored".into())
        }
    }
}

/// Login to Personio and create a Tokio runtime
fn init_auth(config: &model::xml::config::Configuration) -> Result<(String, Runtime), BoxedError> {
    if let Some(client_id) = config.get(CFG_CLIENT_ID) {
        if let Some(client_secret) = config.get(CFG_CLIENT_SECRET) {
            let runtime = Runtime::new()?;
            let result: Result<String, BoxedError> =
                runtime.block_on(async { Ok(login(client_id, client_secret).await?) });
            match result {
                Ok(token) => {
                    // We have a valid token now, store it and the tokio runtime
                    Ok((token, runtime))
                }
                Err(e) => Err(e),
            }
        } else {
            Err("client_secret missing".into())
        }
    } else {
        Err("client_id missing".into())
    }
}

#[cfg(test)]
mod tests {
    use model::{BoxedError, xml::config::Configuration};
    use tokio::runtime::Runtime;

    use crate::importers::configuration::{CFG_APP_ID, CFG_OPTIONS_LIMIT, CFG_PARTNER_ID};

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
}
