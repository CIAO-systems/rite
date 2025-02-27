use std::collections::HashMap;

use chrono::{NaiveDate, TimeZone, Utc};
use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{field::add_field, record::Record, BoxedError, Initializable};
use prost_types::Timestamp;

use crate::{
    com::atoss::atc::protobuf::{
        field::Value,
        filter::{
            parameter_meta_data::{First, TreatmentType},
            ParameterMetaData,
        },
        Field, Filter,
    },
    connection::ATCConnection,
};

use super::common::{
    add_fields_filter, add_to_parameter_metadata, atc_value_to_model_value,
    create_parameter_meta_data_single,
};

pub struct ClockRecords {
    config: Option<model::xml::config::Configuration>,
}

const CFG_FILTER_EMPLOYEE: &str = "filter.employee";
const CFG_FILTER_PERIOD: &str = "filter.period";

const ATC_FILTER_EMPLOYEE: &str = "employee";
const ATC_FILTER_TIMESTAMP: &str = "timestamp";

impl ClockRecords {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for ClockRecords {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for ClockRecords {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establich connection to gRPC server
        let connection = ATCConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.dataset_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    if let Some(ref config) = self.config {
                        call_get_clock_records(config, service_client, handler).await?;
                    }

                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn call_get_clock_records(
    config: &model::xml::config::Configuration,
    mut service_client: crate::connection::clients::DataSetClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let table = "Clockin".to_string();

    let mut parameter_meta_data = HashMap::new();

    add_employee_filter(&mut parameter_meta_data, &config)?;
    add_period_filter(&mut parameter_meta_data, &config)?;
    add_fields_filter(&mut parameter_meta_data, &config)?;

    let request = Filter {
        table: table.clone(),
        parameter_meta_data,
    };

    let mut stream = service_client.inner_mut().get(request).await?.into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                let mut record = Record::new();
                let fields = record.fields_as_mut();
                for (_, field) in r.field {
                    if let Some(model_value) = atc_value_to_model_value(field.value) {
                        add_field(fields, &field.name, model_value);
                    }
                }
                handler.handle_record(&mut record)?;
            }
            Err(e) => {
                log::error!("Error processing dataset stream for '{table}': {e}");
            }
        }
    }

    Ok(())
}

fn parse_period(period: &str) -> (Option<NaiveDate>, Option<NaiveDate>) {
    let parts: Vec<&str> = period.split(':').collect();

    let start = if parts.get(0).is_some() && !parts[0].is_empty() {
        match NaiveDate::parse_from_str(parts[0], "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(e) => {
                log::error!("Error parsing period start: {e}");
                None
            }
        }
    } else {
        None
    };

    let end = if parts.get(1).is_some() && !parts[1].is_empty() {
        match NaiveDate::parse_from_str(parts[1], "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(e) => {
                log::error!("Error parsing period end: {e}");
                None
            }
        }
    } else {
        None
    };

    (start, end)
}

fn date_to_protobuf(naive_date: &NaiveDate) -> Result<Timestamp, BoxedError> {
    if let Some(naive_datetime) = naive_date.and_hms_opt(0, 0, 0) {
        // Convert the NaiveDateTime to a DateTime<Utc>.
        let utc_datetime = Utc.from_utc_datetime(&naive_datetime);

        // Create a protobuf Timestamp from the NaiveDate
        let timestamp = Timestamp {
            seconds: utc_datetime.timestamp(),
            nanos: 0,
        };
        Ok(timestamp)
    } else {
        Err("Could not add midnight to date".into())
    }
}

fn add_period_filter(
    parameter_meta_data: &mut HashMap<String, ParameterMetaData>,
    config: &model::xml::config::Configuration,
) -> Result<(), Box<dyn std::error::Error>> {
    /*
       "timestamp": {
         "treatmentType": "pttIntervalCloseClose",
         "lower": {
           "name": "timestamp",
           "timestampValue": "2024-10-01T00:00:00Z"
         },
         "upper": {
           "name": "timestamp",
           "timestampValue": "2024-10-30T00:00:00Z"
         }
       }
    */
    if let Some(period_str) = config.get(CFG_FILTER_PERIOD) {
        let (start, end) = parse_period(&period_str);
        if start.is_some() || end.is_some() {
            // Only add filter if either start or end is given
            let lower = create_period_filter_field(start.or(NaiveDate::from_ymd_opt(1, 1, 1)))?;
            let upper = create_period_filter_field(end.or(NaiveDate::from_ymd_opt(9999, 1, 1)))?;

            let filter = ParameterMetaData {
                treatment_type: TreatmentType::PttIntervalCloseClose.into(),
                upper,
                first: lower.map(|l| First::Lower(l)),
            };
            parameter_meta_data.insert(ATC_FILTER_TIMESTAMP.to_string(), filter);
        }
    }
    Ok(())
}

fn create_period_filter_field(date: Option<NaiveDate>) -> Result<Option<Field>, BoxedError> {
    Ok(if let Some(date) = date {
        Some(Field {
            name: ATC_FILTER_TIMESTAMP.to_string(),
            value: Some(Value::TimestampValue(date_to_protobuf(&date)?)),
        })
    } else {
        None
    })
}

fn add_employee_filter(
    parameter_meta_data: &mut HashMap<String, ParameterMetaData>,
    config: &model::xml::config::Configuration,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(employee) = config.get(CFG_FILTER_EMPLOYEE) {
        let filter = create_parameter_meta_data_single(
            TreatmentType::PttNone,
            ATC_FILTER_EMPLOYEE,
            employee,
        );
        add_to_parameter_metadata(parameter_meta_data, filter);
    }

    Ok(())
}

#[cfg(test)]
mod tests;
