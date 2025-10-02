//! Module for the console exporter
//!
use std::io::{self, Write};

use model::export::Exporter;
use model::{Initializable, record::Record, xml};

/// An [Exporter] that writes the [Record] to a [Write]
pub struct ConsoleExporter {
    writer: Box<dyn Write>,
    prefix: Option<String>,
    postfix: Option<String>,
    separator: Option<String>,
    field_prefix: Option<String>,
}

impl ConsoleExporter {
    /// Creates a new [ConsoleExporter] that writes the [Record] to [io::stdout]
    pub fn new() -> Self {
        let stdout: Box<dyn Write> = Box::new(io::stdout()); // Get a handle to stdout
        ConsoleExporter::new_writer(stdout)
    }

    /// Creates a new [ConsoleExporter] that writes the [Record] to the given [Write]
    /// # Arguments
    /// * `writer` - A boxed [Write] instance that the [Record]s will be written to
    pub fn new_writer(writer: Box<dyn Write>) -> Self {
        ConsoleExporter {
            writer,
            prefix: None,
            postfix: None,
            separator: None,
            field_prefix: None,
        }
    }
}

/// [Exporter] implementation for the [ConsoleExporter]
impl Exporter for ConsoleExporter {
    /// Writes the given [Record] to the configured [Write]
    /// If there is a `prefix` configured, the value of the configuration variable
    /// will be written before the contents of the record.
    /// The [Record] will be written in one line as a comma separated list of
    /// fields in the format: `field`=`value`
    ///
    /// # Arguments
    /// * `record` -  A [Record] that will be written to the [Write]
    fn write(&mut self, record: &Record) -> Result<(), Box<dyn std::error::Error>> {
        let fields = record.fields();

        let separator = if let Some(ref separator) = self.separator {
            separator.as_str()
        } else {
            ", "
        };

        let field_prefix = if let Some(ref field_prefix) = self.field_prefix {
            field_prefix.as_str()
        } else {
            ""
        };

        if let Some(ref prefix) = self.prefix {
            writeln!(&mut self.writer, "{prefix}")?;
        }

        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                write!(&mut self.writer, "{separator}")?;
            }
            write!(
                &mut self.writer,
                "{field_prefix}{}={}",
                field.name(),
                field.value()
            )?;
        }
        writeln!(&mut self.writer)?;
        if let Some(ref postfix) = self.postfix {
            writeln!(&mut self.writer, "{postfix}")?;
        }

        Ok(())
    }
}

const CFG_PREFIX: &str = "prefix";
const CFG_POSTFIX: &str = "postfix";
const CFG_SEPARATOR: &str = "separator";
const CFG_FIELD_PREFIX: &str = "field-prefix";

impl Initializable for ConsoleExporter {
    /// Initializes the exporter from the configuration variables (it uses the key/values)
    /// # Configuration keys
    /// * `prefix` - A text that should be printed before the [Record]s field values
    fn init(
        &mut self,
        config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            if let Some(prefix) = config.get(CFG_PREFIX) {
                self.prefix = Some(String::from(prefix));
            }
            if let Some(postfix) = config.get(CFG_POSTFIX) {
                self.postfix = Some(String::from(postfix));
            }
            if let Some(separator) = config.get(CFG_SEPARATOR) {
                self.separator = Some(String::from(separator));
            }
            if let Some(field_prefix) = config.get(CFG_FIELD_PREFIX) {
                self.field_prefix = Some(String::from(field_prefix));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
