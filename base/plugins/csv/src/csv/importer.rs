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

    static EXAMPLE_CSV: &str = "../../data/test/csv/example.csv";

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
