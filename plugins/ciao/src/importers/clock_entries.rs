use ciao_rs::ciao::time_tracking::ListRequest;
use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{
    record::Record,
    xml::config::get_config_value,
    BoxedError, Initializable,
};

use crate::connection::CiaoConnection;

pub struct ClockEntries {
    config: Option<model::xml::config::Configuration>,
}

impl ClockEntries {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for ClockEntries {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for ClockEntries {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establich connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.time_tracking_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_clock_entries(&self.config, service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn list_clock_entries(
    config: &Option<model::xml::config::Configuration>,
    mut service_client: ciao_rs::ciao::clients::time_tracking::TimeTrackingClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = service_client
        .inner_mut()
        .list(ListRequest {
            time_range: None, // FIXME add function for filter.timeRange
            user_id: get_config_value(config, "filter.userId"),
            creator_id: get_config_value(config, "filter.creatorId"),
            time_type_id: get_config_value(config, "filter.timeTypeId"),
        })
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for clock_entry in r.record {
                    handle_clock_entry(&clock_entry, handler)?;
                }
            }
            Err(e) => {
                log::error!("Error processing project stream: {e}");
            }
        }
    }

    Ok(())
}

fn handle_clock_entry(
    _clock_entry: &ciao_rs::ciao::time_tracking::ClockRecord,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();

    // FIXME implement
    // let fields = record.fields_as_mut();
    // add_field(fields, "id", Value::String(clock_entry.id.clone()));
    // add_optional_field(fields, "icon", time_type.icon.clone());

    handler.handle_record(&mut record)?;

    Ok(())
}
