use clients::manager::ClientManager;
use config::ConnectionConfiguration;
use model::BoxedError;
use tokio::runtime::Runtime;

pub mod clients;
pub mod config;
pub mod interceptor;

#[derive(Debug)]
pub struct ATCConnection {
    pub runtime: Option<Runtime>,
    pub connection_config: Option<ConnectionConfiguration>,
    pub client: Option<ClientManager>,
}

impl ATCConnection {
    /// Connect to the gRPC services
    pub fn connect(config: &Option<model::xml::config::Configuration>) -> Result<Self, BoxedError> {
        if let Some(config) = config {
            let rt = Runtime::new()?;
            let mut result: Result<Self, BoxedError> = rt.block_on(async {
                match ATCConnection::_connect(config).await {
                    Ok((connection_config, client)) => Ok(ATCConnection {
                        runtime: None,
                        connection_config: Some(connection_config),
                        client: Some(client),
                    }),
                    Err(e) => {
                        log::error!("Error on connect: {e}");
                        Err(e)
                    }
                }
            });
            if let Ok(connection) = &mut result {
                connection.runtime = Some(rt);
            }
            result
        } else {
            Err("Configuration incomplete".into())
        }
    }

    /// Async function to connect to gRPC services
    async fn _connect(
        config: &model::xml::config::Configuration,
    ) -> Result<(ConnectionConfiguration, ClientManager), BoxedError> {
        if let Some(connection) = Some(ConnectionConfiguration::from(&config)) {
            let client = match connection.connect().await {
                Ok(client) => Ok(client),
                Err(e) => Err(e),
            }?;
            return Ok((connection, client));
        } else {
            Err("Could not read configuration".into())
        }
    }
}

#[cfg(test)]
mod tests {

    use model::xml::config::Configuration;

    use crate::connection::{
        clients::manager::tests::mocks::start_mock_server,
        config::{CFG_AUTH_TOKEN, CFG_PASSWORD, CFG_URL, CFG_USER},
        ATCConnection,
    };

    #[test]
    fn test_connect_no_config() {
        let result = ATCConnection::connect(&None);
        assert!(result.is_err_and(|e| e.to_string() == "Configuration incomplete"));
    }

    #[test]
    fn test_connect_empty_config() {
        let config = Configuration::new();
        let result = ATCConnection::connect(&Some(config));
        println!("{:?}", result);
        assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
    }

    #[tokio::test]
    async fn test_connect_success() {
        let addr = start_mock_server(50051).await;

        let mut config = Configuration::new();
        config.insert_str(CFG_URL, &format!("http://{}", addr));
        config.insert_str(CFG_USER, "user");
        config.insert_str(CFG_PASSWORD, "password");
        config.insert_str(CFG_AUTH_TOKEN, "auth-token");

        let result = ATCConnection::_connect(&config).await;
        println!("{:?}", result);
        assert!(result.is_ok());

        let (connection_config, _client_manager) = result.unwrap();
        assert_eq!(connection_config.url, Some("http://127.0.0.1:50051".into()));
    }
}
