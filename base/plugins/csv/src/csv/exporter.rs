use export::Exporter;
use std::fs::OpenOptions;
use std::io::Write;

use super::CSV;

impl Exporter for CSV {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref path) = self.filename {
            let mut options = OpenOptions::new();
            options.write(true).create(true);

            if !self.export_header_written && self.export_override {
                options.truncate(true);
            } else {
                options.append(true);
            };

            let mut file = options.open(path)?;

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
    use std::{fs, path::Path};

    use export::Exporter;
    use model::{
        field::add_field, record::Record, value::Value, xml::config::Configuration, Initializable,
    };

    use crate::csv::{CFG_EXPORT_OVERWRITE, CFG_FILENAME, CSV};

    fn generate_temp_name() -> String {
        let path_buf: std::path::PathBuf = (&tempfile::Builder::new()
            .suffix(".csv")
            .tempfile()
            .unwrap()
            .into_temp_path())
            .into();

        path_buf.to_string_lossy().into_owned()
    }

    #[test]
    fn test_exporter() -> Result<(), Box<dyn std::error::Error>> {
        let mut csv = CSV::new();
        let mut config = Configuration::new();
        let output_file_name = generate_temp_name();
        config.insert_str(CFG_FILENAME, &output_file_name);

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

        let contents = fs::read_to_string(&output_file_name)?;
        fs::remove_file(&output_file_name)?;

        let expected = "field1,field2\nvalue1,value2\nvalue1,value2\n";
        assert_eq!(contents, expected);
        Ok(())
    }

    #[test]
    fn test_rit_47() -> Result<(), Box<dyn std::error::Error>> {
        let mut csv = CSV::new();
        let mut config = Configuration::new();

        let output_file_name = generate_temp_name();
        config.insert_str(CFG_FILENAME, &output_file_name);
        config.insert_str(CFG_EXPORT_OVERWRITE, "true");

        csv.init(Some(config))?;

        // Make sure, the file does not exist
        if Path::new(&output_file_name).exists() {
            fs::remove_file(&output_file_name)?;
        }

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

        let expected = "field1,field2\nvalue1,value2\nvalue1,value2\n";
        let contents = fs::read_to_string(&output_file_name)?;
        fs::remove_file(&output_file_name)?;
        assert_eq!(contents, expected);

        Ok(())
    }
}
