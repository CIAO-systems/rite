//! RITE exporter trait
use model::{record::Record, BoxedError, Initializable};

#[derive(Clone)]
pub enum Signal {
    Start,
    End,
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
