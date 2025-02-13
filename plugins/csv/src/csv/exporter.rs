use export::Exporter;
use std::fs::{self, OpenOptions};
use std::io::Write;

use super::CSV;

impl Exporter for CSV {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref path) = self.filename {
            if !self.export_header_written && self.export_override {
                // If file should be overwritten, and this is the first time
                // the write function is called, delete the file
                fs::remove_file(path)?;
            }

            // Open the file in append mode
            let mut file = OpenOptions::new()
                .append(true)
                .create(true) // This will create the file if it doesn't exist
                .open(path)?;

            if !self.export_header_written {
                let mut header = String::new();
                for field in record.fields() {
                    if !header.is_empty() {
                        header.push(',');
                    }
                    header.push_str(field.name());
                }
                writeln!(file, "{}", header)?;
                self.export_header_written = true;
            }

            let mut line = String::new();
            for field in record.fields() {
                if !line.is_empty() {
                    line.push(',');
                }
                line.push_str(&field.value().to_string());
            }
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use export::Exporter;
    use model::{
        field::add_field, record::Record, value::Value, xml::config::Configuration,
        Initializable,
    };

    use crate::csv::{CFG_FILENAME, CSV};

    #[test]
    fn test_exporter() -> Result<(), Box<dyn std::error::Error>> {
        let mut csv = CSV::new();
        let mut config = Configuration::new();
        let output_file = "/tmp/example.outout.csv";
        config.insert_str(CFG_FILENAME, output_file);

        csv.init(Some(config))?;

        let mut record = Record::new();
        for i in 1..=2 {
            add_field(
                record.fields_as_mut(),
                &format!("field{i}"),
                Value::String(format!("value{i}")),
            );
        }
        for _ in 1..=2 {
            csv.write(&record)?;
        }

        let contents = fs::read_to_string(output_file)?;
        fs::remove_file(output_file)?;

        let expected = format!("field1,field2\nvalue1,value2\nvalue1,value2\n");
        assert_eq!(contents, expected);
        Ok(())
    }
}
