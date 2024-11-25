use std::error::Error;

use model::record::Record;

pub trait Transformer {
    /// Initializes the transformer
    fn init(&mut self) -> Result<(), Box<dyn Error>>;

    fn process(&self, record: &Record) -> Result<Record, Box<dyn Error>>;
}

pub mod builtin;

