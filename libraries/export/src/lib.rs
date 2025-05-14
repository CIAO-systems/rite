//! RITE exporter trait
use model::{record::Record, BoxedError, Initializable};

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Start = 0,
    End = 1,
}

impl Default for Signal {
    fn default() -> Self {
        Signal::Start
    }
}
/// The interface for RITE exporter components
pub trait Exporter: Initializable {
    /// Takes a [Record] and writes it
    ///
    fn write(&mut self, record: &Record) -> Result<(), BoxedError>;

    /// Event signaling function
    ///
    /// Exporters can utilize this, to collect records and process them at the
    /// end
    fn event(&mut self, #[allow(unused_variables)] signal: Signal) -> Result<(), BoxedError> {
        Ok(())
    }
}
