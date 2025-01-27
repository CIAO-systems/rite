use ciao_rs::ciao::{devices::DeviceConfigurationResponse, ClientManager};
use import::{Importer, RecordHandler};
use model::{BoxedError, Initializable};
use tokio::runtime::Runtime;

use crate::config::CiaoConnection;

pub struct CiaoDevices {
    connection: Option<CiaoConnection>,
    client: Option<ClientManager>,
}

impl CiaoDevices {
    pub fn new() -> Self {
        Self {
            connection: None,
            client: None,
        }
    }
}

async fn __init(
    config: Option<model::xml::config::Configuration>,
) -> Result<(CiaoConnection, ClientManager), BoxedError> {
    if let Some(config) = config {
        if let Some(connection) = Some(CiaoConnection::from(&config)) {
            let rt = Runtime::new()?;
            let client = rt.block_on(async {
                match connection.connect().await {
                    Ok(client) => Ok(client),
                    Err(e) => Err(e),
                }
            })?;
            return Ok((connection, client));
        }
    }
    Err("Configuration incomplete".into())
}

impl Initializable for CiaoDevices {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        let rt = Runtime::new()?;
        let result: Result<(), BoxedError> = rt.block_on(async {
            let (connection, client) = __init(config).await?;
            self.connection = Some(connection);
            self.client = Some(client);
            Ok(())
        });
        result
    }
}

impl Importer for CiaoDevices {
    fn read(&mut self, _handler: &mut dyn RecordHandler) -> Result<(), BoxedError> {
        // FIXME implement me
        if let Some(ref mut client) = self.client {
            let rt = Runtime::new()?;
            let response: Result<DeviceConfigurationResponse, BoxedError> =
                rt.block_on(async { client.device_client.get_device_configuration("3387").await });
            println!("{:#?}", response?);
        }

        Ok(())
    }

    fn reset(&mut self) -> Result<(), BoxedError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ciao_rs::ciao::ClientManager;
    use model::BoxedError;

    async fn error_causing_fn() -> Result<ClientManager, BoxedError> {
        Err("ooops".into())
    }

    async fn calling_fn() -> Result<(), BoxedError> {
        error_causing_fn().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore = "for manual testing"]
    async fn test() {
        match calling_fn().await {
            Ok(_) => println!("fine"),
            Err(e) => println!("error: {e}"),
        }
    }
}
