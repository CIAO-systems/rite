use std::collections::HashMap;

use model::value::Value;

use crate::com::atoss::atc::protobuf::{
    field::Value::{
        BinaryValue, DoubleValue, DoublesValue, DurationValue, DurationsValue, IntValue, IntsValue,
        ListValue, RecordValue, StringValue, StringsValue, TimestampValue, TimestampsValue,
    },
    filter::{
        parameter_meta_data::{First, TreatmentType},
        ParameterMetaData,
    },
    Field,
};

const CFG_FILTER_FIELDS: &str = "filter.fields";
const ATC_FILTER_FIELDS: &str = "fields";

/// Adds the `filter.fields` filter to the [crate::com::atoss::atc::protobuf::filter::ParameterMetaData]
pub fn add_fields_filter(
    parameter_meta_data: &mut HashMap<String, ParameterMetaData>,
    config: &model::xml::config::Configuration,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(fields) = config.get(CFG_FILTER_FIELDS) {
        let filter =
            create_parameter_meta_data_single(TreatmentType::PttNone, ATC_FILTER_FIELDS, fields);
        add_to_parameter_metadata(parameter_meta_data, filter);
    }

    Ok(())
}

/// Adds a [ParameterMetaData] to a map of [ParameterMetaData] using the name from the first
pub fn add_to_parameter_metadata(
    parameter_meta_data: &mut HashMap<String, ParameterMetaData>,
    data: ParameterMetaData,
) {
    if let Some(ref first) = data.first {
        if let First::Value(ref field) = first {
            parameter_meta_data.insert(field.name.clone(), data);
        }
    }
}

/// Creates a [ParameterMetaData]
pub fn create_parameter_meta_data_single(
    treatment: TreatmentType,
    name: &str,
    value: String,
) -> ParameterMetaData {
    ParameterMetaData {
        treatment_type: treatment.into(),
        upper: None,
        first: Some(First::Value(Field {
            name: name.to_string(),
            value: Some(StringValue(value)),
        })),
    }
}

/// Function to convert a value from ATC to a RITE model value
pub fn atc_value_to_model_value(
    atc_value: Option<crate::com::atoss::atc::protobuf::field::Value>,
) -> Option<model::value::Value> {
    if let Some(value) = atc_value {
        match value {
            IntValue(v) => Some(Value::I32(v)),
            StringValue(v) => Some(Value::String(v)),
            DoubleValue(v) => Some(Value::F64(v)),
            BinaryValue(v) => Some(Value::Blob(v)),
            TimestampValue(v) => Some(Value::String(timestamp_to_string(v))),
            DurationValue(v) => Some(Value::I64(duration_to_i64(v))),
            IntsValue(v) => Some(Value::Collection(
                v.ints_value.into_iter().map(Value::I32).collect(),
            )),
            StringsValue(v) => Some(Value::Collection(
                v.strings_value.into_iter().map(Value::String).collect(),
            )),
            DoublesValue(v) => Some(Value::Collection(
                v.doubles_value.into_iter().map(Value::F64).collect(),
            )),
            TimestampsValue(v) => Some(Value::Collection(
                v.timestamps_value
                    .into_iter()
                    .filter_map(|ts| Some(Value::String(timestamp_to_string(ts))))
                    .collect(),
            )),
            DurationsValue(v) => Some(Value::Collection(
                v.durations_value
                    .into_iter()
                    .filter_map(|d| Some(Value::I64(duration_to_i64(d))))
                    .collect(),
            )),

            // TODO implement the lists
            ListValue(_v) => None,
            // TODO implement after [RIT-22 Support for Record values](https://ciao-systems.youtrack.cloud/issue/RIT-22)
            RecordValue(_v) => None,
        }
    } else {
        None
    }
}

/// Converts a [prost_types::Duration] to milliseconds
pub fn duration_to_i64(v: prost_types::Duration) -> i64 {
    v.seconds * 1000 + ((v.nanos as i64) / 1_000_000)
}

/// Converts a [prost_types::Timestamp] to a string
pub fn timestamp_to_string(v: prost_types::Timestamp) -> String {
    v.to_string()
}

#[cfg(test)]
mod tests;
