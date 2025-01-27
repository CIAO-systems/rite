use ciao_rs::ciao::time_tracking::project::ListRequest;
use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{field::Field, record::Record, BoxedError, Initializable};
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
            rt.block_on(async {
                let request = ListRequest { active_at: None };
                match client.project_client.inner_mut().list(request).await {
                    Ok(response) => {
                        let mut stream = response.into_inner();
                        while let Some(response) = stream.next().await {
                            match response {
                                Ok(response) => {
                                    //
                                    for project in response.projects {
                                        let s = format!("{:#?}", project);
                                        let mut record = Record::new();
                                        record.fields_as_mut().push(Field::new_string(
                                            "debug".to_string(),
                                            s.clone(),
                                        ));
                                        println!("{s}");

                                        if let Err(e) = handler.handle_record(&mut record) {
                                            log::error!("Error while handling record: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("{}", e);
                                }
                            }
                        }
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            })?;
        }
        Ok(())
    }

    fn reset(&mut self) -> Result<(), BoxedError> {
        // Not supported
        Ok(())
    }
}
