use ciao_rs::ciao::ClientManager;
use model::BoxedError;
use tokio::runtime::Runtime;

use crate::config::ConnectionConfiguration;

#[derive(Debug)]
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
                match CiaoConnection::_connect(config).await {
                    Ok((connection, client)) => Ok(CiaoConnection {
                        connection_config: Some(connection),
                        client: Some(client),
                    }),
                    Err(e) => {
                        log::error!("Error on connect: {e}");
                        Err(e)
                    }
                }
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

#[cfg(test)]
mod tests {
    use ciao_rs::ciao::time_tracking::project::GetRequest;
    use model::{
        xml::config::{ConfigItem, Configuration},
        BoxedError,
    };
    use tokio::runtime::Runtime;

    use super::CiaoConnection;

    #[test]
    #[ignore = "for manual testing"]
    fn manual_connection() -> Result<(), BoxedError> {
        let config = Some(Configuration {
            xml: None,
            config: Some(vec![
                ConfigItem {
                    key: String::from("url"),
                    value: String::from("https://backend-api.ciao.software:443"),
                },
                ConfigItem {
                    key: String::from("api-key"),
                    value: String::from(""),
                },
            ]),
        });

        if let Ok(connection) = CiaoConnection::connect(config) {
            let mut connection_opt = Some(connection);
            let mut client_ref = CiaoConnection::client(&mut connection_opt);
            let future = if let Some(ref mut client) = client_ref {
                let pc = &mut client.project_client;
                Some(pc.inner_mut().get(GetRequest { id: "".to_string() }))
            } else {
                None
            };

            if let Some(future) = future {
                let rt = Runtime::new()?;
                rt.block_on(async {
                    match future.await {
                        Ok(r) => println!("{:?}", r),
                        Err(e) => println!("Error: {e}"),
                    }
                });
            }
        }

        Ok(())
    }
}
