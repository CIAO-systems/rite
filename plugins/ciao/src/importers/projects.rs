use import::{Importer, RecordHandler};
use model::{BoxedError, Initializable};
use tokio::runtime::Runtime;

use crate::connection::CiaoConnection;

pub struct CiaoProjects {
    connection: Option<CiaoConnection>,
}

impl CiaoProjects {
    pub fn new() -> Self {
        Self { connection: None }
    }
}

impl Initializable for CiaoProjects {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.connection = Some(CiaoConnection::connect(config)?);
        Ok(())
    }
}

impl Importer for CiaoProjects {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut client) = CiaoConnection::client(&mut self.connection) {
            let rt = Runtime::new()?;
            let response =
                rt.block_on(async { 
                    //
                    client.device_client.get_device_configuration("3387").await 
                });
            println!("{:#?}", response?);
        }
        Ok(())
    }

    fn reset(&mut self) -> Result<(), BoxedError> {
        // Not supported
        Ok(())
    }
}
