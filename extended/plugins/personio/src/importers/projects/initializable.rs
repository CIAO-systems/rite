use model::Initializable;

use crate::importers::configuration::GeneralConfiguration;

impl Initializable for super::Projects {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            self.general = GeneralConfiguration::load(&config)?;
        }
        Ok(())
    }
}
