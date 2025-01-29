//! RITE exporter trait
use model::{record::Record, BoxedError, Initializable};

/// The interface for RITE exporter components
pub trait Exporter: Initializable {
    /// Takes a [Record] and writes it
    ///
    fn write(&mut self, record: &Record) -> Result<(), BoxedError>;
}
