use model::{record::Record, Initializable};
use std::error::Error;

pub trait Exporter: Initializable {
    /// Takes a [Record] and writes it
    fn write(&mut self, record: &Record) -> Result<(), Box<dyn Error>>;
}
