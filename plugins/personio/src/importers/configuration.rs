use model::BoxedError;
use personio_rs::auth::login;
use tokio::runtime::Runtime;

pub const CFG_CLIENT_ID: &str = "client_id";
pub const CFG_CLIENT_SECRET: &str = "client_secret";
pub const CFG_PARTNER_ID: &str = "X-Personio-Partner-ID";
pub const CFG_APP_ID: &str = "X-Personio-App-ID";

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
    pub personio_headers: PersonioHeaders,
}

impl GeneralConfiguration {
    pub fn new() -> Self {
        GeneralConfiguration {
            token: None,
            runtime: None,
            personio_headers: PersonioHeaders::new(),
        }
    }

    /// Load the general configuration from the [Configuration]
    pub fn load(config: &model::xml::config::Configuration) -> Result<Self, BoxedError> {
        let (token, runtime) = init_auth(config)?;
        Ok(GeneralConfiguration {
            token: Some(token),
            runtime: Some(runtime),
            personio_headers: PersonioHeaders::init(
                config.get(CFG_PARTNER_ID),
                config.get(CFG_APP_ID),
            ),
        })
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
