use ciao_rs::ciao::time_tracking::project::{CreateRequest, Project};
use model::export::Exporter;
use model::Initializable;

use crate::{connection::CiaoConnection, model::get_timestamp};

pub struct Projects {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl Projects {
    pub fn new() -> Self {
        Projects {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for Projects {
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

impl Exporter for Projects {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.project_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        create_project(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }
        Ok(())
    }
}

async fn create_project(
    service_client: &mut ciao_rs::ciao::clients::time_tracking::projects::ProjectClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let request = CreateRequest {
        project: Some(project_from_record(record)),
    };

    let response = service_client
        .inner_mut()
        .create(request)
        .await?
        .into_inner();
    log::info!("Project {:?} created", response);
    Ok(())
}

fn project_from_record(record: &model::record::Record) -> Project {
    Project {
        id: record
            .field_by_name("id")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
        external_id: record
            .field_by_name("externalId")
            .map(|v| v.value().to_string()),
        name: record
            .field_by_name("name")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
        start_date: match get_timestamp(record, "startDate") {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("{e}");
                None
            }
        },
        end_date: match get_timestamp(record, "endDate") {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("{e}");
                None
            }
        },
        closed_date: match get_timestamp(record, "closedDate") {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("{e}");
                None
            }
        },
        parent_id: record
            .field_by_name("parentId")
            .map(|v| Some(v.value().to_string()))
            .unwrap_or(None),
    }
}

#[cfg(test)]
mod tests;
