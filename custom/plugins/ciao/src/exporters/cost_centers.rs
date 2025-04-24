use ciao_rs::ciao::time_tracking::cost_center::{CostCenter, CreateRequest};
use export::Exporter;
use model::Initializable;

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
        // 1. Establich connection to gRPC server
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

async fn create_cost_center(
    service_client: &mut ciao_rs::ciao::clients::time_tracking::cost_centers::CostCenterClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let id = record.field_by_name("id");
    let name = record.field_by_name("name");
    if id.is_some() && name.is_some() {
        let cost_center = CostCenter {
            id: id.unwrap().value().to_string(),
            name: name.unwrap().value().to_string(),
        };
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
    } else {
        Err("Missing mandatory fields".into())
    }
}

#[cfg(test)]
mod tests {
    use export::Exporter;
    use model::{field::add_field, record::Record, value::Value, xml::config::Configuration, BoxedError, Initializable};

    use super::CostCenters;

    #[test]
    #[ignore = "for manual testing"]
    fn test_create_cost_center() -> Result<(), BoxedError> {
        let mut exporter = CostCenters::new();
        let mut config = Configuration::new();
        config.insert_str("url", "http://localhost:50051");
        config.insert_str("api-key", "top-secret-api-key");

        exporter.init(Some(config))?;

        let mut record = Record::new();
        let fields = record.fields_as_mut();
        add_field(
            fields,
            "id",
            Value::String("new-cost-center-id".to_string()),
        );
        add_field(
            fields,
            "name",
            Value::String("new-cost-center-name".to_string()),
        );

        exporter.write(&record)?;

        Ok(())
    }

}