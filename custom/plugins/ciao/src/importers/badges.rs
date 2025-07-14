use import::Importer;
use model::Initializable;

pub struct Badges {
    config: Option<model::xml::config::Configuration>,
}

impl Badges {
    pub fn new() -> Self {
        Badges { config: None }
    }
}
impl Initializable for Badges {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Badges {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establish connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_badges(&self.config, service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }
        Ok(())
    }
}
