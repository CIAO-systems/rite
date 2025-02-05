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
                    list_time_types(service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn list_time_types(
    mut ttc: TimeTypeClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    // FIXME implement me

    let mut stream = ttc
        .inner_mut()
        .list(ListRequest {
            absence: None,  // TODO make configurable
            bookable: None, // TODO make configurable
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
                log::error!("Error processing project stream: {e}");
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
        "description",
        Value::String(time_type.description.clone()),
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
