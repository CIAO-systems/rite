use ciao_rs::ciao::time_tracking::project::task::{CreateRequest, ProjectTask};
use model::export::Exporter;
use model::Initializable;

use crate::connection::CiaoConnection;

pub struct ProjectTasks {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl ProjectTasks {
    pub fn new() -> Self {
        ProjectTasks {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for ProjectTasks {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;
        // 1. Establish connection to gRPC server
        self.connection = Some(CiaoConnection::connect(&self.config)?);
        Ok(())
    }
}

impl Exporter for ProjectTasks {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.project_task_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        create_project_task(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }
        Ok(())
    }
}

async fn create_project_task(
    service_client: &mut ciao_rs::ciao::clients::time_tracking::projects::tasks::ProjectTaskClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let request = CreateRequest {
        task: Some(task_from_record(record)),
    };
    let response = service_client
        .inner_mut()
        .create(request)
        .await?
        .into_inner();
    log::info!("Project task {:?} created", response);

    Ok(())
}

fn task_from_record(
    record: &model::record::Record,
) -> ciao_rs::ciao::time_tracking::project::task::ProjectTask {
    ProjectTask {
        id: record
            .field_by_name("id")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
        project_id: record
            .field_by_name("projectId")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
        name: record
            .field_by_name("name")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
    }
}

#[cfg(test)]
mod tests;
