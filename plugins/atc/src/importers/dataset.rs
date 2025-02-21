use import::{Importer, RecordHandler};
use model::{BoxedError, Initializable};

use crate::connection::ATCConnection;

pub struct Dataset {
    config: Option<model::xml::config::Configuration>,
}

impl Dataset {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for Dataset {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Dataset {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establich connection to gRPC server
        let connection = ATCConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.dataset_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    // FIXME implemnt this
                    // call_dataset_get(service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}
