use std::collections::HashMap;

use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{field::add_field, record::Record, BoxedError, Initializable};

use crate::{com::atoss::atc::protobuf::Filter, connection::ATCConnection};

use super::common::atc_value_to_model_value;

pub struct ClockRecords {
    config: Option<model::xml::config::Configuration>,
}

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
    // FIXME add reasonable filters
    add_employee_filter(&mut parameter_meta_data, &config)?;
    add_period_filter(&mut parameter_meta_data, &config)?;

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

fn add_period_filter(
    parameter_meta_data: &mut HashMap<
        String,
        crate::com::atoss::atc::protobuf::filter::ParameterMetaData,
    >,
    config: &&model::xml::config::Configuration,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn add_employee_filter(
    parameter_meta_data: &mut HashMap<
        String,
        crate::com::atoss::atc::protobuf::filter::ParameterMetaData,
    >,
    config: &&model::xml::config::Configuration,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
