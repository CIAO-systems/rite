use model::{record::Record, BoxedError, Initializable};

pub trait RecordHandler {
    fn handle_record(&mut self, record: &mut Record) -> Result<(), BoxedError>;
}

pub trait Importer: Initializable {
    /// Reads all from the import source and calls the `callback` for each record
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>>;

    /// Resets the importer, so that `next` and `read` start from the beginning again
    /// With default implementation, since most importers will not support it
    fn reset(&mut self) -> Result<(), BoxedError> {
        Ok(())
    }
}

/// Common record handlers
pub mod handlers;

#[cfg(test)]
mod tests {
    use model::{field::Field, record::Record, value::Value, xml, BoxedError, Initializable};

    use crate::{handlers::CollectingRecordHandler, Importer, RecordHandler};

    // Demo importer that simulates reading with delays
    pub struct DemoImporter;

    impl Importer for DemoImporter {
        fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), BoxedError> {
            let mut record = Record::new();
            for i in 1..=5 {
                record
                    .fields_as_mut()
                    .push(Field::new_value("index", Value::U16(i)));

                handler.handle_record(&mut record)?;
            }

            Ok(())
        }
    }

    impl Initializable for DemoImporter {
        fn init(&mut self, _config: Option<xml::config::Configuration>) -> Result<(), BoxedError> {
            Ok(())
        }
    }

    #[test]
    fn test() -> Result<(), BoxedError> {
        let mut importer = DemoImporter;
        let mut records = Vec::<Record>::new();

        let mut handler = CollectingRecordHandler::new(&mut records);
        importer.read(&mut handler)?;

        assert_eq!(5, records.len());
        records.into_iter().for_each(|r| {
            println!("{:?}", r);
        });
        Ok(())
    }

    #[test]
    fn test_reset() -> Result<(), BoxedError> {
        let mut importer = DemoImporter;
        assert!(importer.reset().is_ok());

        Ok(())
    }
}
