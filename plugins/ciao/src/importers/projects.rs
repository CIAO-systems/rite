use ciao_rs::ciao::{
    clients::time_tracking::projects::ProjectClient,
    time_tracking::project::{ListRequest, Project},
};
use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{
    field::{add_field, add_optional_field},
    record::Record,
    value::Value,
    BoxedError, Initializable,
};

use crate::connection::CiaoConnection;

pub struct Projects {
    config: Option<model::xml::config::Configuration>,
}

impl Projects {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for Projects {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Projects {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establich connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.project_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_projects(service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn list_projects(
    mut pc: ProjectClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = pc
        .inner_mut()
        .list(ListRequest { active_at: None })
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for project in r.projects {
                    handle_project(&project, handler)?;
                }
            }
            Err(e) => {
                log::error!("Error processing project stream: {e}");
            }
        }
    }

    Ok(())
}

fn handle_project(
    project: &Project,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::String(project.id.clone()));
    add_field(fields, "name", Value::String(project.name.clone()));
    add_optional_field(fields, "external_id", project.external_id.clone());

    handler.handle_record(&mut record)?;

    Ok(())
}
