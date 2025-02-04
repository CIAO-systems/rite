use handlers::TransformAndExportRecordHandler;

use super::{Exporter, Transformer};

mod handlers;

pub struct Importer<'a> {
    importer: &'a mut Box<dyn import::Importer>,
}

impl<'a> Importer<'a> {
    pub fn new(importer: &'a mut Box<dyn import::Importer>) -> Self {
        Self { importer }
    }

    pub fn import(
        &'a mut self,
        transformer: &'a Option<Transformer<'a>>,
        exporter: &'a mut Option<Exporter<'a>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut record_handler = TransformAndExportRecordHandler::new(transformer, exporter);
        if let Err(e) = self.importer.read(&mut record_handler) {
            log::error!("Error while importing records: {}", e);
            return Err(e);
        }
        Ok(())
    }
}
