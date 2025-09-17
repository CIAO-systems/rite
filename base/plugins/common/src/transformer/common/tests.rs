use chrono::{Local, TimeZone, Utc};
use model::{
    Initializable, field::add_field, record::Record, transform::Transformer, value::Value,
    xml::config::Configuration,
};
use uuid::Uuid;

use crate::transformer::common::CommonTransformer;

#[test]
fn test_transformer() {
    let mut transformer = CommonTransformer::new();
    let mut config = Configuration::new();
    config.insert_str("add_field", "field:uuid");
    config.insert_str("rename_field", "old:new");
    config.insert_str("ignore_field", "field");
    config.insert_str("include_field", "field");
    config.insert_str("format_field", "field:format");
    assert!(transformer.init(Some(config)).is_ok());
    assert_eq!(transformer.adders.len(), 1);
    assert_eq!(transformer.renamers.len(), 1);
    assert_eq!(transformer.ignorers.len(), 1);
    assert_eq!(transformer.includers.len(), 1);
    assert_eq!(transformer.formatters.len(), 1);
}

#[test]
fn test_transformer_process() {
    let mut transformer = CommonTransformer::new();
    let mut config = Configuration::new();
    config.insert_str("add_field", "add:uuid");
    config.insert_str("rename_field", "old:new");
    config.insert_str("ignore_field", "ignore");
    config.insert_str("format_field", "format:unixtime");
    config.insert_str("include_field", "old,format,included");
    assert!(transformer.init(Some(config)).is_ok());

    let mut record = Record::new();
    add_field(record.fields_as_mut(), "old", "old field".into());
    add_field(record.fields_as_mut(), "ignored", "does not matter".into());
    add_field(record.fields_as_mut(), "included", "does matter".into());
    let today = Local::now().date_naive();
    add_field(record.fields_as_mut(), "format", today.into());

    let transformed = transformer.process(&record).ok().unwrap();
    println!("{:?}", transformed);

    // Check add
    assert!(
        transformed
            .field_by_name("add")
            .is_some_and(|f| Uuid::parse_str(&f.value().to_string()).is_ok())
    );

    // Check rename
    assert!(transformed.field_by_name("old").is_none());
    assert!(
        transformed
            .field_by_name("new")
            .is_some_and(|f| f.value().eq(&Value::String("old field".to_string())))
    );

    // Check ignore
    assert!(transformed.field_by_name("ignored").is_none());

    // Check inlcude
    assert!(
        transformed
            .field_by_name("included")
            .is_some_and(|f| f.value().eq(&Value::String("does matter".to_string())))
    );

    // Check format
    let millis = Utc
        .from_utc_datetime(&today.and_hms_opt(0, 0, 0).unwrap())
        .timestamp_millis();
    let field = transformed.field_by_name("format");
    assert!(transformed.field_by_name("format").is_some());
    let value = field.unwrap().value_as_ref();
    if let Value::I64(value) = value {
        assert_eq!(value, &millis);
    }
}
