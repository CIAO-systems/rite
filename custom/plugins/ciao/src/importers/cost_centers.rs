use ciao_rs::ciao::time_tracking::cost_center::ListRequest;
use futures::StreamExt;
use model::import::{Importer, RecordHandler};
use model::{field::add_field, record::Record, value::Value, BoxedError, Initializable};

use crate::connection::CiaoConnection;

pub struct CostCenters {
    config: Option<model::xml::config::Configuration>,
}

impl CostCenters {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for CostCenters {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for CostCenters {
    fn read(
        &mut self,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establish connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.cost_center_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_cost_centers(service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn list_cost_centers(
    mut service_client: ciao_rs::ciao::clients::time_tracking::cost_centers::CostCenterClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = service_client
        .inner_mut()
        .list(ListRequest {})
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for cost_center in r.cost_centers {
                    handle_cost_center(&cost_center, handler)?;
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn handle_cost_center(
    cost_center: &ciao_rs::ciao::time_tracking::cost_center::CostCenter,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();

    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::String(cost_center.id.clone()));
    add_field(fields, "name", Value::String(cost_center.name.clone()));

    handler.handle_record(&mut record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use model::import::{handlers::CollectingRecordHandler, Importer};
    use model::{xml::config::Configuration, Initializable};

    use super::CostCenters;

    #[test]
    #[ignore = "for manual testing"]
    fn test_cost_center_importer() -> Result<(), Box<dyn std::error::Error>> {
        let mut importer = CostCenters::new();
        let mut config = Configuration::new();
        config.insert_str("url", "http://localhost:50051");
        config.insert_str("api-key", "top-secret-api-key");

        importer.init(Some(config))?;
        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        importer.read(&mut handler)?;

        assert!(records.len() > 0);
        for cc in records {
            println!("{:?}", cc);
            assert!(cc.field_by_name("id").is_some());
            assert!(cc.field_by_name("name").is_some());
        }
        Ok(())
    }
}
