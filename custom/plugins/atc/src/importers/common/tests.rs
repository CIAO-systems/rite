use std::collections::HashMap;

use chrono::NaiveDate;
use model::{value::Value, xml::config::Configuration};
use prost_types::{Duration, Timestamp};

use crate::{
    com::atoss::atc::protobuf::{
        field::Value as ATC_Value,
        filter::{
            parameter_meta_data::{First, TreatmentType},
            ParameterMetaData,
        },
        DoubleCollection, DurationCollection, Field, IntCollection, List, Record, StringCollection,
        TimestampCollection,
    },
    importers::common::{
        add_fields_filter, atc_value_to_model_value, date_to_protobuf, duration_to_i64,
        parse_period, timestamp_to_string, CFG_FILTER_FIELDS,
    },
};

#[test]
fn test_add_fields_filter() {
    let mut parameter_meta_data = HashMap::new();
    let expected_fields = String::from("example_field1,example_field2");

    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_FIELDS, &expected_fields);

    add_fields_filter(&mut parameter_meta_data, &config).unwrap();

    let expected_field = Field {
        name: "fields".to_string(),
        value: Some(ATC_Value::StringValue(expected_fields.clone())),
    };

    let expected_meta_data = ParameterMetaData {
        treatment_type: TreatmentType::PttNone.into(),
        upper: None,
        first: Some(First::Value(expected_field)),
    };

    assert_eq!(parameter_meta_data.get("fields"), Some(&expected_meta_data));
}

#[test]
fn test_atc_value_to_model_value_string() {
    let atc_value: Option<ATC_Value> =
        Some(ATC_Value::StringValue(String::from("atc string value")));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(model_value, Value::String("atc string value".to_string()));
}

#[test]
fn test_atc_value_to_model_value_int() {
    let atc_value: Option<ATC_Value> = Some(ATC_Value::IntValue(73));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(model_value, Value::I32(73));
}

#[test]
fn test_atc_value_to_model_value_double() {
    let atc_value: Option<ATC_Value> = Some(ATC_Value::DoubleValue(73.42));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(model_value, Value::F64(73.42));
}

#[test]
fn test_atc_value_to_model_value_binary() {
    let data: Vec<u8> = vec![23, 156, 47, 7, 178];

    let atc_value: Option<ATC_Value> = Some(ATC_Value::BinaryValue(data.clone()));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(model_value, Value::Blob(data));
}

#[test]
fn test_atc_value_to_model_value_timestamp() {
    let data = Timestamp {
        seconds: 60,
        nanos: 0,
    };
    let atc_value: Option<ATC_Value> = Some(ATC_Value::TimestampValue(data));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(
        model_value,
        Value::String("1970-01-01T00:01:00Z".to_string())
    );
}

#[test]
fn test_atc_value_to_model_value_duration() {
    let data = Duration {
        seconds: 42,
        nanos: 73,
    };
    let atc_value: Option<ATC_Value> = Some(ATC_Value::DurationValue(data));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(model_value, Value::I64(42 * 1000 + (73 / 1_000_000)));
}

#[test]
fn test_atc_value_to_model_value_ints() {
    let data = IntCollection {
        ints_value: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 987, 654, 321],
    };

    let atc_value: Option<ATC_Value> = Some(ATC_Value::IntsValue(data.clone()));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(
        model_value,
        Value::Collection(data.ints_value.into_iter().map(Value::I32).collect())
    );
}

#[test]
fn test_atc_value_to_model_value_strings() {
    let data = StringCollection {
        strings_value: vec![
            String::from("This"),
            String::from("are"),
            String::from("not"),
            String::from("the"),
            String::from("drones"),
            String::from("you"),
            String::from("are"),
            String::from("looking"),
            String::from("for!"),
        ],
    };

    let atc_value: Option<ATC_Value> = Some(ATC_Value::StringsValue(data.clone()));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(
        model_value,
        Value::Collection(data.strings_value.into_iter().map(Value::String).collect())
    );
}

#[test]
fn test_atc_value_to_model_value_doubles() {
    let data = DoubleCollection {
        doubles_value: vec![
            1.9, 2.8, 3.7, 4.6, 5.5, 6.4, 7.3, 8.2, 9.1, 98.7, 6.54, 32.1,
        ],
    };

    let atc_value: Option<ATC_Value> = Some(ATC_Value::DoublesValue(data.clone()));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(
        model_value,
        Value::Collection(data.doubles_value.into_iter().map(Value::F64).collect())
    );
}

#[test]
fn test_atc_value_to_model_value_timestamps() {
    let data = TimestampCollection {
        timestamps_value: vec![
            Timestamp {
                seconds: 10,
                nanos: 0,
            },
            Timestamp {
                seconds: 9,
                nanos: 0,
            },
            Timestamp {
                seconds: 8,
                nanos: 0,
            },
            Timestamp {
                seconds: 7,
                nanos: 0,
            },
            Timestamp {
                seconds: 6,
                nanos: 0,
            },
            Timestamp {
                seconds: 5,
                nanos: 0,
            },
            Timestamp {
                seconds: 4,
                nanos: 0,
            },
            Timestamp {
                seconds: 3,
                nanos: 0,
            },
            Timestamp {
                seconds: 2,
                nanos: 0,
            },
            Timestamp {
                seconds: 1,
                nanos: 0,
            },
            Timestamp {
                seconds: 0,
                nanos: 0,
            },
        ],
    };

    let atc_value: Option<ATC_Value> = Some(ATC_Value::TimestampsValue(data.clone()));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(
        model_value,
        Value::Collection(
            data.timestamps_value
                .into_iter()
                .filter_map(|ts| Some(Value::String(timestamp_to_string(ts))))
                .collect(),
        )
    );
}

