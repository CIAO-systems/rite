//! RITE exporter trait
use model::{record::Record, Initializable};
use std::error::Error;

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
