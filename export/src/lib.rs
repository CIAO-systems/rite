//! RITE exporter trait
use std::error::Error;

use model::{record::Record, Initializable};

/// The interface for RITE exporter components
pub trait Exporter: Initializable {
    /// Takes a [Record] and writes it
    /// 
    /// # Example
    /// An example implementation:
    /// ```
    /// fn write(&mut self, record: &model::record::Record) -> Result<(), Box<dyn std::error::Error>> {
    ///     let fields = record.fields();
    /// 
    ///     for (i, field) in fields.iter().enumerate() {
    ///         if i > 0 {
    ///             print!(",")?;
    ///         }
    ///         print!("{}={}", field.name(), field.value())?;
    ///     }
    ///     println!(&mut self.writer)?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    fn write(&mut self, record: &Record) -> Result<(), Box<dyn Error>>;
}