#[test]
fn test_atc_value_to_model_value_durations() {
    let data = DurationCollection {
        durations_value: vec![
            Duration {
                seconds: 10,
                nanos: 0,
            },
            Duration {
                seconds: 9,
                nanos: 0,
            },
            Duration {
                seconds: 8,
                nanos: 0,
            },
            Duration {
                seconds: 7,
                nanos: 0,
            },
            Duration {
                seconds: 6,
                nanos: 0,
            },
            Duration {
                seconds: 5,
                nanos: 0,
            },
            Duration {
                seconds: 4,
                nanos: 0,
            },
            Duration {
                seconds: 3,
                nanos: 0,
            },
            Duration {
                seconds: 2,
                nanos: 0,
            },
            Duration {
                seconds: 1,
                nanos: 0,
            },
            Duration {
                seconds: 0,
                nanos: 0,
            },
        ],
    };

    let atc_value: Option<ATC_Value> = Some(ATC_Value::DurationsValue(data.clone()));
    let model_value = atc_value_to_model_value(atc_value).unwrap();
    assert_eq!(
        model_value,
        Value::Collection(
            data.durations_value
                .into_iter()
                .filter_map(|d| Some(Value::I64(duration_to_i64(d))))
                .collect(),
        )
    );
}

#[test]
fn test_parse_period_full() {
    let (start, end) = parse_period("2025-01-01:2025-12-31");
    assert_eq!(start, NaiveDate::from_ymd_opt(2025, 1, 1));
    assert_eq!(end, NaiveDate::from_ymd_opt(2025, 12, 31));
}

#[test]
fn test_parse_period_start() {
    let (start, end) = parse_period("2025-01-01:");
    assert_eq!(start, NaiveDate::from_ymd_opt(2025, 1, 1));
    assert_eq!(end, None);
}

#[test]
fn test_parse_period_end() {
    let (start, end) = parse_period(":2025-12-31");
    assert_eq!(start, None);
    assert_eq!(end, NaiveDate::from_ymd_opt(2025, 12, 31));
}

#[test]
fn test_parse_period_empty() {
    let (start, end) = parse_period(":");
    assert_eq!(start, None);
    assert_eq!(end, None);
}

#[test]
fn test_parse_period_invalid() {
    let (start, end) = parse_period("this is not a date or a period:nor is this");
    assert_eq!(start, None);
    assert_eq!(end, None);
}

/// Test case: Valid date conversion (normal date)
#[test]
fn test_date_to_protobuf_valid_date() {
    // Arrange
    let naive_date = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();
    let expected_seconds = NaiveDate::from_ymd_opt(2023, 1, 15)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    // Act
    let result = date_to_protobuf(&naive_date);

    // Assert
    assert!(result.is_ok());
    let timestamp = result.unwrap();
    assert_eq!(timestamp.seconds, expected_seconds);
    assert_eq!(timestamp.nanos, 0);
}

/// Test case: Valid date conversion (epoch date)
#[test]
fn test_date_to_protobuf_epoch_date() {
    let naive_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let expected_seconds = NaiveDate::from_ymd_opt(1970, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let result = date_to_protobuf(&naive_date);

    assert!(result.is_ok());
    let timestamp = result.unwrap();
    assert_eq!(timestamp.seconds, expected_seconds);
    assert_eq!(timestamp.nanos, 0);
}

/// Test case: Valid date conversion (date in the past)
#[test]
fn test_date_to_protobuf_past_date() {
    let naive_date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let expected_seconds = NaiveDate::from_ymd_opt(1999, 12, 31)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let result = date_to_protobuf(&naive_date);

    assert!(result.is_ok());
    let timestamp = result.unwrap();
    assert_eq!(timestamp.seconds, expected_seconds);
    assert_eq!(timestamp.nanos, 0);
}

/// Test case: Valid date conversion (date in the future)
#[test]
fn test_date_to_protobuf_future_date() {
    let naive_date = NaiveDate::from_ymd_opt(2050, 6, 1).unwrap();
    let expected_seconds = NaiveDate::from_ymd_opt(2050, 6, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let result = date_to_protobuf(&naive_date);

    assert!(result.is_ok());
    let timestamp = result.unwrap();
    assert_eq!(timestamp.seconds, expected_seconds);
    assert_eq!(timestamp.nanos, 0);
}

#[test]
fn test_list_value() {
    let list = List { list: vec![] };

    let result = atc_value_to_model_value(Some(ATC_Value::ListValue(list)));
    assert!(result.is_none()); // until implemented
}

#[test]
fn test_record_value() {
    let record = Record {
        field: HashMap::new(),
    };
    let result = atc_value_to_model_value(Some(ATC_Value::RecordValue(record)));
    assert!(result.is_none()); // until implemented
}

#[test]
fn test_none() {
    let result = atc_value_to_model_value(None);
    assert!(result.is_none()); 
}
