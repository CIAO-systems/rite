use std::collections::HashMap;

use chrono::{Local, NaiveDate, TimeZone, Utc};
use model::{value::Value, BoxedError};
use prost_types::Timestamp;

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

/// Parses the a string of two dates separated by `:`.
/// Dates can be empty and use the format `"%Y-%m-%d"`.
/// Returns a tuple of start and end
pub fn parse_period(period: &str) -> (Option<NaiveDate>, Option<NaiveDate>) {
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

/// Converts a [NaiveDate] to a protobuf [Timestamp]
pub fn date_to_protobuf(naive_date: &NaiveDate) -> Result<Timestamp, BoxedError> {
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
        unreachable!()
    }
}

/// Converts an optional protobuf [Timestamp] to a [NaiveDate]
/// If the `timestamp` is None, the current date will be returned.
pub fn protobuf_to_date(timestamp: Option<Timestamp>) -> Result<NaiveDate, BoxedError> {
    match timestamp {
        Some(ts) => {
            // Convert the timestamp to a NaiveDate
            let datetime_utc = Utc
                .timestamp_opt(ts.seconds, ts.nanos as u32)
                .single()
                .ok_or_else(|| {
                    format!(
                        "Invalid timestamp value: seconds={}, nanos={}",
                        ts.seconds, ts.nanos
                    )
                })?;

            Ok(datetime_utc.date_naive())
        }
        None =>
        // If the timestamp is None, return the current date in the **local timezone**.
        {
            Ok(Local::now().date_naive())
        }
    }
}

#[cfg(test)]
mod tests;
