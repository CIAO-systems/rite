use model::import::{Importer, RecordHandler};
use model::{
    xml::{self},
    Initializable,
};

pub struct MariaDBImporter;

impl MariaDBImporter {
    pub fn new() -> Self {
        MariaDBImporter {}
    }
}

impl Initializable for MariaDBImporter {
    fn init(
        &mut self,
        _config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl Importer for MariaDBImporter {
    fn read(&mut self, _handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}