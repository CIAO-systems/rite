use std::io::Read;

use super::ConsoleExporter;
use model::export::Exporter;
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
