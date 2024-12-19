//! RITE exporter trait
use std::error::Error;

use model::{record::Record, Initializable};

/// The interface for RITE exporter components
pub trait Exporter: Initializable {
    /// Takes a [Record] and writes it
    /// 
    fn write(&mut self, record: &Record) -> Result<(), Box<dyn Error>>;
}
