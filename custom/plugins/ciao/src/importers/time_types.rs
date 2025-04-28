use ciao_rs::ciao::{
    clients::time_tracking::time_type::TimeTypeClient,
    time_tracking::time_type::{ListRequest, TimeType},
};
use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{
    field::{add_field, add_optional_field},
    record::Record,
    value::Value,
    xml::config::get_config_value,
    BoxedError, Initializable,
};

use crate::connection::CiaoConnection;

pub struct TimeTypes {
    config: Option<model::xml::config::Configuration>,
}

impl TimeTypes {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for TimeTypes {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for TimeTypes {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establich connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.time_type_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_time_types(&self.config, service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn list_time_types(
    config: &Option<model::xml::config::Configuration>,
    mut ttc: TimeTypeClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = ttc
        .inner_mut()
        .list(ListRequest {
            absence: get_config_value::<bool>(config, "filter.absence"),
            bookable: get_config_value::<bool>(config, "filter.bookable"),
        })
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for time_type in r.time_types {
                    handle_time_type(&time_type, handler)?;
                }
            }
            Err(e) => {
                log::error!("Error processing time types stream: {e}");
            }
        }
    }

    Ok(())
}

fn handle_time_type(
    time_type: &TimeType,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::String(time_type.id.clone()));
    add_field(
        fields,
        "name",
        Value::String(time_type.name.clone()),
    );
    add_field(
        fields,
        "shorthand",
        Value::String(time_type.shorthand.clone()),
    );
    if let Some(color) = time_type.color {
        add_field(fields, "color.alpha", Value::I32(color.alpha));
        add_field(fields, "color.red", Value::I32(color.red));
        add_field(fields, "color.green", Value::I32(color.green));
        add_field(fields, "color.blue", Value::I32(color.blue));
    }
    add_optional_field(fields, "icon", time_type.icon.clone());
    if let Some(options) = time_type.options {
        add_field(fields, "options.absence", Value::Bool(options.absence));
        add_field(fields, "options.bookable", Value::Bool(options.bookable));
    }

    handler.handle_record(&mut record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use ciao_rs::ciao::{
        common::Color,
        time_tracking::time_type::{TimeType, TimeTypeOptions},
    };
    use import::handlers::CollectingRecordHandler;
    use model::value::Value;

    use super::handle_time_type;

    #[test]
    fn test_handle_time_type() -> Result<(), Box<dyn std::error::Error>> {
        let time_type = TimeType {
            id: "759b2a5d-70fb-4d51-9516-0ab724e36d1d".to_string(),
            name: "A name".to_string(),
            shorthand: "AD".to_string(),
            color: Some(Color {
                alpha: 0,
                red: 255,
                green: 0,
                blue: 0,
            }),
            icon: Some("An Icon".to_string()),
            options: Some(TimeTypeOptions {
                bookable: false,
                absence: false,
            }),
        };
        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);

        handle_time_type(&time_type, &mut handler)?;

        assert_eq!(records.len(), 1);
        let first = records.first().unwrap();

        assert_eq!(
            first.field_by_name("id").unwrap().value(),
            Value::String(time_type.id)
        );
        assert_eq!(
            first.field_by_name("name").unwrap().value(),
            Value::String(time_type.name)
        );
        assert_eq!(
            first.field_by_name("shorthand").unwrap().value(),
            Value::String(time_type.shorthand)
        );
        assert_eq!(
            first.field_by_name("color.alpha").unwrap().value(),
            Value::I32(time_type.color.unwrap().alpha)
        );
        assert_eq!(
            first.field_by_name("color.red").unwrap().value(),
            Value::I32(time_type.color.unwrap().red)
        );
        assert_eq!(
            first.field_by_name("color.green").unwrap().value(),
            Value::I32(time_type.color.unwrap().green)
        );
        assert_eq!(
            first.field_by_name("color.blue").unwrap().value(),
            Value::I32(time_type.color.unwrap().blue)
        );
        assert_eq!(
            first.field_by_name("icon").unwrap().value(),
            Value::String(time_type.icon.unwrap())
        );
        assert_eq!(
            first.field_by_name("options.bookable").unwrap().value(),
            Value::Bool(time_type.options.unwrap().bookable)
        );
        assert_eq!(
            first.field_by_name("options.absence").unwrap().value(),
            Value::Bool(time_type.options.unwrap().absence)
        );

        Ok(())
    }
}
