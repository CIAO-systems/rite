use std::collections::HashMap;

use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{field::add_field, record::Record, value::Value, BoxedError, Initializable};

use crate::{
    com::atoss::atc::protobuf::{
        field::Value::{
            BinaryValue, DoubleValue, DoublesValue, DurationValue, DurationsValue, IntValue,
            IntsValue, ListValue, RecordValue, StringValue, StringsValue, TimestampValue,
            TimestampsValue,
        },
        filter::{
            parameter_meta_data::{First, TreatmentType},
            ParameterMetaData,
        },
        Field, Filter,
    },
    connection::{
        config::{CFG_FILTER_FIELDS, CFG_FILTER_TABLE},
        ATCConnection,
    },
};

pub struct Dataset {
    config: Option<model::xml::config::Configuration>,
}

impl Dataset {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for Dataset {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Dataset {
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
                        call_dataset_get(config, service_client, handler).await?;
                    }

                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn call_dataset_get(
    config: &model::xml::config::Configuration,
    mut service_client: crate::connection::clients::DataSetClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let table = match config.get(CFG_FILTER_TABLE) {
        Some(table) => table,
        None => return Err(format!("Parameter '{}' missing", CFG_FILTER_TABLE).into()),
    };

    let mut parameter_meta_data = HashMap::new();
    if let Some(fields) = config.get(CFG_FILTER_FIELDS) {
        add_fields_filter(fields, &mut parameter_meta_data);
    }

    let request = Filter {
        table,
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
                log::error!("Error processing dataset stream: {e}");
            }
        }
    }

    Ok(())
}

fn add_fields_filter(fields: String, parameter_meta_data: &mut HashMap<String, ParameterMetaData>) {
    let fields_filter = ParameterMetaData {
        treatment_type: TreatmentType::PttNone.into(),
        upper: None,
        first: Some(First::Value(Field {
            name: "fields".to_string(),
            value: Some(StringValue(fields)),
        })),
    };
    parameter_meta_data.insert(String::from("fields"), fields_filter);
}

fn atc_value_to_model_value(
    atc_value: Option<crate::com::atoss::atc::protobuf::field::Value>,
) -> Option<model::value::Value> {
    if let Some(value) = atc_value {
        match value {
            IntValue(v) => Some(Value::I32(v)),
            StringValue(v) => Some(Value::String(v)),
            DoubleValue(v) => Some(Value::F64(v)),
            BinaryValue(v) => Some(Value::Blob(v)),
            TimestampValue(v) => Some(Value::String(v.to_string())),
            RecordValue(_v) => None, // FIXME recursive field adding with prefix "<name>."?
            DurationValue(_v) => None, // FIXME duration
            // FIXME implement the lists
            ListValue(_v) => None,
            IntsValue(_v) => None,
            StringsValue(_v) => None,
            DoublesValue(_v) => None,
            TimestampsValue(_v) => None,
            DurationsValue(_v) => None,
        }
    } else {
        None
    }
}
