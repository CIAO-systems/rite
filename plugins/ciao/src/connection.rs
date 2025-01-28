use std::future::Future;

use ciao_rs::ciao::ClientManager;
use model::BoxedError;
use tokio::runtime::Runtime;

use crate::config::ConnectionConfiguration;

#[derive(Debug)]
pub struct CiaoConnection {
    pub runtime: Option<Runtime>,
    pub connection_config: Option<ConnectionConfiguration>,
    pub client: Option<ClientManager>,
}

impl CiaoConnection {
    /// Connect to the gRPC services
    pub fn connect(config: &Option<model::xml::config::Configuration>) -> Result<Self, BoxedError> {
        if let Some(config) = config {
            let rt = Runtime::new()?;
            let mut result: Result<Self, BoxedError> = rt.block_on(async {
                match CiaoConnection::_connect(config).await {
                    Ok((connection, client)) => Ok(CiaoConnection {
                        runtime: None,
                        connection_config: Some(connection),
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

    /// Helper function to get the ClientManager
    pub fn client(connection: &mut CiaoConnection) -> Option<&mut ClientManager> {
        connection.client.as_mut()
    }

    pub fn block_on<F: Future>(&mut self, future: F) -> F::Output {
        if let Some(ref rt) = self.runtime {
            rt.block_on(future)
        } else {
            panic!("Argh")
        }
    }
}

#[cfg(test)]
mod tests;
