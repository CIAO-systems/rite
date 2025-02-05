use import::Importer;
use model::{BoxedError, Initializable};

pub struct TimeTypes {
    config: Option<model::xml::config::Configuration>,
}

impl TimeTypes {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for TimeTypes {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for TimeTypes {
    fn read(
        &mut self,
        _handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // FIXME implement me
        Ok(())
    }
}
