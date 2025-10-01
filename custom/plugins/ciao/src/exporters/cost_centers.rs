use ciao_rs::ciao::time_tracking::cost_center::{CostCenter, CreateRequest};
use model::export::Exporter;
use model::{BoxedError, Initializable};

use crate::connection::CiaoConnection;

pub struct CostCenters {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl CostCenters {
    pub(crate) fn new() -> Self {
        CostCenters {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for CostCenters {
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

impl Exporter for CostCenters {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.cost_center_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        create_cost_center(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }

        Ok(())
    }
}

fn cost_center_from(record: &model::record::Record) -> Result<CostCenter, BoxedError> {
    let id = record.field_by_name("id");
    let name = record.field_by_name("name");
    let external_id = record.field_by_name("externalId");
    if id.is_some() && name.is_some() {
        return Ok(CostCenter {
            id: id.unwrap().value().to_string(),
            name: name.unwrap().value().to_string(),
            external_id: external_id.map(|f| f.value_as_ref().to_string()),
        });
    }

    Err("Missing mandatory fields".to_string().into())
}

async fn create_cost_center(
    service_client: &mut ciao_rs::ciao::clients::time_tracking::cost_centers::CostCenterClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let cost_center = cost_center_from(record)?;
    let result = service_client
        .inner_mut()
        .create(CreateRequest {
            cost_center: Some(cost_center),
        })
        .await?
        .into_inner();
    if result.cost_center.is_some() {
        Ok(())
    } else {
        Err("Error while creating costcenter".into())
    }
}

#[cfg(test)]
mod tests;
