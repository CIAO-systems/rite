use csv::ReaderBuilder;
use model::import::Importer;
use model::{field::add_field, record::Record, value::Value};

use super::{CFG_FILENAME, CSV};

impl Importer for CSV {
    fn read(
        &mut self,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref path) = self.filename {
            let mut reader_builder = ReaderBuilder::new();
            if let Some(delimiter) = self.delimiter {
                log::info!("Using delimiter {delimiter}");
                reader_builder.delimiter(delimiter);
            }
            let mut rdr = reader_builder.from_path(path)?;
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
    use model::import::{handlers::CollectingRecordHandler, Importer};
    use model::{value::Value, xml::config::Configuration, Initializable};

    use crate::csv::CFG_DELIMITER;

    use super::{CFG_FILENAME, CSV};

    static EXAMPLE_CSV: &str = "../../data/test/csv/example.csv";
    static DELIMITER_TEST_CSV: &str = "../../data/test/csv/delimiter_test.csv";

    fn assert_field(
        records: &Vec<model::record::Record>,
        index: usize,
        field_name: &str,
        expected_value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let field = records[index].field_by_name(field_name);
        if field.is_none() {
            return Err(format!("Field {field_name} not found").into());
        }

        assert_eq!(
            records[index].field_by_name(field_name).unwrap().value(),
            Value::String(expected_value.to_string())
        );
        Ok(())
    }

    #[test]
    fn test_importer() -> Result<(), Box<dyn std::error::Error>> {
        let mut importer = CSV::new();
        let mut config = Configuration::new();
        config.insert_str(CFG_FILENAME, EXAMPLE_CSV);
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
        assert_field(&records, 0, "ProjectID", "1")?;
        assert_field(&records, 0, "ProjectName", "Website Redesign")?;
        assert_field(&records, 0, "StartDate", "2023-01-15")?;
        assert_field(&records, 0, "EndDate", "2023-03-30")?;
        assert_field(&records, 0, "Status", "Completed")?;

        assert_field(&records, 4, "ProjectID", "5")?;
        assert_field(&records, 4, "ProjectName", "Software Update")?;
        assert_field(&records, 4, "StartDate", "2023-05-20")?;
        assert_field(&records, 4, "EndDate", "2023-08-10")?;
        assert_field(&records, 4, "Status", "In Progress")?;

        Ok(())
    }

    #[test]
    fn test_importer_delimiter() -> Result<(), Box<dyn std::error::Error>> {
        let mut importer = CSV::new();
        let mut config = Configuration::new();
        config.insert_str(CFG_FILENAME, DELIMITER_TEST_CSV);
        config.insert_str(CFG_DELIMITER, ";");
        importer.init(Some(config))?;

        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        importer.read(&mut handler)?;

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].fields().len(), 5);

        // ProjectID;ProjectName;StartDate;EndDate;Status
        // 1;Website Redesign;2023-01-15;2023-03-30;Completed
        assert_field(&records, 0, "ProjectID", "1")?;
        assert_field(&records, 0, "ProjectName", "Website Redesign")?;
        assert_field(&records, 0, "StartDate", "2023-01-15")?;
        assert_field(&records, 0, "EndDate", "2023-03-30")?;
        assert_field(&records, 0, "Status", "Completed")?;
        Ok(())
    }
}
