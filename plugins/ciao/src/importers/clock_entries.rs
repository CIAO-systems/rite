use ciao_rs::ciao::time_tracking::{clock_record, ListRequest};
use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{
    field::{add_field, add_optional_field},
    record::Record,
    value::Value,
    xml::config::get_config_value,
    BoxedError, Initializable,
};

use crate::{config::get_config_time_range, connection::CiaoConnection};

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
            time_range: get_config_time_range(config, "filter.timeRange"),
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
                log::error!("Error processing clock entries stream: {e}");
            }
        }
    }

    Ok(())
}

fn handle_clock_entry(
    clock_entry: &ciao_rs::ciao::time_tracking::ClockRecord,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();

    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::String(clock_entry.id.clone().unwrap()));

    if let Some(identity) = &clock_entry.identity {
        let (name, value) = match identity {
            clock_record::Identity::UserId(id) => ("userId", Value::String(id.clone())),
            clock_record::Identity::BadgeId(id) => ("badgeId", Value::String(id.clone())),
        };
        add_field(fields, &format!("identitiy.{}", name), value);
    }

    if let Some(timestamp) = clock_entry.timestamp {
        if let Some(proto_timestamp) = timestamp.time_utc {
            let millis: i64 =
                proto_timestamp.seconds * 1000 + (proto_timestamp.nanos as i64) / 1000000;

            add_field(fields, "timestamp.timeUtc", Value::I64(millis));

            // FIXME when https://github.com/CIAO-systems/ciao-backend/issues/320 is done
            add_field(
                fields,
                "timestamp.zoneId",
                Value::String("Europe/Berlin".to_string()),
            );
        }
    }

    add_optional_field(fields, "deviceId", clock_entry.device_id.clone());
    add_optional_field(fields, "timeTypeId", clock_entry.time_type_id.clone());
    add_optional_field(fields, "projectId", clock_entry.project_id.clone());
    add_optional_field(fields, "projectTaskId", clock_entry.project_task_id.clone());
    add_optional_field(fields, "costCenterId", clock_entry.cost_center_id.clone());

    handler.handle_record(&mut record)?;

    Ok(())
}
