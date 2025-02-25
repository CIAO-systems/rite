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
