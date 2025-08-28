use model::import::Importer;
use model::{BoxedError, Initializable, field::add_field, record::Record, value::Value};

pub struct EnvImporter;
impl EnvImporter {
    pub fn new() -> Self {
        Self
    }
}

impl Initializable for EnvImporter {
    fn init(
        &mut self,
        _config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        // No configuration needed
        Ok(())
    }
}

impl Importer for EnvImporter {
    fn read(&mut self, handler: &mut dyn model::import::RecordHandler) -> Result<(), BoxedError> {
        // Iterate over all the environment variables.
        for (key, value) in std::env::vars() {
            let mut record = Record::new();
            add_field(record.fields_as_mut(), "name", Value::String(key));
            add_field(record.fields_as_mut(), "value", Value::String(value));

            handler.handle_record(&mut record)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use model::import::{Importer, handlers::CollectingRecordHandler};
    use model::{BoxedError, value::Value};
    use uuid::Uuid;

    use crate::importer::env::EnvImporter;

    #[test]
    fn test_importer() -> Result<(), BoxedError> {
        // Arrange
        let mut importer = EnvImporter::new();
        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);

        let expected_value = Uuid::new_v4();
        unsafe { std::env::set_var("TEST_VAR", expected_value.to_string()) };

        // Act
        importer.read(&mut handler)?;

        // Assert
        assert!(records.len() > 0);
        assert!(records.iter().any(|record| {
            if let Some(name_field) = record.field_by_name("name") {
                if let Value::String(env_name) = name_field.value() {
                    if env_name == "TEST_VAR" {
                        if let Some(value_field) = record.field_by_name("value") {
                            if let Value::String(env_value) = value_field.value() {
                                return env_value == expected_value.to_string();
                            }
                        }
                    }
                }
            }
            false
        }));

        // Annihilate
        unsafe { std::env::remove_var("TEST_VAR") };

        Ok(())
    }
}
