use model::{export, field::add_field, import::Importer, record::Record, transform, Initializable};

use super::*;

struct TestStructure {
    read_error: bool,
    transform_error: bool,
}
impl TestStructure {
    fn new() -> Self {
        Self { read_error: false, transform_error: false }
    }

    fn with_read_error() -> Self {
        Self { read_error: true, transform_error: false }
    }

    fn with_transform_error() -> Self {
        Self { read_error: false, transform_error: true }
    }

}

impl Initializable for TestStructure {
    fn init(
        &mut self,
        _config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        Ok(())
    }
}

impl Importer for TestStructure {
    fn read(
        &mut self,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.read_error {
            return Err("simulated read error".into());
        }
        let mut record = Record::new();
        handler.handle_record(&mut record)?;
        Ok(())
    }
}

impl transform::Transformer for TestStructure {
    fn process(&self, record: &Record) -> Result<Record, model::BoxedError> {
        if self.transform_error {
            return Err("simulated transform error".into());
        }
        let mut result = record.clone();
        add_field(result.fields_as_mut(), "transformed", true.into());
        Ok(result)
    }
}

impl export::Exporter for TestStructure {
    fn write(&mut self, _record: &Record) -> Result<(), model::BoxedError> {
        Ok(())
    }
}

#[test]
fn test_import() {
    let mut importer: Box<dyn Importer> = Box::new(TestStructure::new());
    let mut import = super::Importer::new(&mut importer);

    let mut transformers: Vec<Box<dyn transform::Transformer>> = Vec::new();
    transformers.push(Box::new(TestStructure::new()));

    let t = Some(Transformer::new(&transformers));

    let mut exporters: Vec<Box<dyn export::Exporter>> = Vec::new();
    exporters.push(Box::new(TestStructure::new()));
    let mut e = Some(Exporter::new(&mut exporters));
    let result = import.import(&t, &mut e);
    assert!(result.is_ok());
}

#[test]
fn test_import_read_error() {
    let mut importer: Box<dyn Importer> = Box::new(TestStructure::with_read_error());
    let mut import = super::Importer::new(&mut importer);

    let mut transformers: Vec<Box<dyn transform::Transformer>> = Vec::new();
    transformers.push(Box::new(TestStructure::new()));

    let t = Some(Transformer::new(&transformers));

    let mut exporters: Vec<Box<dyn export::Exporter>> = Vec::new();
    exporters.push(Box::new(TestStructure::new()));
    let mut e = Some(Exporter::new(&mut exporters));
    let result = import.import(&t, &mut e);
    assert!(result.is_err_and(|e| e.to_string() == "simulated read error"));
}

#[test]
fn test_import_transform_error() {
    let mut importer: Box<dyn Importer> = Box::new(TestStructure::new());
    let mut import = super::Importer::new(&mut importer);

    let mut transformers: Vec<Box<dyn transform::Transformer>> = Vec::new();
    transformers.push(Box::new(TestStructure::with_transform_error()));

    let t = Some(Transformer::new(&transformers));

    let mut exporters: Vec<Box<dyn export::Exporter>> = Vec::new();
    exporters.push(Box::new(TestStructure::new()));
    let mut e = Some(Exporter::new(&mut exporters));
    let result = import.import(&t, &mut e);
    assert!(result.is_ok());
}
