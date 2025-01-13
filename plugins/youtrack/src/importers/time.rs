use import::Importer;
use model::Initializable;

use crate::importers::connection::YouTrackConnection;

pub struct YouTrackImporterTime {
    connection: Option<YouTrackConnection>,
}

impl YouTrackImporterTime {
    pub(crate) fn new() -> Self {
        Self { connection: None }
    }
}

impl Initializable for YouTrackImporterTime {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

impl Importer for YouTrackImporterTime {
    fn read(&mut self, callback: import::RecordCallback) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
