use std::io::Read;

use super::ConsoleExporter;
use crate::Exporter;
use model::{field::Field, record::Record};

#[test]
fn test_write() {
    // Redirect stdout
    let mut buf = gag::BufferRedirect::stdout().unwrap();

    let mut exporter = ConsoleExporter::new();

    let mut record = Record::new();
    let f1 = Field::new_string("string".to_string(), "value".to_string());
    let f2 = Field::new_i32("int".to_string(), 73);
    record.fields_as_mut().push(f1);
    record.fields_as_mut().push(f2);
    let _ = exporter.write(&record);

    // Read the output into a string
    let mut output = String::new();
    let _ = buf.read_to_string(&mut output);

    assert_eq!("string=value,int=73\n", output);
}
