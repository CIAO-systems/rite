use config::RiteRandomImport;
use model::import::Importer;
use model::{
    field::add_field, record::Record, xml::file::load_and_substitute_from_env, Initializable,
};

mod config;
mod functions;

pub struct Faker {
    config: Option<model::xml::config::Configuration>,
    random: Option<RiteRandomImport>,
}

impl Faker {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            random: None,
        }
    }
}

impl Initializable for Faker {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;

        if let Some(ref config) = self.config {
            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let random: RiteRandomImport = match serde_xml_rs::from_str(&xml_contents) {
                            Ok(x) => x,
                            Err(e) => {
                                return Err(
                                    format!("Cannot parse contents from {}: {}", xml, e).into()
                                )
                            }
                        };
                        self.random = Some(random);
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }

        Ok(())
    }
}

impl Importer for Faker {
    fn read(
        &mut self,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref random) = self.random {
            for _ in 0..random.generator.number {
                let mut record = Record::new();
                let fields = record.fields_as_mut();

                for f in &random.generator.fields {
                    // Add field if not optional
                    if f.is_needed() {
                        if let Some(generator) = f.create_generator() {
                            add_field(fields, &f.name, generator.generate());
                        }
                    }
                }

                handler.handle_record(&mut record)?;
            }

            Ok(())
        } else {
            Err("No configuration found".into())
        }
    }
}

#[cfg(test)]
mod tests {

    use model::import::{handlers::CollectingRecordHandler, Importer};
    use model::{xml::config::Configuration, Initializable};

    use super::Faker;

    static CONFIGURATION: &str = "../../data/test/faker/fake-records-generator.xml";
    #[test]
    fn test_importer() -> Result<(), Box<dyn std::error::Error>> {
        let mut faker = Faker::new();
        let config = Configuration::with_xml(CONFIGURATION);
        faker.init(Some(config))?;
        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        faker.read(&mut handler)?;

        assert_eq!(records.len(), 10);

        let first = records.first().unwrap();
        println!("{:?}", first);
        assert!(first.field_by_name("timestamp.timeUtc").is_some());
        assert!(first.field_by_name("timestamp.timeZone").is_some());

        // let has_user = first.field_by_name("identity.userId").is_some();
        // let has_badge = first.field_by_name("identity.badgeId").is_some();
        // assert!(has_user || has_badge);

        assert!(first.field_by_name("deviceId").is_some());
        assert!(first.field_by_name("timeTypeId").is_some());
        assert!(first.field_by_name("projectId").is_some());
        assert!(first.field_by_name("projectTaskId").is_some());
        assert!(first.field_by_name("costcenterId").is_some());

        Ok(())
    }
}
