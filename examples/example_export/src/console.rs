use std::io::{self, Write};

use export::Exporter;


pub struct ConsoleExporter {
    writer: Box<dyn Write>,
}

impl ConsoleExporter {
    pub fn new() -> Self {
        let stdout: Box<dyn Write> = Box::new(io::stdout()); // Get a handle to stdout
        ConsoleExporter::new_writer(stdout)
    }

    pub fn new_writer(writer: Box<dyn Write>) -> Self {
        ConsoleExporter { writer }
    }
}

impl Exporter for ConsoleExporter {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn write(&mut self, record: &model::record::Record) -> Result<(), Box<dyn std::error::Error>> {
        let fields = record.fields();

        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                write!(&mut self.writer, ",")?;
            }
            write!(&mut self.writer, "{}={}", field.name(), field.value())?;
        }
        writeln!(&mut self.writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
