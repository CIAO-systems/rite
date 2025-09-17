use std::io::Read;

use crate::exporter::console::{CFG_FIELD_PREFIX, CFG_POSTFIX, CFG_PREFIX, CFG_SEPARATOR};

use super::ConsoleExporter;
use model::Initializable;
use model::export::Exporter;
use model::field::add_field;
use model::xml::config::Configuration;
use model::{field::Field, record::Record};

#[test]
#[ignore = "stdout redirect does not work reliably"]
fn test_write() {
    // Redirect stdout
    let mut buf = gag::BufferRedirect::stdout().unwrap();

    let mut exporter = ConsoleExporter::new();

    let mut record = Record::new();
    let f1 = Field::new_value("string", model::value::Value::String("value".to_string()));
    let f2 = Field::new_value("int", model::value::Value::I32(73));
    record.fields_as_mut().push(f1);
    record.fields_as_mut().push(f2);
    let _ = exporter.write(&record);

    // Read the output into a string
    let mut output = String::new();
    let _ = buf.read_to_string(&mut output);

    assert_eq!("string=value, int=73\n", output);
}

#[test]
fn test_export() {
    let mut exporter = ConsoleExporter::new();
    let config = Configuration::new();
    let result = exporter.init(Some(config));
    assert!(result.is_ok());

    let record = Record::new();
    let result = exporter.write(&record);
    assert!(result.is_ok());

    let mut config = Configuration::new();
    config.insert_str(CFG_PREFIX, "prefix-value");
    config.insert_str(CFG_POSTFIX, "postfix-value");
    config.insert_str(CFG_FIELD_PREFIX, "field-prefix-value");
    config.insert_str(CFG_SEPARATOR, "separator-value");
    let result = exporter.init(Some(config));
    assert!(result.is_ok());
    assert_eq!(exporter.prefix.clone().unwrap().clone(), "prefix-value");
    assert_eq!(exporter.postfix.clone().unwrap().clone(), "postfix-value");
    assert_eq!(
        exporter.field_prefix.clone().unwrap().clone(),
        "field-prefix-value"
    );
    assert_eq!(
        exporter.separator.clone().unwrap().clone(),
        "separator-value"
    );

    let mut record = Record::new();
    add_field(record.fields_as_mut(), "f1", "value1".into());
    add_field(record.fields_as_mut(), "f2", "value2".into());
    let result = exporter.write(&record);
    assert!(result.is_ok());
}
