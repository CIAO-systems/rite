use model::record::Record;
use std::error::Error;

pub trait Exporter {
    /// Initializes the importer
    fn init(&mut self) -> Result<(), Box<dyn Error>>;

    fn write(&mut self, record: &Record) -> Result<(), Box<dyn Error>>;
}

pub mod builtin;
