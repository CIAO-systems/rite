use model::Initializable;

const CFG_FILENAME: &str = "filename";
const CFG_EXPORT_OVERWRITE: &str = "overwrite";

pub struct CSV {
    filename: Option<String>,
    export_header_written: bool,
    export_override: bool,
}
impl CSV {
    pub(crate) fn new() -> Self {
        CSV {
            filename: None,
            export_header_written: false,
            export_override: false,
        }
    }
}

impl Initializable for CSV {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            self.filename = config.get(CFG_FILENAME);
            self.export_override = match config.get(CFG_EXPORT_OVERWRITE) {
                Some(value) => value.parse::<bool>()?,
                None => false,
            };
        }

        Ok(())
    }
}
mod exporter {
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
}

mod importer {
    use csv::ReaderBuilder;
    use import::Importer;
    use model::{field::add_field, record::Record, value::Value};

    use super::{CFG_FILENAME, CSV};

    impl Importer for CSV {
        fn read(
            &mut self,
            handler: &mut dyn import::RecordHandler,
        ) -> Result<(), Box<dyn std::error::Error>> {
            if let Some(ref path) = self.filename {
                let mut rdr = ReaderBuilder::new().from_path(path)?;
                let headers = rdr.headers().cloned()?;
                for result in rdr.records() {
                    let record = result?;
                    let mut record = convert(&headers, &record);
                    handler.handle_record(&mut record)?;
                }
                Ok(())
            } else {
                Err(format!("Missing configuration: {}", CFG_FILENAME).into())
            }
        }
    }

    fn convert(headers: &csv::StringRecord, record: &csv::StringRecord) -> model::record::Record {
        let mut result = Record::new();
        let fields = result.fields_as_mut();

        for (index, value) in record.iter().enumerate() {
            if let Some(header) = headers.get(index) {
                add_field(fields, header, Value::String(value.to_string()));
            }
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use import::{handlers::CollectingRecordHandler, Importer};
        use model::{value::Value, xml::config::Configuration, Initializable};

        use super::{CFG_FILENAME, CSV};

        #[test]
        fn test_importer() -> Result<(), Box<dyn std::error::Error>> {
            let mut importer = CSV::new();
            let mut config = Configuration::new();
            config.insert_str(CFG_FILENAME, "../../data/csv/example.csv");
            importer.init(Some(config))?;

            let mut records = Vec::new();
            let mut handler = CollectingRecordHandler::new(&mut records);
            importer.read(&mut handler)?;

            assert_eq!(records.len(), 5);

            // ProjectID,ProjectName,StartDate,EndDate,Status
            // 1,Website Redesign,2023-01-15,2023-03-30,Completed
            // 2,Mobile App Development,2023-02-01,2023-06-15,In Progress
            // 3,Data Migration,2023-03-10,2023-05-20,Completed
            // 4,Marketing Campaign,2023-04-01,2023-07-31,Planning
            // 5,Software Update,2023-05-20,2023-08-10,In Progress
            assert_eq!(
                records[0].field_by_name("ProjectID").unwrap().value(),
                Value::String("1".to_string())
            );
            assert_eq!(
                records[0].field_by_name("ProjectName").unwrap().value(),
                Value::String("Website Redesign".to_string())
            );
            assert_eq!(
                records[0].field_by_name("StartDate").unwrap().value(),
                Value::String("2023-01-15".to_string())
            );
            assert_eq!(
                records[0].field_by_name("EndDate").unwrap().value(),
                Value::String("2023-03-30".to_string())
            );
            assert_eq!(
                records[0].field_by_name("Status").unwrap().value(),
                Value::String("Completed".to_string())
            );
            Ok(())
        }
    }
}
