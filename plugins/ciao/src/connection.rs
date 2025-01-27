use ciao_rs::ciao::ClientManager;
use model::BoxedError;
use tokio::runtime::Runtime;

use crate::config::ConnectionConfiguration;

pub struct CiaoConnection {
    pub connection_config: Option<ConnectionConfiguration>,
    pub client: Option<ClientManager>,
}

impl CiaoConnection {
    /// Connect to the gRPC services
    pub fn connect(config: Option<model::xml::config::Configuration>) -> Result<Self, BoxedError> {
        if let Some(config) = config {
            let rt = Runtime::new()?;
            let result: Result<Self, BoxedError> = rt.block_on(async {
                let (connection, client) = CiaoConnection::_connect(config).await?;

                Ok(CiaoConnection {
                    connection_config: Some(connection),
                    client: Some(client),
                })
            });
            result
        } else {
            Err("Configuration incomplete".into())
        }
    }

    /// Async function to connect to gRPC services
    async fn _connect(
        config: model::xml::config::Configuration,
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

    /// Helper function to get the ClientManager
    pub fn client(connection: &mut Option<CiaoConnection>) -> Option<&mut ClientManager> {
        if let Some(ref mut connection) = connection {
            connection.client.as_mut()
        } else {
            None
        }
    }
}
