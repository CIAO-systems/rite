use ciao_rs::ciao::time_tracking::{clock_record::Identity, ClockRecord};
use model::export::Exporter;
use model::{field::Field, value::Value, BoxedError, Initializable};

use crate::connection::CiaoConnection;

pub struct ClockEntries {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl ClockEntries {
    pub(crate) fn new() -> Self {
        ClockEntries {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for ClockEntries {
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

impl Exporter for ClockEntries {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.time_tracking_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        clock(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }

        Ok(())
    }
}

async fn clock(
    service_client: &mut ciao_rs::ciao::clients::time_tracking::TimeTrackingClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let clock_request = clock_record_from(record)?;
    let response = service_client
        .inner_mut()
        .clock(clock_request)
        .await?
        .into_inner();
    log::info!("Clock response: {}", response.message);
    Ok(())
}

fn clock_record_from(record: &model::record::Record) -> Result<ClockRecord, model::BoxedError> {
    let timestamp = get_timestamp(record)?;
    let identity = get_identity(record)?;

    let device_id = record.field_by_name("deviceId");
    let time_type_id = record.field_by_name("timeTypeId");
    let project_id = record.field_by_name("projectId");
    let project_task_id = record.field_by_name("projectTaskId");
    let cost_center_id = record.field_by_name("costcenterId");

    Ok(ClockRecord {
        timestamp: Some(timestamp),
        device_id: device_id.map(|f| f.value().to_string()),
        time_type_id: time_type_id.map(|f| f.value().to_string()),
        project_id: project_id.map(|f| f.value().to_string()),
        cost_center_id: cost_center_id.map(|f| f.value().to_string()),
        project_task_id: project_task_id.map(|f| f.value().to_string()),
        id: None,
        identity: Some(identity),
    })
}

fn get_identity(record: &model::record::Record) -> Result<Identity, BoxedError> {
    if let Some(user_id) = record.field_by_name("identity.userId") {
        Ok(Identity::UserId(user_id.value().to_string()))
    } else if let Some(badge_id) = record.field_by_name("identity.badgeId") {
        Ok(Identity::BadgeId(badge_id.value().to_string()))
    } else {
        Err("Neither 'identity.userId' nor 'identity.badgeId' found in record".into())
    }
}

fn get_timestamp(
    record: &model::record::Record,
) -> Result<ciao_rs::ciao::common::Timestamp, BoxedError> {
    let time_utc = get_field(record, "timestamp.timeUtc")?;
    let time_zone = get_field(record, "timestamp.timeZone")?;

    if let Value::I64(time_utc_millis) = time_utc.value() {
        let ts = prost_types::Timestamp {
            seconds: time_utc_millis / 1000,
            nanos: ((time_utc_millis as i64 % 1000) * 1_000_000) as i32, // Convert remaining milliseconds to nanoseconds
        };

        Ok(ciao_rs::ciao::common::Timestamp {
            time_utc: Some(ts),
            time_zone: time_zone.value().to_string(),
        })
    } else {
        Err("Field 'timestamp.timeUtc' is not a i64".into())
    }
}

fn get_field<'a>(
    record: &'a model::record::Record,
    name: &str,
) -> Result<&'a Field, model::BoxedError> {
    if let Some(field) = record.field_by_name(name) {
        Ok(field)
    } else {
        Err(format!("Field '{name}' missing in record").into())
    }
}

#[cfg(test)]
mod tests;

