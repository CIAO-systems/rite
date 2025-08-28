use ciao_rs::ciao::devices::{
    device_action::Configuration::{Clock, Custom},
    DeviceActionType, DeviceType, ListRequest,
};
use futures::StreamExt;
use model::import::{Importer, RecordHandler};
use model::{
    field::{add_field, add_optional_field},
    record::Record,
    value::Value,
    BoxedError, Initializable,
};

use crate::connection::CiaoConnection;

pub struct Devices {
    config: Option<model::xml::config::Configuration>,
}

impl Devices {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for Devices {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Devices {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), BoxedError> {
        // 1. Establish connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.device_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_devices(service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn list_devices(
    mut service_client: ciao_rs::ciao::clients::devices::DeviceClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), BoxedError> {
    let mut stream = service_client
        .inner_mut()
        .list(ListRequest {})
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(response) => {
                for device in response.devices {
                    handle_device(&device, handler)?;
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn handle_device(
    device: &ciao_rs::ciao::devices::Device,
    handler: &mut dyn RecordHandler,
) -> Result<(), BoxedError> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::String(device.id.clone()));
    add_field(
        fields,
        "externalId",
        Value::String(device.external_id.clone()),
    );
    add_field(
        fields,
        "name",
        Value::String(device.name.clone()),
    );
    add_field(
        fields,
        "type",
        Value::String(
            DeviceType::try_from(device.r#type)?
                .as_str_name()
                .to_string(),
        ),
    );
    add_optional_field(fields, "timeZone", device.time_zone_id.clone());

    // Add all actions
    let mut index = 0;
    for action in &device.actions {
        let element_prefix = format!("action[{}]", index);
        index += 1;

        add_field(
            fields,
            &format!("{element_prefix}.id"),
            Value::I32(action.id.clone()),
        );
        add_field(
            fields,
            &format!("{element_prefix}.deviceActionId"),
            Value::I32(action.device_action_id.clone()),
        );
        add_optional_field(
            fields,
            &format!("{element_prefix}.icon"),
            action.icon.clone(),
        );
        add_optional_field(
            fields,
            &format!("{element_prefix}.name"),
            action.name.clone(),
        );
        add_field(
            fields,
            &format!("{element_prefix}.type"),
            Value::String(
                DeviceActionType::try_from(action.r#type)?
                    .as_str_name()
                    .to_string(),
            ),
        );
        if let Some(ref configuration) = action.configuration {
            match configuration {
                Clock(clock_configuration) => {
                    let element_prefix = format!("{element_prefix}.configuration.clock");
                    add_field(
                        fields,
                        &format!("{element_prefix}.timeTypeId"),
                        Value::String(clock_configuration.time_type_id.clone()),
                    );
                    add_field(
                        fields,
                        &format!("{element_prefix}.costCenterId"),
                        Value::String(clock_configuration.cost_center_id.clone()),
                    );
                    add_field(
                        fields,
                        &format!("{element_prefix}.projectId"),
                        Value::String(clock_configuration.project_id.clone()),
                    );
                    add_field(
                        fields,
                        &format!("{element_prefix}.projectTaskId"),
                        Value::String(clock_configuration.project_task_id.clone()),
                    );
                }
                Custom(custom_configuration) => {
                    let element_prefix = format!("{element_prefix}.configuration.custom");
                    add_field(
                        fields,
                        &format!("{element_prefix}.operation"),
                        Value::String(custom_configuration.operation.clone()),
                    );
                }
            }
        }
    }

    handler.handle_record(&mut record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use model::import::{handlers::CollectingRecordHandler, Importer};
    use model::{xml::config::Configuration, Initializable};

    use crate::importers::devices::Devices;

    #[test]
    #[ignore = "for manual testing"]
    fn test_device_importer() -> Result<(), Box<dyn std::error::Error>> {
        let mut importer = Devices::new();
        let mut config = Configuration::new();
        config.insert_str("url", "http://localhost:50051");
        config.insert_str("api-key", "top-secret-api-key");

        importer.init(Some(config))?;
        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        importer.read(&mut handler)?;

        assert!(records.len() > 0);
        for record in records {
            println!("{:?}", record);
            assert!(record.field_by_name("id").is_some());
            assert!(record.field_by_name("name").is_some());
        }
        Ok(())
    }
}
