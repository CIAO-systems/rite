use import::{Importer, RecordHandler};
use model::{BoxedError, Initializable};


pub struct Devices {
    config: Option<model::xml::config::Configuration>,
}

impl Devices {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for Devices {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Devices {
    fn read(&mut self, _handler: &mut dyn RecordHandler) -> Result<(), BoxedError> {
        // FIXME implement me
        // if let Some(ref mut client) = CiaoConnection::client(&mut self.connection) {
        //     let rt = Runtime::new()?;
        //     let response: Result<DeviceConfigurationResponse, BoxedError> =
        //         rt.block_on(async { client.device_client.get_device_configuration("3387").await });
        //     println!("{:#?}", response?);
        // }

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
