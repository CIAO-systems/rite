use model::import::{Importer, RecordHandler};
use model::xml::file::load_and_substitute_from_env;
use model::{
    Initializable,
    xml::{self},
};

use crate::importer::config::RiteMariaDBImport;

mod config;

pub struct MariaDBImporter {
    mariadb: Option<RiteMariaDBImport>,
}

impl MariaDBImporter {
    pub fn new() -> Self {
        MariaDBImporter { mariadb: None }
    }
}

impl Initializable for MariaDBImporter {
    fn init(
        &mut self,
        config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let mariadb: config::RiteMariaDBImport =
                            match serde_xml_rs::from_str(&xml_contents) {
                                Ok(x) => x,
                                Err(e) => {
                                    return Err(format!(
                                        "Cannot parse contents from {}: {}",
                                        xml, e
                                    )
                                    .into())
                                }
                            };
                        self.mariadb = Some(mariadb);
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Ok(())
    }
}

impl Importer for MariaDBImporter {
    fn read(&mut self, _handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        // TODO implement
        Ok(())
    }
}
